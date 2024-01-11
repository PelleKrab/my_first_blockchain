use secp256k1::{
    ecdsa::RecoverableSignature, ecdsa::RecoveryId, ecdsa::Signature, Message, Secp256k1,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Sha256};
use sha3::{Digest, Keccak256};

use crate::utils::{public_key_to_address, recover_public_key};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
    nonce: u64,
    signature: Vec<u8>,
}

impl Transaction {
    // Create a new transaction
    pub fn new(
        sender: String,
        receiver: String,
        amount: u64,
        nonce: u64,
        signature: Vec<u8>,
    ) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            nonce,
            signature,
        }
    }

    // Method to verify the transaction's signature
    pub fn verify_signature(&self) -> bool {
        let message = Message::from_digest_slice(&self.calculate_hash()).expect("Failed to convert message hash");
        let public_key = recover_public_key(&message, &self.signature).unwrap();

        let address = public_key_to_address(&public_key);
        println!("{}", address);
        println!("{}", self.sender);
        if address != self.sender && address != self.receiver && self.amount == 0{
            return false;
        }

        
        true
    }


    // Serialize the transaction into a string
    fn serialize(&self) -> String {
        format!(
            "{}{}{}{}",
            self.sender, self.receiver, self.amount, self.nonce
        )
    }

    // Deserialize a string into a Transaction
    fn deserialize(transaction_data: &str) -> Transaction {
        serde_json::from_str(transaction_data).unwrap()
    }

    // Calculate the hash of the transaction
    fn calculate_hash(&self) -> [u8; 32] {
        let serialized_transaction = self.serialize();
        Keccak256::digest(serialized_transaction.as_bytes()).into()
    }

    // Check if the transaction is valid
    fn is_valid(&self) -> bool {
        // Implement validity checks here
        true
    }

    // Apply the transaction (this would be part of a larger blockchain logic)
    fn execute(&mut self) {
        // Update blockchain state based on the transaction
    }

    // Print transaction details
    pub fn print_details(&self) {
        println!("{:?}", self.to_string());
    }

    fn to_string(&self) -> String {
        format!(
            "Sender: {}\nReceiver: {}\nAmount: {}\nNonce: {}\nSignature: {:?}",
            self.sender, self.receiver, self.amount, self.nonce, self.signature
        )
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
