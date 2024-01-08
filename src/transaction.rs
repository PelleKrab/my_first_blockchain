use secp256k1::{
    ecdsa::RecoverableSignature, ecdsa::RecoveryId, ecdsa::Signature, Message, Secp256k1,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

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
    fn verify_signature(&self) -> bool {
        // self.signature

        true
    }

    pub fn recover_public_key(
        &self,
        msg: &Message,
        sig: &[u8],
    ) -> Result<secp256k1::PublicKey, secp256k1::Error> {
        let secp = Secp256k1::new();
        // let message = Message::from_digest_slice(msg)?;
        let recovery_id_value = sig[64] as i32 - 27;
        let recovery_id = RecoveryId::from_i32(recovery_id_value)?;
        let signature = RecoverableSignature::from_compact(&sig[0..64], recovery_id)?;
        secp.recover_ecdsa(&msg, &signature)
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
    fn calculate_hash(&self) -> Vec<u8> {
        let serialized_transaction = self.serialize();
        let mut hasher = Sha256::new();
        hasher.update(serialized_transaction.as_bytes());
        hasher.finalize().to_vec()
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
