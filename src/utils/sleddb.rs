use std::{path::Path, collections::HashMap};
use sled::{Db, IVec, transaction::TransactionResult};

use crate::{ error::BlockchainError, utils::{deserialize, serialize}};
use crate::blocks::block::Block;
use crate::transactions::Txoutput;

pub const TIP_KEY: &str = "tip_hash";
pub const HEIGHT: &str = "height";
pub const TABLE_OF_BLOCK: &str = "blocks";
pub const UTXO_SET: &str = "utxos";

pub trait Storage: Send + Sync + 'static {
    fn get_tip(&self) -> Result<Option<String>, BlockchainError>;
    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockchainError>;
    fn get_height(&self) -> Result<Option<usize>, BlockchainError>;
    fn update_blocks(&self, key: &str, block: &Block, height: usize);
    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockchainError>;

    fn get_utxo_set(&self) -> HashMap<String, Vec<Txoutput>>;
    fn write_utxo(&self, txid: &str, outs: Vec<Txoutput>) -> Result<(), BlockchainError>;
    fn clear_utxo_set(&self);
}

pub struct StorageIterator<T> {
    data: T
}

impl<T> StorageIterator<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Iterator for StorageIterator<T>
    where
        T: Iterator,
        T::Item: Into<Block>
{
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}

pub struct SledDb {
    db: Db
}

impl SledDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            db: sled::open(path).unwrap()
        }
    }

    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }
}

impl Storage for SledDb {
    fn get_tip(&self) -> Result<Option<String>, BlockchainError> {
        let result = self.db.get(TIP_KEY)?.map(|v| deserialize::<String>(&v.to_vec()));
        result.map_or(Ok(None), |v| v.map(Some))
    }

    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockchainError> {
        let name = Self::get_full_key(TABLE_OF_BLOCK, key);
        let result = self.db.get(name)?.map(|v| v.into());
        Ok(result)
    }

    fn get_height(&self) -> Result<Option<usize>, BlockchainError> {
        let result = self.db.get(HEIGHT)?.map(|v| deserialize::<usize>(&v.to_vec()));
        result.map_or(Ok(None), |v| v.map(Some))
    }

    fn update_blocks(&self, key: &str, block: &Block, height: usize) {
        let _: TransactionResult<(), ()> = self.db.transaction(|db| {
            let name = Self::get_full_key(TABLE_OF_BLOCK, key);
            db.insert(name.as_str(), serialize(block).unwrap())?;
            db.insert(TIP_KEY, serialize(key).unwrap())?;
            db.insert(HEIGHT, serialize(&height).unwrap())?;
            db.flush();
            Ok(())
        });
    }

    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockchainError> {
        let prefix = format!("{}:", TABLE_OF_BLOCK);
        let iter = StorageIterator::new(self.db.scan_prefix(prefix));
        Ok(Box::new(iter))
    }

    fn get_utxo_set(&self) -> HashMap<String, Vec<Txoutput>> {
        let mut map = HashMap::new();

        let prefix = format!("{}:", UTXO_SET);

        for item in self.db.scan_prefix(prefix) {
            let (k, v) = item.unwrap();
            let txid = String::from_utf8(k.to_vec()).unwrap();
            let txid = txid.split(":").collect::<Vec<_>>()[1].into();
            let outputs = deserialize::<Vec<Txoutput>>(&v.to_vec()).unwrap();

            map.insert(txid, outputs);
        }

        map
    }

    fn write_utxo(&self, txid: &str, outs: Vec<Txoutput>) -> Result<(), BlockchainError> {
        let name = format!("{}:{}", UTXO_SET, txid);
        self.db.insert(name, serialize(&outs)?)?;
        Ok(())
    }

    fn clear_utxo_set(&self) {
        let prefix = format!("{}:", UTXO_SET);
        self.db.remove(prefix).unwrap();
    }
}

impl From<IVec> for Block {
    fn from(v: IVec) -> Self {
        let result = deserialize::<Block>(&v.to_vec());
        match result {
            Ok(block) => block,
            Err(_) => Block::default(),
        }
    }
}

impl From<Result<(IVec, IVec), sled::Error>> for Block {
    fn from(result: Result<(IVec, IVec), sled::Error>) -> Self {
        match result {
            Ok((_, v)) => match deserialize::<Block>(&v.to_vec()) {
                Ok(block) => block,
                Err(_) => Block::default(),
            },
            Err(_) => Block::default(),
        }
    }
}