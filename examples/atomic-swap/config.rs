use bdk::bitcoin::secp256k1::{All, Secp256k1, SecretKey as SecpSecretKey};
use bdk::bitcoin::{Network as BitcoinNetwork, PrivateKey as BitcoinPrivateKey};
use bdk::bitcoincore_rpc::Client as BitcoinClient;
use bdk::blockchain::{
    rpc::Auth as BdkRpcAuth, ConfigurableBlockchain, RpcBlockchain as BitcoinWalletClient,
    RpcConfig as BdkRpcConfig,
};
use bdk::database::MemoryDatabase;
use bdk::wallet::wallet_name_from_descriptor;
use bdk::{descriptor, SyncOptions};
use ethers::providers::{Http, Provider as EthereumClient};
use ethers::signers::LocalWallet as EthereumWallet;
use ethers::types::Address as EthereumAddress;
use eyre::{Context, Result};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

/// Bitcoin's  [`bdk::Wallet`] with hardcoded [`MemoryDatabase`].
pub type BitcoinWallet = bdk::Wallet<MemoryDatabase>;

/// Contains the RAW config data.
///
/// Use [`Config::<ConfigRaw>from`] to convert it to the 'high-level' [`Config`].
#[derive(serde::Deserialize)]
struct ConfirRaw {
    pub atomic_swap_contract_address: EthereumAddress,
    pub ethereum_rpc_url: String,
    pub bitcoin_rpc: BitcoinRpcConfig,

    #[serde(rename = "alice")]
    pub alice_config: WalletsConfigRaw,
    #[serde(rename = "bob")]
    pub bob_config: WalletsConfigRaw,
}

impl ConfirRaw {
    /// Returns the [`ethers::signers::LocalWallet`] that can be used to sign transactions for the
    /// Ethereum network.
    pub fn ethereum_wallet(secret_key: String) -> Result<EthereumWallet> {
        let wallet = EthereumWallet::from_str(secret_key.as_str())?;

        Ok(wallet)
    }

    /// Return the [`BitcoinWallet`] that can be used to operate with UTXOs and the
    /// [`BitcoinWalletClient`] for retrieving the available UTXOs from the Bitcoin network.
    pub fn bitcoin_wallet(
        &self,
        secp_ctx: &Secp256k1<All>,
        secret_key: SecpSecretKey,
    ) -> Result<(BitcoinWallet, BitcoinWalletClient)> {
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
            .bitcoin_client_for_wallet(secp_ctx, secret_key)
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

    /// Returns the [`bdk::bitcoincore_rpc::Client`] that can be used to send transactions to
    /// the Bitcoin network.
    pub fn bitcoin_client(&self) -> Result<BitcoinClient> {
        let client =
            BitcoinClient::new(&self.bitcoin_rpc.url, self.bitcoin_rpc.auth.clone().into())?;

        Ok(client)
    }

    /// Returns the [`bdk::blockchain::RpcBlockchain`] for the wallet. It will be used there to
    /// retrieve the UTXOs from Bitcoin.
    fn bitcoin_client_for_wallet(
        &self,
        secp_ctx: &Secp256k1<All>,
        secret_key: SecpSecretKey,
    ) -> Result<BitcoinWalletClient> {
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

        let blockchain = BitcoinWalletClient::from_config(&config)?;

        Ok(blockchain)
    }
}

#[derive(serde::Deserialize)]
pub struct WalletsConfigRaw {
    pub bitcoin_private_key: SecpSecretKey,
    pub ethereum_private_key: String,
}

#[derive(serde::Deserialize)]
pub struct BitcoinRpcConfig {
    pub url: String,
    pub auth: BdkRpcAuth,
    pub network: BitcoinNetwork,
}

pub struct WalletConfig {
    pub bitcoin_wallet: BitcoinWallet,
    pub bitcoin_wallet_client: BitcoinWalletClient,
    pub ethereum_wallet: EthereumWallet,
}

/// Contains the config data that is needed to run the example.
///
/// Use [`Config::try_from`] to load the config from a file.
pub struct Config {
    pub atomic_swap_contract_address: EthereumAddress,
    pub bitcoin_client: BitcoinClient,
    pub ethereum_client: EthereumClient<Http>,

    pub alice: WalletConfig,
    pub bob: WalletConfig,
}

impl TryFrom<ConfirRaw> for Config {
    type Error = eyre::Error;

    fn try_from(raw: ConfirRaw) -> Result<Self> {
        let secp_ctx = Secp256k1::new();

        let bitcoin_client = raw
            .bitcoin_client()
            .wrap_err("failed to initialize Bitcoin RPC client")?;
        let ethereum_client = raw
            .ethereum_client()
            .wrap_err("failed to initialize Ethereum RPC client")?;

        let (alice_btc_wallet, alice_bitcoin_client) = raw
            .bitcoin_wallet(&secp_ctx, raw.alice_config.bitcoin_private_key)
            .wrap_err("failed to initialize Alice's Bitcoin wallet")?;
        let alice_eth_wallet =
            ConfirRaw::ethereum_wallet(raw.alice_config.ethereum_private_key.clone())
                .wrap_err("failed to intialize Alice's Ethereum wallet")?;

        let (bob_wallet, bob_bitcoin_client) = raw
            .bitcoin_wallet(&secp_ctx, raw.bob_config.bitcoin_private_key)
            .wrap_err("failed to initialize Bob's Bitcoin wallet")?;
        let bob_eth_wallet =
            ConfirRaw::ethereum_wallet(raw.bob_config.ethereum_private_key.clone())
                .wrap_err("failed to initialize Bob's Ethereum wallet")?;

        Ok(Self {
            atomic_swap_contract_address: raw.atomic_swap_contract_address,
            bitcoin_client,
            ethereum_client,
            alice: WalletConfig {
                bitcoin_wallet: alice_btc_wallet,
                bitcoin_wallet_client: alice_bitcoin_client,
                ethereum_wallet: alice_eth_wallet,
            },
            bob: WalletConfig {
                bitcoin_wallet: bob_wallet,
                bitcoin_wallet_client: bob_bitcoin_client,
                ethereum_wallet: bob_eth_wallet,
            },
        })
    }
}

impl TryFrom<PathBuf> for Config {
    type Error = eyre::Error;

    /// Load the [`Config`] from a file by specified `path`.
    fn try_from(path: PathBuf) -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::from(path))
            .build()?;

        config
            .try_deserialize::<ConfirRaw>()
            .wrap_err("failed to deserialize config")?
            .try_into()
            .wrap_err("failed to convert config")
    }
}
