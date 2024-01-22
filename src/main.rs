extern crate config as exconfig;

use bdk::bitcoin::secp256k1::{All, Secp256k1, SecretKey};
use bdk::bitcoin::{Address, PublicKey};
use bdk::bitcoincore_rpc::Client as BitcoinClient;
use bdk::blockchain::RpcBlockchain as BitcoinWalletClient;
use bdk::database::MemoryDatabase;
use ethers::prelude::Http;
use ethers::providers::Provider as EthereumClient;
use ethers::signers::{LocalWallet as EthereumWallet, Signer};
use ethers::types::Address as EthereumAddress;
use eyre::{Context, Result};
use num::{bigint::Sign, BigInt, ToPrimitive, Zero};
use rand::rngs::ThreadRng;
use rapidsnark::{groth16_prover, groth16_verifier};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::ops::Div;
use std::path::PathBuf;
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

    pub fn new_atomic_swap(&self, rng: &mut ThreadRng) -> Result<(String, String)> {
        println!("\n{} starts atomic-swap", self.name);

        let swap_secret = SecretKey::new(rng);

        println!("Swap secret: {}", swap_secret.display_secret());

        let swap_secret_bigint = BigInt::from_bytes_be(Sign::Plus, &swap_secret.secret_bytes());
        let broken_swap_secret_bigint = u256_to_u64array(swap_secret_bigint)
            .expect("Secret is always lseq than u256")
            .iter()
            .map(|val| BigInt::from(*val))
            .collect();

        let mut prover_inputs = HashMap::new();
        prover_inputs.insert("secret".to_string(), broken_swap_secret_bigint);

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

    pub fn accept_atomic_swap(&self, proof: String, public_inputs: String) -> Result<bool> {
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
            public_inputs.as_bytes(),
        )
        .wrap_err("failed to verify proof")?;

        if !is_proof_valid {
            println!("Proof is invalid");
        }

        println!();
        Ok(is_proof_valid)
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

    let (proof, public_inputs) = alice.new_atomic_swap(rng)?;

    let is_proof_valid = bob.accept_atomic_swap(proof, public_inputs)?;
    if !is_proof_valid {
        println!("Atomic-swap failed");
    }

    Ok(())
}

fn u256_to_u64array(mut input: BigInt) -> Option<[u64; 4]> {
    let mut result = [0u64; 4];

    let u64_max = BigInt::from(u64::MAX);

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

#[cfg(test)]
mod test {
    use crate::u256_to_u64array;
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
                1015469868109144153,
                7042290186432306956,
                10826996139724932949,
                17982017980625340072,
            ],
        );
        do_test_u256_to_u64array(BigInt::from_str("1").unwrap(), vec![1, 0, 0, 0]);
        do_test_u256_to_u64array(BigInt::from_str("0").unwrap(), vec![0, 0, 0, 0]);
        do_test_u256_to_u64array(
            BigInt::from_str("9134136032198266807219851950679215").unwrap(),
            vec![5858704018890565205, 495162506494374, 0, 0],
        );
    }

    fn do_test_u256_to_u64array(input: BigInt, expect: Vec<u64>) {
        assert_eq!(expect.len(), 4);

        let result = u256_to_u64array(input).unwrap();

        assert_eq!(result[0], expect[0]);
        assert_eq!(result[1], expect[1]);
        assert_eq!(result[2], expect[2]);
        assert_eq!(result[3], expect[3]);
    }
}
