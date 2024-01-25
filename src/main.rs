extern crate config as exconfig;

use bdk::bitcoin::secp256k1::{All, Secp256k1, SecretKey};
use bdk::bitcoin::{secp256k1, Address, PublicKey};
use bdk::bitcoincore_rpc::Client as BitcoinClient;
use bdk::blockchain::RpcBlockchain as BitcoinWalletClient;
use bdk::database::MemoryDatabase;
use ethers::prelude::Http;
use ethers::providers::Provider as EthereumClient;
use ethers::signers::{LocalWallet as EthereumWallet, Signer};
use ethers::types::Address as EthereumAddress;
use eyre::{eyre, Context, Result};
use num::{bigint::Sign, BigInt, One, ToPrimitive, Zero};
use rand::rngs::ThreadRng;
use rapidsnark::{groth16_prover, groth16_verifier};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Div, Mul};
use std::path::PathBuf;
use std::str::FromStr;
use witness_calculator::WitnessCalculator;

mod config;
use crate::config::{CircomConfig, Config, WalletsConfig};

/// Bitcoin's  [`bdk::Wallet`] with hardcoded [`MemoryDatabase`].
pub type BitcoinWallet = bdk::Wallet<MemoryDatabase>;

pub struct SwapParticipant {
    name: String,

    atomic_swap_contract_address: EthereumAddress,
    circom: CircomConfig,
    bitcoin_client: BitcoinClient,
    ethereum_client: EthereumClient<Http>,

    bitcoin_wallet: BitcoinWallet,
    bitcoin_wallet_client: BitcoinWalletClient,
    ethereum_wallet: EthereumWallet,
}

impl SwapParticipant {
    pub fn from_config(
        name: String,
        config: &Config,
        wallets_config: &WalletsConfig,
        secp_ctx: &Secp256k1<All>,
    ) -> Result<Self> {
        let bitcoin_client = config
            .bitcoin_client()
            .wrap_err("failed to initialize Bitcoin RPC client")?;

        let ethereum_client = config
            .ethereum_client()
            .wrap_err("failed to initialize Ethereum RPC client")?;

        let (bitcoin_wallet, bitcoin_wallet_client) = config
            .bitcoin_wallet(secp_ctx, wallets_config.bitcoin_private_key)
            .wrap_err("failed to initialize Bitcoin wallet with its RPC client")?;
        let ethereum_wallet = Config::ethereum_wallet(wallets_config.ethereum_private_key.clone())
            .wrap_err("failed to intialize Ethereum wallet")?;

        println!("Initialized new participant with wallets: ");
        println!(
            "Bitcoin P2WPKH address: {}",
            Address::p2wpkh(
                &PublicKey::new(wallets_config.bitcoin_private_key.public_key(secp_ctx)),
                config.bitcoin_rpc.network
            )?
        );
        println!("Ethereum address: {}", ethereum_wallet.address());

        Ok(Self {
            name,
            atomic_swap_contract_address: config.atomic_swap_contract_address,
            circom: config.circom.clone(),
            bitcoin_client,
            ethereum_client,
            bitcoin_wallet,
            bitcoin_wallet_client,
            ethereum_wallet,
        })
    }

    pub fn new_atomic_swap(
        &self,
        rng: &mut ThreadRng,
        secp_ctx: &Secp256k1<All>,
    ) -> Result<(String, String)> {
        println!("\n{} starts atomic-swap", self.name);

        let swap_secret = SecretKey::new(rng);

        println!("Swap k secret: {}", swap_secret.display_secret());
        println!("Swap k public: {}", swap_secret.public_key(secp_ctx));

        let swap_secret_bigint = BigInt::from_bytes_be(Sign::Plus, &swap_secret.secret_bytes());
        let swap_secret_u64array = u256_to_u64array(swap_secret_bigint)
            .expect("Secret is always lseq than u256")
            .iter()
            .map(|val| BigInt::from(*val))
            .collect();

        let mut prover_inputs = HashMap::new();
        prover_inputs.insert("secret".to_string(), swap_secret_u64array);

        println!("Loading wasm...");

        let mut witness_calculator =
            WitnessCalculator::new(self.circom.witnes_calculator_path.clone())
                .wrap_err("failed to load witness calculator")?;

        println!("Calculating witness...");

        let witness = witness_calculator
            .calculate_witness(prover_inputs, true)
            .wrap_err("failed to calculate witness")?;

        println!("Loading proving keys...");

        let mut proving_key_file = File::open(self.circom.proving_key_path.clone())
            .wrap_err("failed to open proving key file")?;
        let mut proving_key = Vec::new();
        proving_key_file
            .read_to_end(&mut proving_key)
            .wrap_err("failed to read proving key file")?;

        println!("Calculating proof...");

        let proof = groth16_prover(&proving_key, &witness)
            .wrap_err("failed to generate atomic-swap zero-knowledge proof")?;

        Ok(proof)
    }

    pub fn accept_atomic_swap(&self, proof: String, public_inputs_json: String) -> Result<()> {
        println!("\n{} accepts atomic-swap", self.name);
        println!("Loading verification key...");

        let mut verification_key_file = File::open(self.circom.verification_key_path.clone())
            .wrap_err("failed to open verification key file")?;
        let mut verification_key = Vec::new();
        verification_key_file
            .read_to_end(&mut verification_key)
            .wrap_err("failed to read verification key file")?;

        println!("Verifying proof...");

        let is_proof_valid = groth16_verifier(
            &verification_key,
            proof.as_bytes(),
            public_inputs_json.as_bytes(),
        )
        .wrap_err("failed to verify proof")?;

        if !is_proof_valid {
            return Err(eyre!("invalid atomic-swap proof"));
        }

        let public_inputs: Vec<String> = serde_json::from_str(public_inputs_json.as_str())?;
        let swap_pubkey = parse_pubkey_from_pub_signals(public_inputs)?;
        println!("{}", swap_pubkey.to_string());

        Ok(())
    }
}

fn main() -> Result<()> {
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
            .wrap_err("failed to initialize Alice")?;

    let bob = SwapParticipant::from_config("Bob".to_string(), &cfg, &cfg.bob_config, &secp_ctx)
        .wrap_err("failed to initialize Bob")?;

    let (proof, public_inputs) = alice.new_atomic_swap(rng, &secp_ctx)?;

    bob.accept_atomic_swap(proof, public_inputs)?;

    Ok(())
}

fn parse_pubkey_from_pub_signals(pubsignals: Vec<String>) -> Result<secp256k1::PublicKey> {
    let key_x = parse_scalar_from_vec_str(pubsignals[0..4].to_vec())?;
    let key_y = parse_scalar_from_vec_str(pubsignals[4..8].to_vec())?;

    // Public key prefix 0x04
    let mut key_raw = vec![0x4];
    key_raw.append(&mut key_x.to_bytes_be().1.to_vec());
    key_raw.append(&mut key_y.to_bytes_be().1.to_vec());

    Ok(secp256k1::PublicKey::from_str(
        hex::encode(key_raw.as_slice()).as_str(),
    )?)
}

fn parse_scalar_from_vec_str(scalar_raw: Vec<String>) -> Result<BigInt> {
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
    use crate::{u256_to_u64array, u64array_to_u256};
    use num::BigInt;
    use std::str::FromStr;

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
