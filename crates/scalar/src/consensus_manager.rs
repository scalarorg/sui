use crate::consensus_client::GrpcConsensusClient;
use crate::proto::GrpcConsensusOutput;
use futures::StreamExt;
use mysten_metrics::{spawn_monitored_task, RegistryService};
use std::sync::Arc;
use sui_config::NodeConfig;
use sui_core::{
    checkpoints::CheckpointService,
    consensus_handler::{scalar::ConsensusOutput, ConsensusHandler, ConsensusHandlerInitializer},
};
use tonic::Streaming;
use tracing::error;
pub struct ConsensusManager {
    client: Arc<GrpcConsensusClient>,
    //mysticeti_consensus_handler: Mutex<Option<MysticetiConsensusHandler>>,
}
impl ConsensusManager {
    pub fn new(registry_service: &RegistryService, client: Arc<GrpcConsensusClient>) -> Self {
        ConsensusManager {
            client,
            //mysticeti_consensus_handler: Mutex::new(None),
        }
    }
    pub(crate) async fn start(
        &self,
        config: &NodeConfig,
        consensus_handler_initializer: ConsensusHandlerInitializer,
    ) {
        if let Ok(response) = self.client.start(config).await {
            //Spawn thread for handle consensus ouput
            let consensus_handler = consensus_handler_initializer.new_consensus_handler();
            // let (commit_sender, commit_receiver) = unbounded_channel("mysticeti_consensus_output");
            // let mut mysticeti_consensus_handler = self.mysticeti_consensus_handler.lock().await;
            // *mysticeti_consensus_handler = Some(MysticetiConsensusHandler::new(
            //     consensus_handler_initializer.new_consensus_handler(),
            //     commit_receiver,
            // ));
            let resp_stream = response.into_inner();
            self.handle_consensus_output(resp_stream, consensus_handler)
                .await;
        }
    }
    async fn handle_consensus_output(
        &self,
        mut resp_stream: Streaming<GrpcConsensusOutput>,
        mut consensus_handler: ConsensusHandler<CheckpointService>,
    ) {
        let _handle = spawn_monitored_task!(async move {
            while let Some(received) = resp_stream.next().await {
                match received {
                    Ok(grpc_output) => {
                        let consensus_output: ConsensusOutput = grpc_output.into();
                        consensus_handler
                            .handle_scalaris_ouput(consensus_output)
                            .await;
                    }
                    Err(err) => error!("{:?}", err),
                }
                // match received
                //     .map_err(|err| anyhow!("{}", err))
                //     .and_then(|consensus_output| match consensus_output.protocol {

                //     })
                // {
                //     Ok(consensus_output) => {
                //         //info!("Received {:?} commited blocks.", blocks.len());
                //         consensus_handler
                //             .handle_consensus_output(consensus_output)
                //             .await;
                //     }
                //     Err(err) => {
                //         error!("{:?}", err);
                //     }
                // }
            }
        });
    }
    pub async fn shutdown(&self) {}
}
