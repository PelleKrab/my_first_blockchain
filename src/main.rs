use my_first_blockchain::transaction;
use my_first_blockchain::utils::{public_key_to_address, sign_transaction};
use my_first_blockchain::{blockchain::Blockchain, utils::generate_key_pair};
use std::sync::{Arc, Mutex};
use transaction::Transaction;

fn main() {
    // Create a new blockchain
    let mut blockchain1 = Blockchain::new();

    // Generate some key pairs for the transactions
    let (prikey1, pubkey1) = generate_key_pair();
    let (_, pubkey2) = generate_key_pair();

    // Create some transactions
    let transaction1 = vec![Transaction::new(
        public_key_to_address(&pubkey1),
        public_key_to_address(&pubkey2),
        10,
        0,
        sign_transaction(
            prikey1,
            public_key_to_address(&pubkey1),
            public_key_to_address(&pubkey2),
            10,
            0,
        ),
    )];

    println!("Difficulity is: {}", blockchain1.get_difficulty());

    // Mine a block with this transaction

    let arc_blockchain = Arc::new(Mutex::new(blockchain1.clone()));
    let transactions = Arc::new(transaction1.clone());
    blockchain1.mine_block(transactions, Arc::clone(&arc_blockchain));

    let mut blockchain = arc_blockchain.lock().unwrap();

    // Display the blockchain
    blockchain.print_chain();

    // Check if the blockchain is valid
    if blockchain.is_chain_valid() {
        println!("The blockchain is valid.");
    } else {
        println!("The blockchain is not valid.");
    }

    // Verify the transaction's signature
    let transaction = &blockchain.get_chain()[1].get_data_raw()[0];
    if transaction.verify_signature() {
        println!("The transaction's signature is valid.");
    } else {
        println!("The transaction's signature is not valid.");
    }

    // Check the transaction's inclusion in the Merkle tree
    if blockchain.check_transaction_validity(&transaction1[0]).unwrap_or(false) {
        println!("The transaction is included in the Merkle tree.");
    } else {
        println!("The transaction is not included in the Merkle tree.");
    }
}
