use chrono::Utc;
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

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
            difficulty: 1,
        }
    }

    fn create_genesis_block() -> Block {
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

    fn add_block(&mut self, data: String, _nonce: u64) -> bool {
        let last_block = self.chain.last().unwrap();
        let new_block = Block {
            index: last_block.index + 1,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
            previous_hash: last_block.hash.clone(),
            hash: String::new(),
            nonce: _nonce,
        };
x
        // let new_block = calculate_hash(new_block.index, new_block.timestamp, &new_block.data, &new_block.previous_hash);

        if self.is_block_valid(&calculate_hash(&new_block)) && new_block.index == last_block.index {
            self.chain.push(new_block);
            return true;
        } else {
            error!("Block is not valid");
            return false;
        }
    }

    pub fn getLastBlock(&self) -> &Block {
        self.chain.last().unwrap()
    }

    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        chain.windows(2).all(|window| {
            let first = &window[0];
            let second = &window[1];
            self.is_blockpair_valid(second, first)
        }) && self.is_block_valid(&calculate_hash(self.chain.last().unwrap()))
    }

    fn is_blockpair_valid(&self, new: &Block, old: &Block) -> bool {
        if new.previous_hash != old.hash && new.index != old.index + 1 && self.is_block_valid(&calculate_hash(old)) {
            return false;
        }

        true
    }

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
}

impl Block {
    fn mine_block(chain: &Blockchain, data: &str) -> Block {
        info!("mining block...");
        let mut nonce = 0;
        let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        loop {
            let last_block = chain.getLastBlock();

            let new_block = Block {
                index: last_block.index + 1,
                timestamp: timestamp,
                data: data.to_string(),
                previous_hash: last_block.hash.clone(),
                hash: String::new(),
                nonce: nonce,
            };

            let hash = calculate_hash(&new_block);

            if chain.is_block_valid(&hash) {
                return new_block;
            }

            if nonce % 100000 == 0 {
                timestamp = SystemTime::now()
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

    for n in 0..1000 {
        blockchain.add_block("TEST".to_string(), n);
    }

    for block in blockchain.chain {
        println!(
            "{} {} {} {} {}",
            block.index, block.timestamp, block.data, block.previous_hash, block.hash
        );
    }

    // blockchain.add_block("First Block".to_string());
    // blockchain.add_block("Second Block".to_string());

    // println!("{}", blockchain.chain[0].hash);
}
