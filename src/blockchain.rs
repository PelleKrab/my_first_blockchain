use log::{error, info};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};



pub struct Block {
    index: u128,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

/// Represents a blockchain.
pub struct Blockchain {
    chain: Vec<Block>, // The chain of blocks in the blockchain.
    difficulty: u32,   // The difficulty level for mining new blocks.
}

impl Blockchain {
    /// Creates a new instance of the blockchain with a genesis block.
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
            difficulty: 1,
        }
    }

    /// Creates the genesis block of the blockchain.
    fn create_genesis_block() -> Block {
        // Create a new block with index 0, current timestamp, "Genesis Block" as data,
        // "0" as previous hash, "0" as hash, and nonce 0.
        let mut genesis_block = Block {
            index: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            hash: "0".to_string(),
            nonce: 0,
        };
        genesis_block.hash = genesis_block.calculate_hash();

        genesis_block
    }

    /// Adds a new block to the blockchain.
    pub fn add_block(&mut self, _data: String, _nonce: u64, _timestamp: u64) -> bool {
        let last_block = self.chain.last().unwrap();
        let new_block = Block {
            index: last_block.index + 1,
            timestamp: _timestamp,
            data: _data,
            previous_hash: last_block.hash.clone(),
            hash: String::new(),
            nonce: _nonce,
        };

        if self.is_block_valid(&new_block.calculate_hash())
            && new_block.index == last_block.index + 1
        {
            self.chain.push(new_block);
            return true;
        } else {
            error!("Block is not valid");
            return false;
        }
    }

    /// Checks if the blockchain is valid.
   pub fn is_chain_valid(&self, chain: &[Block]) -> bool {
        chain.windows(2).all(|window| {
            let first = &window[0];
            let second = &window[1];
            self.is_blockpair_valid(second, first)
        }) && self.is_block_valid(&self.chain.last().unwrap().calculate_hash())
    }

    /// Checks if a block pair is valid.
    fn is_blockpair_valid(&self, new: &Block, old: &Block) -> bool {
        if new.previous_hash != old.hash
            && new.index != old.index + 1
            && self.is_block_valid(&old.calculate_hash())
        {
            return false;
        }

        true
    }

    /// Checks if a block is valid.
    fn is_block_valid(&self, hash: &str) -> bool {
        let mut is_valid = true;

        for _ in 0..self.difficulty {
            if hash.chars().any(|c| c != '0') {
                is_valid = false;
                break;
            }
        }

        is_valid
    }

    /// Mines a new block with the given data.
    pub fn mine_block(&mut self, data: &str) -> bool {
        info!("mining block...");
        let mut nonce = 0;
        let mut _timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        loop {
            if nonce % 10000 == 0 {
                info!("nonce: {}", nonce);
            }

            let last_block = self.chain.last().unwrap();

            let new_block = Block {
                index: last_block.index + 1,
                timestamp: _timestamp,
                data: data.to_string(),
                previous_hash: last_block.hash.clone(),
                hash: String::new(),
                nonce: nonce,
            };

            let hash = new_block.calculate_hash();

            if self.is_block_valid(&hash) {
                return self.add_block(data.to_string(), nonce, _timestamp);
            }
            if nonce % 100000 == 0 {
                _timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
            }

            nonce += 1;
        }
    }


    // Getter for chain
    pub fn get_chain(&self) -> &[Block] {
        &self.chain
    }

    // Getter for chain length
    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }

    // Getter for difficulty
    pub fn get_difficulty(&self) -> u32 {
        self.difficulty
    }

}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.index.to_string().as_bytes());
        hasher.update(&self.timestamp.to_string().as_bytes());
        hasher.update(&self.data.as_bytes());
        hasher.update(&self.previous_hash.as_bytes());
    
        format!("{:x}", hasher.finalize())
    }
    // Getter for index
    pub fn get_index(&self) -> u128 {
        self.index
    }

    // Getter for timestamp
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    // Getter for data
    pub fn get_data(&self) -> &str {
        &self.data
    }

    // Getter for previous_hash
    pub fn get_previous_hash(&self) -> &str {
        &self.previous_hash
    }

    // Getter for hash
    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    // Getter for nonce
    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }
}
