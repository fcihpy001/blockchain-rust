use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::mem::swap;
use tracing::info;
use crate::error::BlockchainError;
use crate::utils::{deserialize, serialize};
use crate::wallets::wallet::Wallet;

pub const WALLET_FILE: &str = "wallet.dat";

pub struct Walltes {
    wallets: HashMap<String, Wallet>
}

impl Walltes {
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

    pub fn get_wallet(&self, address: &str) -> Option<&Wallte> {
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
            let wallets = Walltes {
                wallets: HashMap::new()
            };
            return Ok(wallets)
        }
        let wallets_ser = fs::read(&path).unwrap();
        let wallets = deserialize(&wallets_ser);
        wallets
    }

}
