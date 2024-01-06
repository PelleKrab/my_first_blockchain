use log::{error, info};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

struct Block {
    index: u128,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

/// Represents a blockchain.
struct Blockchain {
    chain: Vec<Block>, // The chain of blocks in the blockchain.
    difficulty: u32,   // The difficulty level for mining new blocks.
}

impl Blockchain {
    /// Creates a new instance of the blockchain with a genesis block.
    fn new() -> Blockchain {
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
        genesis_block.hash = calculate_hash(&genesis_block);

        genesis_block
    }

    /// Adds a new block to the blockchain.
    fn add_block(&mut self, _data: String, _nonce: u64, _timestamp: u64) -> bool {
        let last_block = self.chain.last().unwrap();
        let new_block = Block {
            index: last_block.index + 1,
            timestamp: _timestamp,
            data: _data,
            previous_hash: last_block.hash.clone(),
            hash: String::new(),
            nonce: _nonce,
        };

        if self.is_block_valid(&calculate_hash(&new_block))
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
    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        chain.windows(2).all(|window| {
            let first = &window[0];
            let second = &window[1];
            self.is_blockpair_valid(second, first)
        }) && self.is_block_valid(&calculate_hash(self.chain.last().unwrap()))
    }

    /// Checks if a block pair is valid.
    fn is_blockpair_valid(&self, new: &Block, old: &Block) -> bool {
        if new.previous_hash != old.hash
            && new.index != old.index + 1
            && self.is_block_valid(&calculate_hash(old))
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
    pub fn mine_block(&self, data: &str) -> (u64, u64) {
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

            let hash = calculate_hash(&new_block);

            if self.is_block_valid(&hash) {
                return (nonce, _timestamp);
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
}

fn calculate_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&block.index.to_string().as_bytes());
    hasher.update(&block.timestamp.to_string().as_bytes());
    hasher.update(&block.data.as_bytes());
    hasher.update(&block.previous_hash.as_bytes());

    format!("{:x}", hasher.finalize())
}

fn main() {
    let mut blockchain = Blockchain::new();

    let (nonce, timestamp) = blockchain.mine_block("Test");
    blockchain.add_block("Test".to_string(), nonce, timestamp);

    for block in blockchain.chain {
        println!(
            "{} {} {} {} {}",
            block.index, block.timestamp, block.data, block.previous_hash, block.hash
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_chain_valid() {
        let mut blockchain = Blockchain::new();

        let (nonce, timestamp) = blockchain.mine_block("Test");
        blockchain.add_block("Test".to_string(), nonce, timestamp);

        assert!(blockchain.is_chain_valid(&blockchain.chain));
    }

    #[test]
    fn test_is_blockpair_valid() {
        let block1 = Block {
            index: 0,
            timestamp: 0,
            data: "Test1".to_string(),
            previous_hash: String::from(""),
            hash: String::from("hash1"),
            nonce: 0,
        };

        let block2 = Block {
            index: 1,
            timestamp: 0,
            data: "Test2".to_string(),
            previous_hash: String::from("hash1"),
            hash: String::from("hash2"),
            nonce: 0,
        };

        let blockchain = Blockchain::new();

        assert!(blockchain.is_blockpair_valid(&block2, &block1));
    }

    #[test]
    fn test_is_block_valid() {
        let blockchain = Blockchain::new();
        let hash = "0000";

        assert!(blockchain.is_block_valid(hash));
    }

    #[test]
    fn test_mine_block() {
        let mut blockchain = Blockchain::new();
        let data = "Test";

        let (nonce, timestamp) = blockchain.mine_block(data);
        blockchain.add_block(data.to_string(), nonce, timestamp);

        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.chain[1].data, data);
        assert_eq!(blockchain.chain[1].nonce, nonce);
        assert_eq!(blockchain.chain[1].timestamp, timestamp);
    }
}
