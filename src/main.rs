mod blockchain;
use blockchain::Blockchain;
mod transaction;
use libp2p;
use secp256k1::{Secp256k1, PublicKey, SecretKey, rand, Message};
use sha3::{Digest, Keccak256};
use transaction::Transaction;

mod utils;
use utils::sign_transaction;

fn generate_key_pair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
    (secret_key, public_key)
}

fn main() {
    // let mut blockchain = Blockchain::new();

    // blockchain.mine_block("Test");
    let (prikey, pubkey) = generate_key_pair();

    let sender = "0x1234".to_string();
    let receiver = "0x5678".to_string();
    let amount = 100;
    let nonce = 0;

    let signature = sign_transaction(prikey, sender.clone(), receiver.clone(), amount, nonce);

    let message = format!("{}{}{}{}", sender, receiver, amount, nonce);
    let message_hash = Keccak256::digest(message.as_bytes());
    let message_hash = Message::from_digest_slice(&message_hash).expect("Failed to convert message hash");


    let transaction = Transaction::new(sender.clone(), receiver.clone(), amount, nonce, signature.clone());

    let recovered_pubkey = transaction.recover_public_key(&message_hash, &signature);

    assert_eq!(recovered_pubkey, Ok(pubkey));
    println!("Public key: {:?}", pubkey);
    println!("Recovered public key: {:?}", recovered_pubkey);

    

    
}

