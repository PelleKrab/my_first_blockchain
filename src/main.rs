use std::hash;
use sha256::{digest, try_digest};


struct Block{
    index: u128,
    timestamp: std::time::SystemTime,
    data: String,
    previous_hash: String,
    hash: String,
}

fn main() {
    let mut blockChain: Vec<Block> = Vec::new();
    


    println!("Hello, world!");
}
