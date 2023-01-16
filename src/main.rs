extern crate core;

use crate::blocks::blockchain::Blockchain;

pub mod blocks;
pub mod utils;
pub mod error;
pub mod transactions;


fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt().init();

    let mut bc = Blockchain::new();
    bc.mine_block("tom-alic 2 btc");
    bc.mine_block("jone -> 5 etc");
    bc.blocks_info();
}
