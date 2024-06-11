mod service;
mod types;
pub use service::consensus_api_client::*;
use sui_core::consensus_handler::scalar::ConsensusOutput;
use types::{Block, ReputationScore};
pub use types::{ConsensusOutput as GrpcConsensusOutput, ExternalTransaction};

impl Into<ConsensusOutput> for GrpcConsensusOutput {
    fn into(self) -> ConsensusOutput {
        let GrpcConsensusOutput {
            leader_round,
            leader_author_index,
            commit_timestamp,
            commit_sub_dag_index,
            commit_digest,
            reputation_scores,
            blocks,
        } = self;
        let reputation_scores = reputation_scores
            .into_iter()
            .map(
                |ReputationScore {
                     authority_index,
                     score,
                 }| (authority_index, score),
            )
            .collect();
        let blocks = blocks
            .into_iter()
            .map(
                |Block {
                     authority_index,
                     transactions,
                 }| {
                    // let transactions = transactions
                    //     .into_iter()
                    //     .map(|tx| {
                    //         let ExternalTransaction { chain_id, tx_bytes } =
                    //             bcs::from_bytes(tx.as_slice()).expect("");
                    //         tx_bytes
                    //     })
                    //     .collect();
                    (authority_index, transactions)
                },
            )
            .collect();
        ConsensusOutput {
            reputation_scores: Some(reputation_scores),
            leader_round,
            leader_author_index,
            commit_timestamp,
            commit_sub_dag_index,
            commit_digest,
            blocks,
        }
    }
}
// impl TryInto<consensus_core::CommittedSubDag> for MysticetiOuput {
//     type Error = anyhow::Error;
//     fn try_into(self) -> Result<consensus_core::CommittedSubDag, Self::Error> {
//         let MysticetiOuput {
//             leader,
//             blocks,
//             timestamp_ms,
//             commit_ref,
//             reputation_scores_desc,
//         } = self;
//         let leader = bcs::from_bytes(leader.as_slice())?;
//         let blocks = blocks
//             .into_iter()
//             .map(|verified_blocks| verified_blocks.try_into()?)
//             .collect();
//         Ok(consensus_core::CommittedSubDag {
//             leader,
//             blocks: todo!(),
//             timestamp_ms,
//             commit_ref: todo!(),
//             reputation_scores_desc: todo!(),
//         })
//     }
// }

// impl TryInto<ConsensusOutput> for NarwhalOuput {
//     type Error = anyhow::Error;
//     fn try_into(self) -> Result<ConsensusOutput, Self::Error> {
//         let NarwhalOuput { sub_dag, batches } = self;
//         let sub_dag: narwhal_types::CommittedSubDag =
//             bcs::from_bytes(sub_dag.as_slice()).map_err(|err| anyhow!("{:?}", err))?;
//         let batches: Vec<Vec<Batch>> =
//             bcs::from_bytes(batches.as_slice()).map_err(|err| anyhow!("{:?}", err))?;
//         let consensus_output = ConsensusOutput {
//             sub_dag: Arc::new(sub_dag),
//             batches,
//         };
//         Ok(ConsensusOutput {
//             sub_dag: Arc::new(sub_dag),
//             batches,
//         })
//     }
// }
