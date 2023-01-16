use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::mem::swap;
use tracing::info;
use crate::error::BlockchainError;
use crate::utils::{deserialize, serialize};
use crate::wallets::wallet::Wallet;

pub const WALLET_FILE: &str = "wallet.dat";

#[derive(Serialize, Deserialize)]
pub struct Wallets {
    wallets: HashMap<String, Wallet>
}

impl Wallets {
    pub fn new() -> Result<Self, BlockchainError> {
        let wallets = Self::load_wallet_from_file();
        wallets
    }

    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.save_wallet_file().unwrap();
        address
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.get(address)
    }
    pub fn get_address(&self) -> Vec<&String> {
        self.wallets.keys().collect()
    }

    pub fn save_wallet_file(&self) -> Result<(), BlockchainError> {
        let path = current_dir().unwrap().join(WALLET_FILE);
        let wallets_ser = serialize(&self)?;
        fs::write(path, &wallets_ser).unwrap();
        Ok(())
    }

    pub fn load_wallet_from_file() -> Result<Self, BlockchainError> {
        let path = current_dir().unwrap().join(WALLET_FILE);
        info!("Wallet path: {:?}", path);

        if !path.exists() {
            let wallets = Wallets {
                wallets: HashMap::new()
            };
            return Ok(wallets)
        }
        let wallets_ser = fs::read(&path).unwrap();
        let wallets = deserialize(&wallets_ser);
        wallets
    }

}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::blocks::blockchain::Blockchain;
    use crate::transactions::{Transaction, UTXOSet};
    use crate::utils::SledDb;
    use super::*;

    #[test]
    fn testwallet() {
        tracing_subscriber::fmt().init();

        let mut wallets = Wallets::new().unwrap();
        let genesis_addr = wallets.create_wallet();
        println!("==> genesis address: {}", genesis_addr);

        let path = current_dir().unwrap().join("data");
        let storage = Arc::new(SledDb::new(path));

        let bc = Blockchain::new(storage.clone(), &genesis_addr);
        let utxos = UTXOSet::new(storage);
        utxos.reindex(&bc).unwrap();

        bc.blocks_info();
    }

}
