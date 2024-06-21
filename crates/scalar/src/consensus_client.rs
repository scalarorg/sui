// Copyright (c) Scalar Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use arc_swap::{ArcSwapOption, Guard};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use sui_config::NodeConfig;
use sui_core::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use sui_core::consensus_adapter::SubmitToConsensus;
use sui_types::digests::ChainIdentifier;
use sui_types::error::{SuiError, SuiResult};
use sui_types::messages_consensus::ConsensusTransaction;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::time::{sleep, timeout};
use tonic::transport::Channel;
use tracing::{debug, info, warn};

use crate::proto::{ConsensusApiClient, ExternalTransaction, GrpcConsensusOutput};

type TransactionData = Vec<u8>;
type TransactionClient = ConsensusApiClient<Channel>;
type TransactionSender = UnboundedSender<Vec<TransactionData>>;

/// Grpc client for submit transactions into scalaris consensus layer
#[derive(Default)]
pub struct GrpcConsensusClient {
    chain_id: ChainIdentifier,
    client: Arc<ArcSwapOption<TransactionClient>>,
    tx_sender: Arc<ArcSwapOption<TransactionSender>>,
}

impl GrpcConsensusClient {
    pub fn new(chain_id: ChainIdentifier) -> Self {
        Self {
            chain_id,
            client: Arc::new(ArcSwapOption::const_empty()),
            tx_sender: Arc::new(ArcSwapOption::const_empty()),
        }
    }
    pub async fn start(
        &self,
        _config: &NodeConfig,
    ) -> SuiResult<tonic::Response<tonic::codec::Streaming<GrpcConsensusOutput>>> {
        let consensus_address =
            std::env::var("CONSENSUS_ADDR").unwrap_or(String::from("http://127.0.0.1:8080"));
        info!(
            "Try to connect to consensus server at {:?}",
            consensus_address.as_str()
        );
        // We expect this to get called during the SUI process start. After that at least one
        // object will have initialised and won't need to call again.
        const START_TIMEOUT: Duration = Duration::from_secs(30);
        const RETRY_INTERVAL: Duration = Duration::from_millis(100);
        if let Ok(mut client) = timeout(START_TIMEOUT, async {
            loop {
                if let Ok(client) = ConsensusApiClient::connect(consensus_address.clone()).await {
                    return client;
                } else {
                    warn!(
                        "Cannot connect to the consensus server {:?}",
                        &consensus_address
                    );
                    sleep(RETRY_INTERVAL).await;
                }
            }
        })
        .await
        {
            debug!("Init transaction stream");
            let (tx_transaction, mut rx_transaction) = unbounded_channel();
            self.set_tx_sender(Arc::new(tx_transaction));
            let chain_id = self.chain_id.to_string();
            let stream = async_stream::stream! {
                while let Some(tx_bytes) = rx_transaction.recv().await {
                    //Received serialized data from a slice &[ConsensusTransaction]
                    info!("Receive a pending transaction hash, send it into scalaris consensus {:?}", &tx_bytes);
                    let consensus_transaction = ExternalTransaction { chain_id: chain_id.clone(), tx_bytes };
                    yield consensus_transaction;
                }
            };
            //pin_mut!(stream);
            let stream = Box::pin(stream);
            let response = client.init_transaction(stream).await?;
            self.set_client(Arc::new(client));
            return Ok(response);
        } else {
            return Err(SuiError::ExecutionError(format!(
                "Cannot connect to consensus server at {}",
                consensus_address,
            )));
        }
    }

    // async fn get(&self) -> Guard<Option<Arc<TransactionClient>>> {
    //     let client = self.client.load();
    //     if client.is_some() {
    //         return client;
    //     }

    //     // We expect this to get called during the SUI process start. After that at least one
    //     // object will have initialised and won't need to call again.
    //     const START_TIMEOUT: Duration = Duration::from_secs(30);
    //     const RETRY_INTERVAL: Duration = Duration::from_millis(100);
    //     if let Ok(client) = timeout(START_TIMEOUT, async {
    //         loop {
    //             let client = self.client.load();
    //             if client.is_some() {
    //                 return client;
    //             } else {
    //                 sleep(RETRY_INTERVAL).await;
    //             }
    //         }
    //     })
    //     .await
    //     {
    //         return client;
    //     }

    //     panic!(
    //         "Timed out after {:?} waiting for Consensus client connection!",
    //         START_TIMEOUT,
    //     );
    // }

    pub fn set_client(&self, client: Arc<TransactionClient>) {
        self.client.store(Some(client));
    }
    pub fn set_tx_sender(&self, sender: Arc<TransactionSender>) {
        self.tx_sender.store(Some(sender));
    }
    pub fn get_tx_sender(&self) -> Guard<Option<Arc<TransactionSender>>> {
        let sender = self.tx_sender.load();
        sender
    }
    pub fn clear(&self) {
        self.client.store(None);
    }
}

#[async_trait]
impl SubmitToConsensus for GrpcConsensusClient {
    async fn submit_to_consensus(
        &self,
        transactions: &[ConsensusTransaction],
        _epoch_store: &Arc<AuthorityPerEpochStore>,
    ) -> SuiResult {
        if let Some(sender) = self.get_tx_sender().as_ref() {
            let transactions_bytes = transactions
                .iter()
                .map(|t| bcs::to_bytes(t).expect("Serializing consensus transaction cannot fail"))
                .collect::<Vec<_>>();
            let _ = sender.send(transactions_bytes);
            // client
            //     .submit(transactions_bytes)
            //     .await
            //     .tap_err(|r| {
            //         // Will be logged by caller as well.
            //         warn!("Submit transactions failed with: {:?}", r);
            //     })
            //     .map_err(|err| SuiError::FailedToSubmitToConsensus(err.to_string()))?;
        } else {
            debug!("Consensus client is not initialized");
            return SuiResult::Err(SuiError::ExecutionError(
                "Consensus client is not initialized".to_string(),
            ));
        }
        Ok(())
    }
}
