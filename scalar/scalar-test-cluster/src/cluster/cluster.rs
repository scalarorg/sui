use crate::genesis_config::GenesisConfig;
use crate::network_config::NetworkConfig;
use crate::node::ScalarNodeHandle;
use crate::swarm::Swarm;
use crate::{Env, LocalClusterBuilder, LocalClusterConfig};
use async_trait::async_trait;
use fastcrypto::traits::KeyPair;
use jsonrpsee::{
    http_client::{HttpClient, HttpClientBuilder},
    ws_client::{WsClient, WsClientBuilder},
};
use move_binary_format::access::ModuleAccess;
use std::{net::SocketAddr, path::Path};
use sui_config::{Config, PersistedConfig, SUI_KEYSTORE_FILENAME, SUI_NETWORK_CONFIG};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::sui_client_config::{SuiClientConfig, SuiEnv};
use sui_sdk::{wallet_context::WalletContext, SuiClient, SuiClientBuilder};
use sui_types::base_types::SuiAddress;
use sui_types::crypto::{get_key_pair, AccountKeyPair, SuiKeyPair};
use tracing::{error, info};

const DEVNET_FAUCET_ADDR: &str = "https://faucet.devnet.sui.io:443";
const STAGING_FAUCET_ADDR: &str = "https://faucet.staging.sui.io:443";
const CONTINUOUS_FAUCET_ADDR: &str = "https://faucet.ci.sui.io:443";
const CONTINUOUS_NOMAD_FAUCET_ADDR: &str = "https://faucet.nomad.ci.sui.io:443";
const TESTNET_FAUCET_ADDR: &str = "https://faucet.testnet.sui.io:443";
const DEVNET_FULLNODE_ADDR: &str = "https://rpc.devnet.sui.io:443";
const STAGING_FULLNODE_ADDR: &str = "https://fullnode.staging.sui.io:443";
const CONTINUOUS_FULLNODE_ADDR: &str = "https://fullnode.ci.sui.io:443";
const CONTINUOUS_NOMAD_FULLNODE_ADDR: &str = "https://fullnode.nomad.ci.sui.io:443";
const TESTNET_FULLNODE_ADDR: &str = "https://fullnode.testnet.sui.io:443";

pub struct ClusterFactory;

impl ClusterFactory {
    pub async fn start(
        options: &LocalClusterConfig,
    ) -> Result<Box<dyn Cluster + Sync + Send>, anyhow::Error> {
        Ok(match &options.env {
            Env::NewLocal => Box::new(LocalNewCluster::start(options).await?),
            _ => Box::new(RemoteRunningCluster::start(options).await?),
        })
    }
}

/// Cluster Abstraction
#[async_trait]
pub trait Cluster {
    async fn start(options: &LocalClusterConfig) -> Result<Self, anyhow::Error>
    where
        Self: Sized;

    // fn fullnode_url(&self) -> &str;
    
    // fn indexer_url(&self) -> &Option<String>;

    // /// Returns faucet url in a remote cluster.
    // fn remote_faucet_url(&self) -> Option<&str>;

    // /// Returns faucet key in a local cluster.
    // fn local_faucet_key(&self) -> Option<&AccountKeyPair>;

    fn user_key(&self) -> AccountKeyPair;
    /// Place to put config for the wallet, and any locally running services.
    fn config_directory(&self) -> &Path;
}

/// Represents an up and running cluster deployed remotely.
pub struct RemoteRunningCluster {
    fullnode_url: String,
    faucet_url: String,
    config_directory: tempfile::TempDir,
}

#[async_trait]
impl Cluster for RemoteRunningCluster {
    async fn start(options: &LocalClusterConfig) -> Result<Self, anyhow::Error> {
        let (fullnode_url, faucet_url) = match options.env {
            Env::Devnet => (
                String::from(DEVNET_FULLNODE_ADDR),
                String::from(DEVNET_FAUCET_ADDR),
            ),
            Env::Staging => (
                String::from(STAGING_FULLNODE_ADDR),
                String::from(STAGING_FAUCET_ADDR),
            ),
            Env::Ci => (
                String::from(CONTINUOUS_FULLNODE_ADDR),
                String::from(CONTINUOUS_FAUCET_ADDR),
            ),
            Env::CiNomad => (
                String::from(CONTINUOUS_NOMAD_FULLNODE_ADDR),
                String::from(CONTINUOUS_NOMAD_FAUCET_ADDR),
            ),
            Env::Testnet => (
                String::from(TESTNET_FULLNODE_ADDR),
                String::from(TESTNET_FAUCET_ADDR),
            ),
            Env::CustomRemote => (
                options
                    .fullnode_address
                    .clone()
                    .expect("Expect 'fullnode_address' for Env::Custom"),
                options
                    .faucet_address
                    .clone()
                    .expect("Expect 'faucet_address' for Env::Custom"),
            ),
            Env::NewLocal => unreachable!("NewLocal shouldn't use RemoteRunningCluster"),
        };

        // TODO: test connectivity before proceeding?

        Ok(Self {
            fullnode_url,
            faucet_url,
            config_directory: tempfile::tempdir()?,
        })
    }

    // fn fullnode_url(&self) -> &str {
    //     &self.fullnode_url
    // }

    // fn indexer_url(&self) -> &Option<String> {
    //     &None
    // }

    // fn remote_faucet_url(&self) -> Option<&str> {
    //     Some(&self.faucet_url)
    // }

    // fn local_faucet_key(&self) -> Option<&AccountKeyPair> {
    //     None
    // }
    
    fn user_key(&self) -> AccountKeyPair {
        get_key_pair().1
    }

    fn config_directory(&self) -> &Path {
        self.config_directory.path()
    }
}

pub struct FullNodeHandle {
    pub sui_node: ScalarNodeHandle,
    pub sui_client: SuiClient,
    pub rpc_client: HttpClient,
    pub rpc_url: String,
    pub ws_url: String,
}

impl FullNodeHandle {
    pub async fn new(sui_node: ScalarNodeHandle, json_rpc_address: SocketAddr) -> Self {
        let rpc_url = format!("http://{}", json_rpc_address);
        let rpc_client = HttpClientBuilder::default().build(&rpc_url).unwrap();

        let ws_url = format!("ws://{}", json_rpc_address);
        let sui_client = SuiClientBuilder::default().build(&rpc_url).await.unwrap();

        Self {
            sui_node,
            sui_client,
            rpc_client,
            rpc_url,
            ws_url,
        }
    }

    pub async fn ws_client(&self) -> WsClient {
        WsClientBuilder::default()
            .build(&self.ws_url)
            .await
            .unwrap()
    }
}
pub struct TestCluster {
    pub swarm: Swarm,
    // pub wallet: WalletContext,
    // pub fullnode_handle: FullNodeHandle,
}
impl TestCluster {}
pub struct LocalNewCluster {
    test_cluster: TestCluster,
    // fullnode_url: String,
    // indexer_url: Option<String>,
    // faucet_key: AccountKeyPair,
    config_directory: tempfile::TempDir,
}

impl LocalNewCluster {
    #[allow(unused)]
    pub fn swarm(&self) -> &Swarm {
        &self.test_cluster.swarm
    }
}

#[async_trait]
impl Cluster for LocalNewCluster {
    async fn start(options: &LocalClusterConfig) -> Result<Self, anyhow::Error> {
        // TODO: options should contain port instead of address
        let fullnode_port = options.fullnode_address.as_ref().map(|addr| {
            addr.parse::<SocketAddr>()
                .expect("Unable to parse fullnode address")
                .port()
        });

        let indexer_address = options.indexer_address.as_ref().map(|addr| {
            addr.parse::<SocketAddr>()
                .expect("Unable to parse indexer address")
        });

        let mut cluster_builder = LocalClusterBuilder::new(); //.enable_fullnode_events();
        if let Some(size) = options.cluster_size.as_ref() {
            cluster_builder = cluster_builder.with_num_validators(size.clone());
        }
        cluster_builder = cluster_builder.with_consensus_grpc_port(options.consensus_grpc_port.clone());
        // Check if we already have a config directory that is passed
        if let Some(config_dir) = options.config_dir.clone() {
            assert!(options.epoch_duration_ms.is_none());
            // Load the config of the Sui authority.
            let network_config_path = config_dir.join(SUI_NETWORK_CONFIG);
            match PersistedConfig::read(&network_config_path)
                .map_err(|err| {
                    err.context(format!(
                        "Cannot open Scalar network config file at {:?}",
                        network_config_path
                    ))
                }) {
                Ok(network_config) => {
                    cluster_builder = cluster_builder.set_network_config(network_config);
                },
                Err(err) => {
                    error!("{:?}", err);
                    // Let the faucet account hold 1000 gas objects on genesis
                    let genesis_config = GenesisConfig::custom_genesis(1, 100);
                    // Custom genesis should be build here where we add the extra accounts
                    cluster_builder = cluster_builder.set_genesis_config(genesis_config);

                    if let Some(epoch_duration_ms) = options.epoch_duration_ms {
                        cluster_builder = cluster_builder.with_epoch_duration_ms(epoch_duration_ms);
                    }
                },      
            }
            cluster_builder = cluster_builder.with_config_dir(config_dir);
        } else {
            // Let the faucet account hold 1000 gas objects on genesis
            let genesis_config = GenesisConfig::custom_genesis(1, 100);
            // Custom genesis should be build here where we add the extra accounts
            cluster_builder = cluster_builder.set_genesis_config(genesis_config);

            if let Some(epoch_duration_ms) = options.epoch_duration_ms {
                cluster_builder = cluster_builder.with_epoch_duration_ms(epoch_duration_ms);
            }
        }

        // if let Some(rpc_port) = fullnode_port {
        //     cluster_builder = cluster_builder.with_fullnode_rpc_port(rpc_port);
        // }

        let mut test_cluster = cluster_builder.build().await;

        // Use the wealthy account for faucet
        let faucet_key = test_cluster.swarm.config_mut().account_keys.swap_remove(0);
        let faucet_address = SuiAddress::from(faucet_key.public());
        info!(?faucet_address, "faucet_address");

        // This cluster has fullnode handle, safe to unwrap
        // let fullnode_url = test_cluster.fullnode_handle.rpc_url.clone();
        //Khong chay indexer va graphql
        // if let (Some(pg_address), Some(indexer_address)) =
        //     (options.pg_address.clone(), indexer_address)
        // {
        //     if options.use_indexer_v2 {
        //         // Start in writer mode
        //         start_test_indexer_v2(
        //             Some(pg_address.clone()),
        //             fullnode_url.clone(),
        //             None,
        //             options.use_indexer_experimental_methods,
        //         )
        //         .await;

        //         // Start in reader mode
        //         start_test_indexer_v2(
        //             Some(pg_address),
        //             fullnode_url.clone(),
        //             Some(indexer_address.to_string()),
        //             options.use_indexer_experimental_methods,
        //         )
        //         .await;
        //     } else {
        //         let migrated_methods = if options.use_indexer_experimental_methods {
        //             IndexerConfig::all_implemented_methods()
        //         } else {
        //             vec![]
        //         };
        //         let config = IndexerConfig {
        //             db_url: Some(pg_address),
        //             rpc_client_url: fullnode_url.clone(),
        //             rpc_server_url: indexer_address.ip().to_string(),
        //             rpc_server_port: indexer_address.port(),
        //             migrated_methods,
        //             reset_db: true,
        //             ..Default::default()
        //         };
        //         start_test_indexer(config).await?;
        //     }
        // }

        // if let Some(graphql_address) = &options.graphql_address {
        //     let graphql_address = graphql_address.parse::<SocketAddr>()?;
        //     let graphql_connection_config = ConnectionConfig::new(
        //         Some(graphql_address.port()),
        //         Some(graphql_address.ip().to_string()),
        //         options.pg_address.clone(),
        //         None,
        //         None,
        //     );

        //     start_graphql_server(graphql_connection_config.clone()).await;
        // }

        // Let nodes connect to one another
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // TODO: test connectivity before proceeding?
        Ok(Self {
            test_cluster,
            config_directory: tempfile::tempdir()?,
            // fullnode_url,
            // faucet_key,
            // indexer_url: options.indexer_address.clone(),
        })
    }

    // fn fullnode_url(&self) -> &str {
    //     &self.fullnode_url
    // }

    // fn indexer_url(&self) -> &Option<String> {
    //     &self.indexer_url
    // }

    // fn remote_faucet_url(&self) -> Option<&str> {
    //     None
    // }

    // fn local_faucet_key(&self) -> Option<&AccountKeyPair> {
    //     Some(&self.faucet_key)
    // }

    fn user_key(&self) -> AccountKeyPair {
        get_key_pair().1
    }

    fn config_directory(&self) -> &Path {
        self.config_directory.path()
    }
}

// Make linter happy
#[async_trait]
impl Cluster for Box<dyn Cluster + Send + Sync> {
    async fn start(_options: &LocalClusterConfig) -> Result<Self, anyhow::Error> {
        unreachable!(
            "If we already have a boxed Cluster trait object we wouldn't have to call this function"
        );
    }
    // fn fullnode_url(&self) -> &str {
    //     (**self).fullnode_url()
    // }
    // fn indexer_url(&self) -> &Option<String> {
    //     (**self).indexer_url()
    // }

    // fn remote_faucet_url(&self) -> Option<&str> {
    //     (**self).remote_faucet_url()
    // }

    // fn local_faucet_key(&self) -> Option<&AccountKeyPair> {
    //     (**self).local_faucet_key()
    // }

    fn user_key(&self) -> AccountKeyPair {
        (**self).user_key()
    }

    fn config_directory(&self) -> &Path {
        (**self).config_directory()
    }
}

// pub async fn new_wallet_context_from_cluster(
//     cluster: &(dyn Cluster + Sync + Send),
//     key_pair: AccountKeyPair,
// ) -> WalletContext {
//     let config_dir = cluster.config_directory();
//     let wallet_config_path = config_dir.join("client.yaml");
//     let fullnode_url = cluster.fullnode_url();
//     info!("Use RPC: {}", &fullnode_url);
//     let keystore_path = config_dir.join(SUI_KEYSTORE_FILENAME);
//     let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
//     let address: SuiAddress = key_pair.public().into();
//     keystore.add_key(SuiKeyPair::Ed25519(key_pair)).unwrap();
//     SuiClientConfig {
//         keystore,
//         envs: vec![SuiEnv {
//             alias: "localnet".to_string(),
//             rpc: fullnode_url.into(),
//             ws: None,
//         }],
//         active_address: Some(address),
//         active_env: Some("localnet".to_string()),
//     }
//     .persisted(&wallet_config_path)
//     .save()
//     .unwrap();

//     info!(
//         "Initialize wallet from config path: {:?}",
//         wallet_config_path
//     );

//     WalletContext::new(&wallet_config_path, None, None)
//         .await
//         .unwrap_or_else(|e| {
//             panic!(
//                 "Failed to init wallet context from path {:?}, error: {e}",
//                 wallet_config_path
//             )
//         })
// }
