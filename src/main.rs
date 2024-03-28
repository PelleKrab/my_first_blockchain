use my_first_blockchain::{blockchain::Blockchain, utils::recover_public_key, utils:: generate_key_pair};
use my_first_blockchain::transaction;
use libp2p;
use secp256k1::{Secp256k1, PublicKey, SecretKey, rand, Message};
use sha3::{Digest, Keccak256};
use transaction::Transaction;
use my_first_blockchain::utils::{sign_transaction, public_key_to_address};


fn main() {
    // let mut blockchain = Blockchain::new();

    // blockchain.mine_block("Test");
    let (prikey, pubkey) = generate_key_pair();

    let sender = public_key_to_address(&pubkey);
    let receiver = "0x5678".to_string();
    let amount = 100;
    let nonce = 0;

    let signature1 = sign_transaction(prikey, sender.clone(), receiver.clone(), amount, nonce);
    let signature2 = sign_transaction(prikey, sender.clone(), receiver.clone(), amount+1, nonce+1);


    let message = format!("{}{}{}{}", sender, receiver, amount, nonce);
    let message_hash = Keccak256::digest(message.as_bytes());
    let message_hash = Message::from_digest_slice(&message_hash).expect("Failed to convert message hash");


    let transaction1 = Transaction::new(sender.clone(), receiver.clone(), amount, nonce, signature1.clone());
    let transaction2 = Transaction::new(sender.clone(), receiver.clone(), amount+1, nonce+1, signature2.clone());

    let recovered_pubkey = recover_public_key(&message_hash, &signature1);

    assert_eq!(recovered_pubkey, Ok(pubkey));
    
    println!("Public key: {:?}", pubkey);
    println!("Recovered public key: {:?}", recovered_pubkey);
    assert_eq!(transaction1.verify_signature(), true);

    let mut blockchain = Blockchain::new();
    let data = vec!(transaction1,transaction2);

    let result = blockchain.mine_block(&data);

    assert_eq!(result, true);

    assert_eq!(blockchain.is_chain_valid(), true);

    println!("New test:");
    let transaction_to_check = Transaction::new(sender.clone(), receiver.clone(), amount, nonce, signature1.clone());
    let is_included = blockchain.check_transaction_validity(&transaction_to_check);

    println!("Is transaction included: {}", is_included);

    
    // Quick frontend and add function that searches the chain for accounts
    // Mining transactions
    //Very simple double spend check. give money each time a block is mined.
}

