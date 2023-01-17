use std::env;
use std::env::current_dir;
use std::sync::Arc;
use anyhow::Result;

use crate::networks::node::Node;
use crate::utils::SledDb;

pub mod blocks;
pub mod utils;
pub mod error;
pub mod transactions;
pub mod wallets;
pub mod networks;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let mut path = String::from("data");
    if let Some(args) = env::args().nth(2) {
        path = args;
    }

    let path = current_dir().unwrap().join(path);
    let db = Arc::new(SledDb::new(path));
    let mut node = Node::new(db).await?;
    node.start().await?;
    Ok(())

}
