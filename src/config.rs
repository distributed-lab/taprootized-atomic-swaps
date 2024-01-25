use bdk::bitcoin::secp256k1::{All, Secp256k1, SecretKey as SecpSecretKey};
use bdk::bitcoin::{Network as BitcoinNetwork, PrivateKey as BitcoinPrivateKey};
use bdk::blockchain::{
    rpc::Auth as BdkRpcAuth, ConfigurableBlockchain, RpcBlockchain as BitcoinClient,
    RpcConfig as BdkRpcConfig,
};
use bdk::database::MemoryDatabase;
use bdk::wallet::wallet_name_from_descriptor;
use bdk::{descriptor, SyncOptions};
use ethers::providers::{Http, Provider as EthereumClient};
use ethers::types::Address as EthereumAddress;
use eyre::{Context, Result};
use std::path::PathBuf;
use std::time::Duration;

/// Contains the RAW config data.
///
/// Use [`Config::<ConfigRaw>from`] to convert it to the 'high-level' [`Config`].
#[derive(serde::Deserialize)]
pub struct Config {
    pub atomic_swap_contract_address: EthereumAddress,
    pub ethereum_rpc_url: String,
    pub bitcoin_rpc: BitcoinRpcConfig,
    pub circom: CircomConfig,

    pub swap_params: SwapParams,

    #[serde(rename = "alice")]
    pub alice_config: WalletsConfig,
    #[serde(rename = "bob")]
    pub bob_config: WalletsConfig,
}

impl Config {
    /// Return the [`BitcoinWallet`] that can be used to operate with UTXOs and the
    /// [`BitcoinWalletClient`] for retrieving the available UTXOs from the Bitcoin network.
    pub fn bitcoin_wallet(
        &self,
        secp_ctx: &Secp256k1<All>,
        secret_key: SecpSecretKey,
    ) -> Result<(bdk::Wallet<MemoryDatabase>, BitcoinClient)> {
        let network = self.bitcoin_rpc.network;
        let private_key = BitcoinPrivateKey::new(secret_key, network);

        let wallet = bdk::Wallet::new(
            descriptor!(wpkh(private_key))?,
            None,
            network,
            MemoryDatabase::default(),
        )
        .wrap_err("failed to initialize BDK wallet")?;

        let bitcoin_client = self
            .bitcoin_client(secp_ctx, secret_key)
            .wrap_err("failed to initialize Bitcoin RPC client for wallet")?;

        wallet
            .sync(&bitcoin_client, SyncOptions::default())
            .wrap_err("failed to sync wallet")?;

        Ok((wallet, bitcoin_client))
    }

    /// Returns the [`ethers::providers::Provider`] that can be used to send transactions to
    /// the Ethereum network.
    pub fn ethereum_client(&self) -> Result<EthereumClient<Http>> {
        let provider = EthereumClient::<Http>::try_from(self.ethereum_rpc_url.clone())?
            .interval(Duration::from_millis(10u64));

        Ok(provider)
    }

    /// Returns the [`bdk::blockchain::RpcBlockchain`] for the wallet. It will be used there to
    /// retrieve the UTXOs from Bitcoin.
    fn bitcoin_client(
        &self,
        secp_ctx: &Secp256k1<All>,
        secret_key: SecpSecretKey,
    ) -> Result<BitcoinClient> {
        let network = self.bitcoin_rpc.network;
        let private_key = BitcoinPrivateKey::new(secret_key, network);
        let wallet_name =
            wallet_name_from_descriptor(descriptor!(wpkh(private_key))?, None, network, secp_ctx)?;

        let config = BdkRpcConfig {
            url: self.bitcoin_rpc.url.clone(),
            auth: self.bitcoin_rpc.auth.clone(),
            network,
            wallet_name,
            sync_params: None,
        };

        let blockchain = BitcoinClient::from_config(&config)?;

        Ok(blockchain)
    }
}

#[derive(Clone, serde::Deserialize)]
pub struct SwapParams {
    pub sats_to_swap: u64,
    pub gwei_to_swap: u64,
    pub bitcoin_csv_delay: u32,
    pub ethereum_timelock_secs: u64,
}

#[derive(serde::Deserialize)]
pub struct WalletsConfig {
    pub bitcoin_private_key: SecpSecretKey,
    pub ethereum_private_key: SecpSecretKey,
}

#[derive(serde::Deserialize)]
pub struct BitcoinRpcConfig {
    pub url: String,
    pub auth: BdkRpcAuth,
    pub network: BitcoinNetwork,
}

#[derive(Clone, serde::Deserialize)]
pub struct CircomConfig {
    pub witnes_calculator_path: PathBuf,
    pub proving_key_path: PathBuf,
    pub verification_key_path: PathBuf,
}
