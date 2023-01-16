use std::collections::HashMap;
use std::sync::Arc;
use crate::blocks::blockchain::Blockchain;
use crate::error::BlockchainError;
use crate::utils::sleddb::Storage;

pub struct UTXOSet<T> {
    storage: Arc<T>
}

impl<T: Storage> UTXOSet<T> {
    pub fn new(storage: Arc<T>) -> Self {
        Self {
            storage
        }
    }

    pub fn reindex(&self, bc: &Blockchain) -> Result<(), BlockchainError> {
        self.storage.clear_utxo_set();
        let map = bc.find_utxo();
        for (txid, outs) in map {

        }
    }

    pub fn find_sendable_outputs(
        &self,
        from_addr: &str,
        amount: i32) -> (i32, HashMap<String,Vec<i32> >) {

    }
}