/*
 * 2023-12-13 Taivv
 * Defined transaction with namespace for different system
 */

use narwhal_types::Transaction;
use serde::{Serialize, Deserialize};
pub const NAMESPACE_RETH: &str = "reth";
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NsTransaction {
    pub namespace: String,
    #[serde(with = "serde_bytes")]
    pub transaction: Transaction
}
impl NsTransaction {
    pub fn new(namespace: String, transaction: Transaction) -> NsTransaction {
        Self {
            namespace,
            transaction,
        }
    }
    pub fn new_reth_transaction(transaction: Transaction) -> NsTransaction {
        Self {
            namespace: NAMESPACE_RETH.to_owned(),
            transaction,
        }
    }
}