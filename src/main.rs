extern crate config as exconfig;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Div, Mul};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::{env, thread};

use bdk::bitcoin::hashes::hex::ToHex;
use bdk::bitcoin::secp256k1::{All, Secp256k1};
use bdk::bitcoin::{secp256k1, Address as BitcoinAddress, Txid as BitcoinTxid};
use bdk::blockchain::{Blockchain, RpcBlockchain as BitcoinClient};
use bdk::database::MemoryDatabase;
use bdk::descriptor::IntoWalletDescriptor;
use bdk::miniscript::descriptor::TapTree;
use bdk::miniscript::policy::Concrete;
use bdk::miniscript::Descriptor;
use bdk::wallet::AddressIndex;
use bdk::{bitcoin, SignOptions, SyncOptions, Wallet as BitcoinWallet, Wallet};
use ethers::prelude::{Http, SignerMiddleware};
use ethers::providers::{Middleware, Provider as EthereumClient};
use ethers::signers::{LocalWallet as EthereumWallet, Signer};
use ethers::types::U256;
use ethers::types::{Address as EthereumAddress, TxHash};
use ethers::utils::Units::Gwei;
use eyre::{eyre, Context, Result};
use num::{bigint::Sign, BigInt, BigUint, One, ToPrimitive, Zero};
use rand::rngs::ThreadRng;

use rapidsnark::{groth16_prover, groth16_verifier};
use witness_calculator::WitnessCalculator;

use crate::config::{CircomConfig, Config, SwapParams, WalletsConfig};
use crate::depositor_contract::Depositor as DepositorContract;

mod config;
mod depositor_contract;

/// Index of the pubkey's X last element in the Atomic-swap ZK proof public signals.
const PUBSIGNALS_PUBKEY_X_END: usize = 3;

/// Index of the pubkey's Y last element in the Atomic-swap ZK proof public signals.
const PUBSIGNALS_PUBKEY_Y_END: usize = 7;

/// Index of the secret hash last in the Atomic-swap ZK proof public signals.
const PUBSIGNALS_SECRET_HASH_INDEX: usize = 8;

/// Number of the BDK wallet's sync tries to find the taproot atomic-swap transaction on-chain that
/// has been published by a counterparty.
const MAX_NUMBER_OF_ATTEMPTS_TO_SYNC: usize = 100;

/// Delay between attempts to sync the BDK wallet to find the taproot atomic-swap transaction.
const DELAY_BETWEEN_SYNC_ATTEMPT_SEC: u64 = 5;

pub struct ParticipantKeys {
    pub bitcoin: secp256k1::KeyPair,
    pub ethereum: secp256k1::KeyPair,
}

impl ParticipantKeys {
    pub fn from_config(config: &WalletsConfig, secp_ctx: &Secp256k1<All>) -> Self {
        Self {
            bitcoin: secp256k1::KeyPair::from_secret_key(secp_ctx, &config.bitcoin_private_key),
            ethereum: secp256k1::KeyPair::from_secret_key(secp_ctx, &config.ethereum_private_key),
        }
    }
}

pub struct SwapParticipant {
    name: String,
    keys: ParticipantKeys,

    swap_params: SwapParams,

    atomic_swap_contract_address: EthereumAddress,
    circom: CircomConfig,
    bitcoin_client: BitcoinClient,
    ethereum_client: EthereumClient<Http>,

    bitcoin_wallet: BitcoinWallet<MemoryDatabase>,
    ethereum_wallet: EthereumWallet,
}

impl SwapParticipant {
    pub async fn from_config(
        name: String,
        config: &Config,
        wallets_config: &WalletsConfig,
        secp_ctx: &Secp256k1<All>,
    ) -> Result<Self> {
        let keys = ParticipantKeys::from_config(wallets_config, secp_ctx);

        let ethereum_client = config
            .ethereum_client()
            .wrap_err("failed to initialize Ethereum RPC client")?;

        let chain_id = ethereum_client.get_chainid().await?;

        let ethereum_wallet =
            EthereumWallet::from_bytes(&wallets_config.ethereum_private_key.secret_bytes())?
                .with_chain_id(chain_id.as_u64());

        let (bitcoin_wallet, bitcoin_client) = config
            .bitcoin_wallet(secp_ctx, wallets_config.bitcoin_private_key)
            .wrap_err("failed to initialize Bitcoin wallet with its RPC client")?;

        println!("Initialized new participant with wallets: ");
        println!(
            "Bitcoin P2WPKH address: {}",
            BitcoinAddress::p2wpkh(
                &bitcoin::PublicKey::new(wallets_config.bitcoin_private_key.public_key(secp_ctx)),
                config.bitcoin_rpc.network
            )?
        );
        println!("Ethereum address: {}", ethereum_wallet.address());

        Ok(Self {
            name,
            keys,
            swap_params: config.swap_params.clone(),
            atomic_swap_contract_address: config.atomic_swap_contract_address,
            circom: config.circom.clone(),
            bitcoin_client,
            ethereum_client,
            bitcoin_wallet,
            ethereum_wallet,
        })
    }

    pub fn bitcoin_public_key(&self) -> secp256k1::PublicKey {
        self.keys.bitcoin.public_key()
    }

    pub fn ethereum_address(&self) -> EthereumAddress {
        self.ethereum_wallet.address()
    }

    pub fn new_atomic_swap(
        &self,
        sats_to_swap: u64,
        counterparty_bitcoin_pubkey: secp256k1::PublicKey,
        rng: &mut ThreadRng,
        secp_ctx: &Secp256k1<All>,
    ) -> Result<(String, String)> {
        println!("\n{} starts atomic-swap", self.name);

        let swap_secret = secp256k1::SecretKey::new(rng);

        println!("| Swap k secret: {}", swap_secret.display_secret());
        println!("| Swap k public: {}", swap_secret.public_key(secp_ctx));

        println!("| Calculating zero-knowledge proof...");
        let (proof, pubsignals) = self
            .generate_swap_proof(swap_secret)
            .wrap_err("failed to generate atomic-swap proof")?;

        let swap_pubkey = swap_secret.public_key(secp_ctx);
        let escrow_pubkey = swap_pubkey
            .combine(&counterparty_bitcoin_pubkey)
            .expect("It's impossible to fail for 2 different public keys");

        let tx_id = self
            .send_atomic_swap_tx_to_bitcoin(sats_to_swap, escrow_pubkey, secp_ctx)
            .wrap_err("failed to send swap tx to Bitcoin")?;
        println!(
            "| Taprootized atomic-swap transaction has been sent to Bitcoin: {}",
            tx_id
        );

        Ok((proof, pubsignals))
    }

    pub async fn accept_atomic_swap(
        &self,
        proof: String,
        pubsignals_json: String,
        counterparty_bitcoin_pubkey: secp256k1::PublicKey,
        counterparty_ethereum_address: EthereumAddress,
    ) -> Result<()> {
        println!("\n{} accepts atomic-swap", self.name);

        println!("| Verifying zero-knowledge proof...");
        if !self.verify_swap_proof(proof, pubsignals_json.clone())? {
            return Err(eyre!("invalid atomic-swap proof"));
        }

        let public_inputs: Vec<String> = serde_json::from_str(pubsignals_json.as_str())?;
        let (swap_pubkey, swap_secret_hash) = parse_atomic_swap_proof_pubsignals(public_inputs)?;

        println!("| Swap secret's hash: {}", hex::encode(swap_secret_hash));

        let swap_transaction_found = self
            .check_atomic_swap_tx_appeared_on_bitcoin(swap_pubkey, counterparty_bitcoin_pubkey)
            .wrap_err("failed to check if atomic-swap transaction appeared in Bitcoin")?;

        if !swap_transaction_found {
            return Err(eyre!(
                "taproot atomic-swap transaction hasn't appeared; swap_pubkey: {swap_pubkey}"
            ));
        }

        let tx_id = self
            .send_atomic_swap_tx_to_ethereum(swap_secret_hash, counterparty_ethereum_address)
            .await?;

        println!(
            "| Atomic-swap transaction has been sent to Ethereum: {}",
            tx_id.to_hex()
        );

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    if env::args().len() != 2 {
        eprintln!(
            "Usage: {} <path-to-config-file>",
            env::args().next().unwrap()
        );
        std::process::exit(1);
    }
    let path_to_config = PathBuf::from(env::args().nth(1).unwrap());
    let cfg = exconfig::Config::builder()
        .add_source(exconfig::File::from(path_to_config))
        .build()?
        .try_deserialize()
        .wrap_err("failed to parse config")?;

    let secp_ctx = Secp256k1::new();
    let rng = &mut rand::thread_rng();

    let alice =
        SwapParticipant::from_config("Alice".to_string(), &cfg, &cfg.alice_config, &secp_ctx)
            .await
            .wrap_err("failed to initialize Alice")?;

    let bob = SwapParticipant::from_config("Bob".to_string(), &cfg, &cfg.bob_config, &secp_ctx)
        .await
        .wrap_err("failed to initialize Bob")?;

    let (proof, pubsignals) = alice.new_atomic_swap(
        cfg.swap_params.sats_to_swap,
        bob.bitcoin_public_key(),
        rng,
        &secp_ctx,
    )?;

    bob.accept_atomic_swap(
        proof,
        pubsignals,
        alice.bitcoin_public_key(),
        alice.ethereum_address(),
    )
    .await?;

    Ok(())
}

impl SwapParticipant {
    fn generate_swap_proof(&self, swap_secret: secp256k1::SecretKey) -> Result<(String, String)> {
        let swap_secret_bigint = BigInt::from_bytes_be(Sign::Plus, &swap_secret.secret_bytes());
        let swap_secret_u64array = u256_to_u64array(swap_secret_bigint)
            .expect("Secret is always lseq than u256")
            .iter()
            .map(|val| BigInt::from(*val))
            .collect();

        let mut prover_inputs = HashMap::new();
        prover_inputs.insert("secret".to_string(), swap_secret_u64array);

        let mut witness_calculator =
            WitnessCalculator::new(self.circom.witnes_calculator_path.clone())
                .wrap_err("failed to load witness calculator")?;

        // This process takes most of the time of the proof generation because of WASM. The C
        // binding can be used to speed it up.
        let witness = witness_calculator
            .calculate_witness(prover_inputs, true)
            .wrap_err("failed to calculate witness")?;

        let mut proving_key_file = File::open(self.circom.proving_key_path.clone())
            .wrap_err("failed to open proving key file")?;
        let mut proving_key = Vec::new();
        proving_key_file
            .read_to_end(&mut proving_key)
            .wrap_err("failed to read proving key file")?;

        let proof =
            groth16_prover(&proving_key, &witness).wrap_err("failed to generate groth16 proof")?;

        Ok(proof)
    }

    fn verify_swap_proof(&self, proof: String, pubsignals_json: String) -> Result<bool> {
        let mut verification_key_file = File::open(self.circom.verification_key_path.clone())
            .wrap_err("failed to open verification key file")?;
        let mut verification_key = Vec::new();
        verification_key_file
            .read_to_end(&mut verification_key)
            .wrap_err("failed to read verification key file")?;

        let is_proof_valid = groth16_verifier(
            &verification_key,
            proof.as_bytes(),
            pubsignals_json.as_bytes(),
        )
        .wrap_err("failed to verify proof")?;

        Ok(is_proof_valid)
    }

    async fn send_atomic_swap_tx_to_ethereum(
        &self,
        swap_secret_hash: [u8; 32],
        counterparty_ethereum_address: EthereumAddress,
    ) -> Result<TxHash> {
        let signer = Arc::new(SignerMiddleware::new(
            self.ethereum_client.clone(),
            self.ethereum_wallet.clone(),
        ));

        let contract = DepositorContract::new(self.atomic_swap_contract_address, signer);

        let wei_to_send = U256::from(self.swap_params.gwei_to_swap).mul(10u32.pow(Gwei.as_num()));
        let mut contract_call = contract.deposit(
            counterparty_ethereum_address,
            swap_secret_hash,
            U256::from(self.swap_params.ethereum_timelock_secs),
        );
        contract_call.tx.set_value(wei_to_send);
        let pending_tx = contract_call.send().await?;

        Ok(pending_tx.tx_hash())
    }

    fn send_atomic_swap_tx_to_bitcoin(
        &self,
        sats_to_swap: u64,
        escrow_pubkey: secp256k1::PublicKey,
        secp_ctx: &Secp256k1<All>,
    ) -> Result<BitcoinTxid> {
        let revocation_pubkey = self.keys.bitcoin.public_key();

        let taptree_policy_str = &format!(
            "and(older({}),pk({}))",
            self.swap_params.bitcoin_csv_delay, revocation_pubkey
        );
        let taptree_policy = Concrete::<String>::from_str(taptree_policy_str)?.compile()?;
        let taptree = TapTree::Leaf(Arc::new(taptree_policy));

        let taproot_descriptor = Descriptor::new_tr(escrow_pubkey.to_string(), Some(taptree))?
            .to_string()
            .into_wallet_descriptor(secp_ctx, self.bitcoin_wallet.network())?
            .0;

        // We need it to easy get the address from the descriptor
        let wallet = BitcoinWallet::new(
            taproot_descriptor.clone(),
            None,
            self.bitcoin_wallet.network(),
            MemoryDatabase::new(),
        )?;

        let taproot_address = wallet.get_address(AddressIndex::New)?.address;

        let tx_id = self
            .send_sats_to_specified_address(sats_to_swap, taproot_address.clone())
            .wrap_err(format!(
                "failed to send {} satoshis to {}",
                sats_to_swap, taproot_address
            ))?;

        Ok(tx_id)
    }

    fn send_sats_to_specified_address(
        &self,
        sats_amount: u64,
        address: BitcoinAddress,
    ) -> Result<BitcoinTxid> {
        self.bitcoin_wallet
            .sync(&self.bitcoin_client, SyncOptions::default())?;

        let (mut psbt, _details) = {
            let mut tx_builder = self.bitcoin_wallet.build_tx();
            tx_builder.add_recipient(address.script_pubkey(), sats_amount);
            tx_builder.finish()?
        };

        let is_finalized = self
            .bitcoin_wallet
            .sign(&mut psbt, SignOptions::default())?;

        if !is_finalized {
            return Err(eyre!("failed to sign and finalize a transaction"));
        }

        let txid = psbt.unsigned_tx.txid();

        self.bitcoin_client.broadcast(&psbt.extract_tx())?;

        Ok(txid)
    }

    fn check_atomic_swap_tx_appeared_on_bitcoin(
        &self,
        swap_pubkey: secp256k1::PublicKey,
        revocation_pubkey_raw: secp256k1::PublicKey,
    ) -> Result<bool> {
        let escrow_pubkey = bitcoin::PublicKey::new(
            swap_pubkey
                .combine(&self.bitcoin_public_key())
                .expect("It's impossible to fail for 2 different public keys"),
        );
        let revocation_pubkey = bitcoin::PublicKey::new(revocation_pubkey_raw);

        let taproot_descriptor = bdk::descriptor!(tr(
            escrow_pubkey,
            and_v(v:pk(revocation_pubkey), older(self.swap_params.bitcoin_csv_delay))
        ))?;

        let wallet = Wallet::new(
            taproot_descriptor,
            None,
            self.bitcoin_wallet.network(),
            MemoryDatabase::new(),
        )?;

        let mut unspent_utxos;
        for _ in 0..=MAX_NUMBER_OF_ATTEMPTS_TO_SYNC {
            wallet
                .sync(&self.bitcoin_client, SyncOptions::default())
                .wrap_err("failed to sync a BDK wallet")?;

            unspent_utxos = wallet
                .list_unspent()
                .wrap_err("failed to retrieve unspent UTXOs from BDK wallet")?;

            if !unspent_utxos.is_empty() {
                // The wallet has only a taproot descriptor, so it is our transaction.
                return Ok(true);
            }

            thread::sleep(Duration::from_secs(DELAY_BETWEEN_SYNC_ATTEMPT_SEC))
        }

        Ok(false)
    }
}

fn parse_atomic_swap_proof_pubsignals(
    pubsignals: Vec<String>,
) -> Result<(secp256k1::PublicKey, [u8; 32])> {
    let pubkey = parse_pubkey_from_str_vec(pubsignals.clone())
        .wrap_err("failed to parse pubkey from pubsignals")?;

    let poseidon_hash =
        parse_poseidon_hash_from_str(pubsignals[PUBSIGNALS_SECRET_HASH_INDEX].clone())
            .wrap_err("failed to parse poseidon hash from pubsignals")?;

    Ok((pubkey, poseidon_hash))
}

fn parse_poseidon_hash_from_str(hash_str: String) -> Result<[u8; 32]> {
    let hash = BigUint::from_str(hash_str.as_str())
        .wrap_err("failed to parse BigUint from string")?
        .to_bytes_be()
        .as_slice()
        .try_into()?;

    Ok(hash)
}

fn parse_pubkey_from_str_vec(pubsignals: Vec<String>) -> Result<secp256k1::PublicKey> {
    let key_x = parse_scalar_from_str_slice(pubsignals[0..=PUBSIGNALS_PUBKEY_X_END].to_vec())?;
    let key_y = parse_scalar_from_str_slice(
        pubsignals[PUBSIGNALS_PUBKEY_X_END + 1..=PUBSIGNALS_PUBKEY_Y_END].to_vec(),
    )?;

    // Public key prefix 0x04
    let mut key_raw = vec![0x4];
    key_raw.append(&mut key_x.to_bytes_be().1.to_vec());
    key_raw.append(&mut key_y.to_bytes_be().1.to_vec());

    Ok(secp256k1::PublicKey::from_str(
        hex::encode(key_raw.as_slice()).as_str(),
    )?)
}

fn parse_scalar_from_str_slice(scalar_raw: Vec<String>) -> Result<BigInt> {
    if scalar_raw.len() != 4 {
        return Err(eyre!("invalid number of scalar parts to parse"));
    }

    let mut scalar_u64_array = [0u64; 4];
    for i in 0..4 {
        scalar_u64_array[i] = u64::from_str(scalar_raw[i].as_str())?
    }

    Ok(u64array_to_u256(scalar_u64_array))
}

fn u256_to_u64array(mut input: BigInt) -> Option<[u64; 4]> {
    let mut result = [0u64; 4];

    let u64_max = BigInt::from(u64::MAX) + BigInt::one();

    for x in result.iter_mut() {
        let rem = input.clone() % u64_max.clone();
        *x = rem.to_u64().expect("mod of u64 can't be gr than u64");
        input = input.div(u64_max.clone())
    }

    if input != BigInt::zero() {
        return None;
    }

    Some(result)
}

fn u64array_to_u256(input: [u64; 4]) -> BigInt {
    let mut result = BigInt::from(input[3]);

    let u64_max = BigInt::from(u64::MAX) + BigInt::one();

    for i in (0..=2).rev() {
        result = result.mul(u64_max.clone());
        result = result.add(BigInt::from(input[i]));
    }

    result
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use num::BigInt;

    use crate::{u256_to_u64array, u64array_to_u256};

    #[test]
    fn test_u256_to_u64array() {
        do_test_u256_to_u64array(
            BigInt::from_str(
                "112874956271937818984300676023995443620017137826812392247603206681821520986618",
            )
            .unwrap(),
            vec![
                5264901914485981690,
                2440863701439358041,
                12221174418977567583,
                17982017980625340069,
            ],
        );
        do_test_u256_to_u64array(BigInt::from_str("1").unwrap(), vec![1, 0, 0, 0]);
        do_test_u256_to_u64array(BigInt::from_str("0").unwrap(), vec![0, 0, 0, 0]);
        do_test_u256_to_u64array(
            BigInt::from_str("9134136032198266807219851950679215").unwrap(),
            vec![5858208856384070831, 495162506494374, 0, 0],
        );
    }

    fn do_test_u256_to_u64array(expected_u256: BigInt, expected_u64array: Vec<u64>) {
        assert_eq!(expected_u64array.len(), 4);

        let u64array = u256_to_u64array(expected_u256.clone()).unwrap();

        assert_eq!(u64array[0], expected_u64array[0]);
        assert_eq!(u64array[1], expected_u64array[1]);
        assert_eq!(u64array[2], expected_u64array[2]);
        assert_eq!(u64array[3], expected_u64array[3]);

        let u256 = u64array_to_u256(u64array);

        assert_eq!(u256, expected_u256);
    }
}
