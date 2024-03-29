use crate::transaction::{self, Transaction};
use libp2p::multihash::Error;
use log::{error, info};
use rs_merkle::{algorithms::Sha256 as mk_Sha256, Hasher, MerkleTree};
use secp256k1::rand::seq::index;
use serde::Serialize;
use sha2::{Digest, Sha256 as Sha2_256};

// use sha256::{digest, try_digest};
use std::{
    string,
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

pub struct Block {
    index: u128,
    timestamp: u64,
    merkle_root: [u8; 32],
    data: vec::Vec<Transaction>,
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
            difficulty: 0,
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

    fn calculate_merkle_tree(tx_list: &Vec<Transaction>) -> Result<MerkleTree<mk_Sha256>, &str> {
        let leaves: Vec<[u8; 32]> = tx_list
            .iter()
            .map(|x| mk_Sha256::hash(x.serialize().as_bytes()))
            .collect();

        Ok(MerkleTree::<mk_Sha256>::from_leaves(&leaves))
    }

    fn calculate_merkle_root(tx_list: &Vec<Transaction>) -> Result<[u8; 32], &str> {
        let merkle_tree = Self::calculate_merkle_tree(tx_list)?;
        let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root")?;
        Ok(merkle_root)
    }

    fn merkle_transaction_proof(
        &self,
        transaction: &Transaction,
        block_index: usize,
        tx_index: usize,
    ) -> bool {
        let leaves: Vec<[u8; 32]> = self
            .chain
            .get(block_index)
            .unwrap()
            .get_data_raw()
            .iter()
            .map(|x| mk_Sha256::hash(x.serialize().as_bytes()))
            .collect();

        let merkle_tree = MerkleTree::<mk_Sha256>::from_leaves(&leaves);

        let proof = merkle_tree.proof(&[tx_index]);
        let indice_to_prove = [tx_index];
        let leave_to_prove = [mk_Sha256::hash(transaction.serialize().as_bytes())];
        let root = merkle_tree.root().ok_or("couldn't get the merkle root");

        proof.verify(
            root.unwrap(),
            &indice_to_prove,
            &leave_to_prove,
            leaves.len(),
        )
    }

    fn find_block(&self, transaction: &Transaction) -> Result<(usize, usize), &str> {
        for (block_index, block) in self.chain.iter().enumerate() {
            // Use the index variable here
            for (tx_index, tx) in block.get_data_raw().iter().enumerate() {
                if tx.get_signature() == transaction.get_signature() {
                    return Ok((block_index, tx_index));
                }
            }
        }
        Err("Transaction not found")
    }

    pub fn check_transaction_validity(&mut self, transaction: &Transaction) -> bool {
        let (block_index, tx_index) = match self.find_block(transaction) {
            Ok((block_index, tx_index)) => (block_index, tx_index),
            Err(_) => {
                // Transaction not found
                return false;
            }
        };

        self.merkle_transaction_proof(transaction, block_index, tx_index)
    }

    /// Adds a new block to the blockchain.
    pub fn add_block(&mut self, new_block: Block) -> bool {
        let last_block = self.chain.last().unwrap();

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
    pub fn is_chain_valid(&self) -> bool {
        if self.chain.len() <= 1 {
            self.is_block_valid(&self.chain.last().unwrap().calculate_hash())
        } else {
            self.chain.windows(2).all(|window| {
                let first = &window[0];
                let second = &window[1];
                self.is_blockpair_valid(second, first).is_ok() // Return false if validation fails
            }) && self.is_block_valid(&self.chain.last().unwrap().calculate_hash())
        }
    }

    /// Checks if a block pair is valid.
    fn is_blockpair_valid(&self, new: &Block, old: &Block) -> Result<(), &str> {
        if new.index != old.index + 1 {
            panic!("Invalid block index");
        }

        if !self.is_block_valid(&old.calculate_hash()) {
            panic!("Invalid previous block hash");
        }

        if new.previous_hash != old.calculate_hash() {
            let new_hash = new.get_previous_hash();
            let new_hash1 = new.get_hash();
            let prev_hash = old.calculate_hash();
            let index1 = new.get_nonce();

            if new.previous_hash != prev_hash {
                panic!("Invalid previous hash");
            }
        }

        Ok(())
    }

    /// Checks if a block is valid.
    fn is_block_valid(&self, hash: &str) -> bool {
        hash.starts_with(&"0".repeat(self.difficulty as usize))
    }

    /// Mines a new block with the given data.
    pub fn mine_block(&mut self, data: &Vec<Transaction>) -> bool {
        info!("mining block...");
        let mut nonce = 0;
        let mut _timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
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
                    .unwrap()
                    .as_secs();
            }

            nonce += 1;
        }
    }

    fn verify_transaction_block(&self, transaction: &Transaction, block: &Block) {
        //Check transaction data to merkle in verfy block

        !todo!()
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }

    pub fn get_difficulty(&self) -> u32 {
        self.difficulty
    }


    /// Prints the entire blockchain.
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
