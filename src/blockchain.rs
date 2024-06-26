use crate::transaction::Transaction;
use log::{error, info};
use rs_merkle::{algorithms::Sha256 as mk_Sha256, Hasher, MerkleTree};
use sha2::{Digest, Sha256 as Sha2_256};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::vec::Vec;

#[derive(Clone)]
pub struct Block {
    index: u128,
    timestamp: u64,
    merkle_root: [u8; 32],
    data: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

/// Represents a blockchain.
#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>, // The chain of blocks in the blockchain.
    difficulty: usize, // The difficulty level for mining new blocks.
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockchainError {
    MerkleRootError(String),
    BlockInvalid(String),
    MerkleProofError(String),
    ChainInvalid,
    TransactionNotFound,
}

impl std::fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockchainError::MerkleRootError(ref err) => write!(f, "Merkle Root Error: {}", err),
            BlockchainError::BlockInvalid(ref err) => write!(f, "Block Invalid: {}", err),
            BlockchainError::MerkleProofError(message) => {
                write!(f, "Merkle proof error: {}", message)
            }
            BlockchainError::ChainInvalid => write!(f, "Blockchain is invalid"),
            BlockchainError::TransactionNotFound => write!(f, "Transaction not found"),
        }
    }
}

impl std::error::Error for BlockchainError {}

impl Blockchain {
    /// Creates a new instance of the blockchain with a genesis block.
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
            difficulty: 4,
        }
    }

    /// Creates the genesis block of the blockchain.
    fn create_genesis_block() -> Block {
        let mut genesis_block = Block {
            index: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|e| {
                    eprintln!("Error getting time since UNIX EPOCH: {:?}", e);
                    Duration::from_secs(0)
                })
                .as_secs(),
            data: vec![Transaction::new(
                "me".to_string(),
                "me".to_string(),
                10,
                0,
                "coinbase".into(),
            )],
            merkle_root: Self::calculate_merkle_root(&vec![Transaction::new(
                "me".to_string(),
                "me".to_string(),
                10,
                0,
                "coinbase".into(),
            )])
            .unwrap(),
            previous_hash: "0".to_string(),
            hash: "0".to_string(),
            nonce: 0,
        };

        genesis_block.hash = genesis_block.calculate_hash();

        genesis_block
    }

    fn calculate_merkle_tree(tx_list: &[Transaction]) -> Result<MerkleTree<mk_Sha256>, &str> {
        let leaves: Vec<[u8; 32]> = tx_list
            .iter()
            .map(|x| mk_Sha256::hash(x.serialize().as_bytes()))
            .collect();
        Ok(MerkleTree::<mk_Sha256>::from_leaves(&leaves))
    }

    fn calculate_merkle_root(tx_list: &[Transaction]) -> Result<[u8; 32], BlockchainError> {
        let merkle_tree = Self::calculate_merkle_tree(tx_list)
            .map_err(|e| BlockchainError::MerkleRootError(e.to_string()))?;

        let merkle_root = merkle_tree.root().ok_or_else(|| {
            BlockchainError::MerkleRootError("couldn't get the merkle root".to_string())
        })?;

        Ok(merkle_root)
    }

    fn merkle_transaction_proof(
        &self,
        transaction: &Transaction,
        block_index: usize,
        tx_index: usize,
    ) -> Result<bool, BlockchainError> {
        let block = self.chain.get(block_index).ok_or_else(|| {
            BlockchainError::MerkleProofError("Block index out of bounds.".into())
        })?;

        let leaves: Vec<[u8; 32]> = block
            .get_data_raw()
            .iter()
            .map(|x| mk_Sha256::hash(x.serialize().as_bytes()))
            .collect();

        let merkle_tree = MerkleTree::<mk_Sha256>::from_leaves(&leaves);
        let proof = merkle_tree.proof(&[tx_index]);
        let leave_to_prove = [mk_Sha256::hash(transaction.serialize().as_bytes())];
        let root = merkle_tree.root().ok_or_else(|| {
            BlockchainError::MerkleProofError("Couldn't get the Merkle root.".into())
        })?;

        Ok(proof.verify(root, &[tx_index], &leave_to_prove, leaves.len()))
    }

    fn find_block(&self, transaction: &Transaction) -> Result<(usize, usize), BlockchainError> {
        for (block_index, block) in self.chain.iter().enumerate() {
            // Use the index variable here
            for (tx_index, tx) in block.get_data_raw().iter().enumerate() {
                if tx.get_signature() == transaction.get_signature() {
                    return Ok((block_index, tx_index));
                }
            }
        }
        Err(BlockchainError::TransactionNotFound)
    }

    pub fn check_transaction_validity(
        &mut self,
        transaction: &Transaction,
    ) -> Result<bool, BlockchainError> {
        let (block_index, tx_index) = self.find_block(transaction)?;
        self.merkle_transaction_proof(transaction, block_index, tx_index)
    }

    /// Adds a new block to the blockchain.
    pub fn add_block(&mut self, new_block: Block) -> bool {
        //checks if the blockchain is empty
        if let Some(last_block) = self.chain.last() {
            if !self.is_block_valid(&new_block.calculate_hash())
                || new_block.index != last_block.index + 1
            {
                error!("Block is not valid or does not follow the last block in the chain.");
                return false;
            }
        }

        self.chain.push(new_block);
        true
    }

    /// Checks if the blockchain is valid.
    pub fn is_chain_valid(&self) -> bool {
        if self.chain.len() <= 1 {
            return self
                .chain
                .last()
                .map(|block| self.is_block_valid(&block.calculate_hash()))
                .unwrap_or(true);
        }
        self.chain.windows(2).all(|window| {
            if let [first, second] = window {
                self.is_blockpair_valid(second, first).is_ok()
            } else {
                false
            }
        }) && self
            .chain
            .last()
            .map(|block| self.is_block_valid(&block.calculate_hash()))
            .unwrap_or(false)
    }

    /// Checks if a block pair is valid.
    fn is_blockpair_valid(&self, new: &Block, old: &Block) -> Result<(), &str> {
        // Genesis block edge case
        if old.index == 0 {
            return Ok(());
        }

        if new.index != old.index + 1 {
            panic!("Invalid block index");
        }

        if !self.is_block_valid(&old.calculate_hash()) {
            panic!("Invalid previous block hash");
        }

        if new.previous_hash != old.calculate_hash() {
            if new.previous_hash != old.calculate_hash() {
                panic!("Invalid previous hash");
            }
        }

        Ok(())
    }

    /// Checks if a block is valid.
    fn is_block_valid(&self, hash: &str) -> bool {
        hash.starts_with(&"0".repeat(self.difficulty as usize))
    }

    pub fn mine_block(
        &mut self,
        data: Arc<Vec<Transaction>>,
        blockchain: Arc<Mutex<Blockchain>>,
    ) -> bool {
        let cores = num_cpus::get();
        let mut handlers = vec![];

        // Move the blockchain-related calculations outside of the threads
        let bc = match blockchain.lock() {
            Ok(bc) => bc,
            Err(e) => {
                log::error!("Failed to acquire blockchain lock: {:?}", e);
                return false;
            }
        };
        let last_block = match bc.chain.last() {
            Some(block) => block,
            None => {
                log::error!("The blockchain is empty.");
                return false;
            }
        };

        let last_index = last_block.index + 1;
        let last_hash = last_block.hash.clone();
        let merkleroot = match Self::calculate_merkle_root(&data) {
            Ok(root) => root,
            Err(e) => {
                log::error!("Failed to calculate Merkle root: {}", e);
                return false; // Return false to indicate failure
            }
        };

        // Drop the lock to allow threads to use the blockchain later
        drop(bc);

        for i in 0..cores {
            let data_clone = Arc::clone(&data);
            let blockchain_clone = Arc::clone(&blockchain);
            let last_hash_clone = last_hash.clone();
            let merkleroot_clone = merkleroot.clone();

            let handler = thread::spawn(move || {
                let mut nonce = i as u64;
                let nonce_step = cores;
                let mut _timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_else(|e| {
                        eprintln!("Error getting time since UNIX EPOCH: {:?}", e);
                        Duration::from_secs(0)
                    })
                    .as_secs();
                let mut new_block = Block {
                    index: last_index,
                    timestamp: _timestamp,
                    data: (*data_clone).clone(),
                    merkle_root: merkleroot_clone.clone(),
                    previous_hash: last_hash_clone.clone(),
                    hash: String::new(),
                    nonce: nonce,
                };

                loop {
                    new_block.nonce = nonce;

                    let hash = new_block.calculate_hash();

                    {
                        let mut bc = match blockchain_clone.lock() {
                            Ok(bc) => bc,
                            Err(e) => {
                                log::error!("Failed to acquire blockchain lock: {:?}", e);
                                return false;
                            }
                        };
                        if bc.is_block_valid(&hash) {
                            new_block.hash = hash;
                            bc.add_block(new_block);
                            return true;
                        }
                    }

                    nonce += nonce_step as u64;

                    if nonce % 1000000 == 0 {
                        new_block.timestamp = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_else(|e| {
                                eprintln!("Error getting time since UNIX EPOCH: {:?}", e);
                                Duration::from_secs(0)
                            })
                            .as_secs();
                    }
                }
            });

            handlers.push(handler);
        }

        for handler in handlers {
            match handler.join() {
                Ok(success) if success => return true, // If the thread returns true, the block was mined successfully.
                Ok(_) => continue, // The thread finished but didn't mine the block successfully.
                Err(e) => {
                    log::error!("A thread panicked while mining: {:?}", e);
                }
            }
        }
        

        false
    }

    //Legacy function
    pub fn mine_block_singlethread(&mut self, data: &Vec<Transaction>) -> bool {
        info!("mining block...");
        let mut nonce = 0;
        let mut _timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|e| {
                eprintln!("Error getting time since UNIX EPOCH: {:?}", e);
                Duration::from_secs(0)
            })
            .as_secs();
        let merkleroot = Self::calculate_merkle_root(&data);
        loop {
            if nonce % 10000 == 0 {
                info!("nonce: {}", nonce);
            }

            let last_block = self.chain.get(self.chain.len() - 1).unwrap();

            let mut new_block = Block {
                index: last_block.index + 1,
                timestamp: _timestamp,
                data: data.clone(),
                merkle_root: merkleroot.clone().unwrap(),
                previous_hash: last_block.hash.clone(),
                hash: String::new(),
                nonce: nonce,
            };

            let hash = new_block.calculate_hash();
            new_block.hash = hash.clone();

            if self.is_block_valid(&hash) {
                return self.add_block(new_block);
            }
            if nonce % 100000 == 0 {
                new_block.timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_else(|e| {
                        eprintln!("Error getting time since UNIX EPOCH: {:?}", e);
                        Duration::from_secs(0)
                    })
                    .as_secs();
            }

            nonce += 1;
        }
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }

    pub fn get_difficulty(&self) -> usize {
        self.difficulty
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("Block Index: {}", block.get_index());
            println!("Timestamp: {}", block.get_timestamp());
            println!("Merkle Root: {:?}", block.merkle_root);
            println!("Previous Hash: {}", block.get_previous_hash());
            println!("Hash: {}", block.get_hash());
            println!("Nonce: {}", block.get_nonce());
            println!("Data:");
            for transaction in block.get_data_raw() {
                println!("  Transaction: {:?}", transaction);
            }
            println!("------------------------");
        }
    }
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha2_256::new();
        hasher.update(&self.index.to_string().as_bytes());
        hasher.update(&self.timestamp.to_string().as_bytes());
        hasher.update(&self.merkle_root);
        hasher.update(&self.previous_hash.as_bytes());
        hasher.update(&self.nonce.to_be_bytes());

        format!("{:x}", hasher.finalize())
    }

    pub fn get_index(&self) -> u128 {
        self.index
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_data_raw(&self) -> &Vec<Transaction> {
        &self.data
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_previous_hash(&self) -> &str {
        &self.previous_hash
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }
}
