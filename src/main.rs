use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};


struct Block{
    index: u128,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain { chain: vec![Blockchain::create_genesis_block()] }
    }

    fn create_genesis_block() -> Block {
        Block {
            index: 0,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            hash: calculate_hash(0, 0, "Genesis Block", "0"),
        }
    }

    fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block {
            index: last_block.index + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            data,
            previous_hash: last_block.hash.clone(),
            hash: String::new(), // We'll calculate this next
        };

        let new_block_hash = calculate_hash(new_block.index, new_block.timestamp, &new_block.data, &new_block.previous_hash);
        let new_block = Block { hash: new_block_hash, ..new_block };

        self.chain.push(new_block);
    }
}


fn calculate_hash(index:u128, timestamp: u64, data: &str, previous_hash: &str)  -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_string().as_bytes());
    hasher.update(timestamp.to_string().as_bytes());
    hasher.update(data.as_bytes());
    hasher.update(previous_hash.as_bytes());

    format!("{:x}", hasher.finalize())
}


fn main() {
    //let mut blockChain: Vec<Block> = Vec::new();
    
    // let genesis_block = Block{
    //     index: 0,
    //     timestamp: SystemTime::now(),
    //     data: String::from("Genesis Block"),
    //     previous_hash: String::from("0"),
    //     hash: calculate_hash(0, SystemTime::now(), "Genesis Block", "0"),
    // };
    
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First Block".to_string());
    blockchain.add_block("Second Block".to_string());

    println!("{}", blockchain.chain[0].hash);
}