use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};


struct Block{
    index: u128,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain { chain: vec![Blockchain::create_genesis_block()], difficulty: 2 }
    }

    fn create_genesis_block() -> Block {
        Block {
            index: 0,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            hash: calculate_hash(0, 0, "Genesis Block", "0"),
            nonce: 0,
        }
    }

    fn add_block(&mut self, data: String, _nonce: u64) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block {
            index: last_block.index + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            data,
            previous_hash: last_block.hash.clone(),
            hash: String::new(),
            nonce: _nonce,  
        };

        // let new_block = calculate_hash(new_block.index, new_block.timestamp, &new_block.data, &new_block.previous_hash);

        if self.is_block_valid(&new_block) {
            
            self.chain.push(new_block);
        }else {
            println!("Block is not valid");
        }

        
    }

    fn is_block_valid(&self, block: &Block) -> bool {
        let mut is_valid = true;
        let mut hash = calculate_hash(block.index, block.timestamp, &block.data, &block.previous_hash);

        for _ in 0..self.difficulty {
            if hash.chars().any(|c| c != '0') {
                is_valid = false;
                break;
            }
        }

        is_valid
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
    let mut blockchain = Blockchain::new();

    // blockchain.add_block("First Block".to_string());
    // blockchain.add_block("Second Block".to_string());

    // println!("{}", blockchain.chain[0].hash);

}