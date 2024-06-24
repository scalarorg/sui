use std::{
    fmt::{self, Display, Formatter},
    io::Write,
};

use consensus_core::CommitDigest;
use fastcrypto::hash::HashFunction;
use serde::{Deserialize, Serialize};
use sui_types::{crypto::DefaultHash, digests::ConsensusCommitDigest, messages_consensus::ConsensusTransaction};
use tracing::{info, debug};

use crate::{
    checkpoints::CheckpointServiceNotify,
    consensus_handler::ConsensusHandler,
    consensus_types::{consensus_output_api::ConsensusOutputAPI, AuthorityIndex},
};
type ConsensusOutputTransactions<'a> = Vec<(AuthorityIndex, Vec<(&'a [u8], ConsensusTransaction)>)>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsensusOutput {
    pub reputation_scores: Option<Vec<(AuthorityIndex, u64)>>,
    pub leader_round: u64,
    pub leader_author_index: AuthorityIndex,
    pub commit_timestamp: u64,
    pub commit_sub_dag_index: u64,
    pub commit_digest: Vec<u8>,
    pub blocks: Vec<(AuthorityIndex, Vec<Vec<u8>>)>,
}

impl ConsensusOutputAPI for ConsensusOutput {
    fn reputation_score_sorted_desc(&self) -> Option<Vec<(AuthorityIndex, u64)>> {
        self.reputation_scores.clone()
    }

    fn leader_round(&self) -> u64 {
        self.leader_round
    }

    fn leader_author_index(&self) -> AuthorityIndex {
        self.leader_author_index
    }

    fn commit_timestamp_ms(&self) -> u64 {
        self.commit_timestamp
    }

    fn commit_sub_dag_index(&self) -> u64 {
        self.commit_sub_dag_index
    }

    fn transactions(&self) -> ConsensusOutputTransactions<'_> {
        self.blocks
            .iter()
            .map(|(cert, batches)| {
                let transactions: Vec<(&[u8], ConsensusTransaction)> =
                    batches.iter().map(|serialized_transaction| {
                        let transaction = match bcs::from_bytes::<ConsensusTransaction>(
                            serialized_transaction.as_slice(),
                        ) {
                            Ok(transaction) => transaction,
                            Err(err) => {
                                // This should have been prevented by Narwhal batch verification.
                                info!("Error while deserializing transaction into ConsensusTransaction");
                                panic!(
                                    "Unexpected malformed transaction (failed to deserialize): {}\n Transaction={:?}",
                                    err, serialized_transaction
                                );
                            }
                        };
                        (serialized_transaction.as_slice(), transaction)
                        
                    }).collect();
                (cert.clone(), transactions)
            }).collect()
    }

    fn consensus_digest(
        &self,
        protocol_config: &sui_protocol_config::ProtocolConfig,
    ) -> sui_types::digests::ConsensusCommitDigest {
        if protocol_config.mysticeti_use_committed_subdag_digest() {
            // We port CommitDigest, a consensus space object, into ConsensusCommitDigest, a sui-core space object.
            // We assume they always have the same format.
            static_assertions::assert_eq_size!(ConsensusCommitDigest, CommitDigest);
            if let Ok(digest) = TryInto::<[u8;32]>::try_into(self.commit_digest.as_slice()) {
                return ConsensusCommitDigest::from(digest);
            } 
        }
        ConsensusCommitDigest::default()
    }
}

impl Display for ConsensusOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConsensusOuput(leader_index={}, blocks=[",
            self.leader_author_index,
        )?;
        for (idx, block) in self.blocks.iter().enumerate() {
            if idx > 0 {
                write!(f, ", ")?;
            }
            let mut hasher = DefaultHash::default();
            block.1.iter().for_each(|tx| {
                let _ = hasher.write(tx.as_slice());
            });
            write!(f, "{}", hasher.finalize())?;
        }
        write!(f, "])")
    }
}

impl<C: CheckpointServiceNotify + Send + Sync> ConsensusHandler<C> {
    pub async fn handle_scalaris_output(&mut self, consensus_output: ConsensusOutput) {
        debug!("Handle scalaris ouput");
        self.handle_consensus_output_internal(consensus_output)
            .await;
    }
}
