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
impl Clone for Transaction {
    fn clone(&self) -> Self {
        Transaction {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
            amount: self.amount,
            nonce: self.nonce,
            signature: self.signature.clone(),
        }
    }
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
        if address != self.sender && address != self.receiver && self.amount == 0{
            return false;
        }

        
        true
    }


    // Serialize the transaction into a JSON string
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // Deserialize a JSON string into a Transaction
    pub fn deserialize(transaction_data: &str) -> Transaction {
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

    pub fn get_sender(&self) -> &str {
        &self.sender
    }

    pub fn get_receiver(&self) -> &str {
        &self.receiver
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    
}

impl std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction {{ sender: {}, receiver: {}, amount: {}, nonce: {}, signature: {:?} }}",
            self.sender, self.receiver, self.amount, self.nonce, self.signature
        )
    }
}