use std::env::current_dir;
use std::sync::Arc;
use crate::blocks::blockchain::Blockchain;
use crate::transactions::{Transaction, UTXOSet};
use crate::utils::SledDb;
use crate::wallets::wallets::Wallets;

pub mod blocks;
pub mod utils;
pub mod error;
pub mod transactions;
pub mod wallets;

fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt().init();





}
