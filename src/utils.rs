
use secp256k1::{SecretKey, Secp256k1, Message, ecdsa::Signature, ecdsa::RecoverableSignature}; 
use sha3::{Digest, Keccak256};


pub fn sign_transaction(key: SecretKey, sender: String, receiver: String, amount: u64, nonce: u64) -> Vec<u8> {
    let message = format!("{}{}{}{}", sender, receiver, amount, nonce);
    let message_hash = Keccak256::digest(message.as_bytes());
    let secp = Secp256k1::new();

    let message_hash = Message::from_digest_slice(&message_hash).expect("Failed to convert message hash");

    let recoverable_sig = secp.sign_ecdsa_recoverable(&message_hash, &key);
    let (rec_id, sig_bytes) = recoverable_sig.serialize_compact();

    let mut signature = sig_bytes.to_vec();
    signature.push(rec_id.to_i32() as u8 + 27); // Adding 27 to match Ethereum's recovery ID offset

    signature
}