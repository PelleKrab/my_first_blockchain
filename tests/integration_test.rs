use my_first_blockchain::blockchain::Blockchain;

#[test]
fn test_blockchain_creation() {
    let blockchain = Blockchain::new();
    assert_eq!(blockchain.get_chain().len(), 1);
    assert_eq!(blockchain.get_difficulty(), 1);
}

#[test]
fn test_blockchain_add_block() {
    let mut blockchain = Blockchain::new();
    let data = "Test Block".to_string();
    let nonce = 123;
    let timestamp = 123456789;

    assert!(blockchain.add_block(data.clone(), nonce, timestamp));
    assert_eq!(blockchain.get_chain().len(), 2);
    assert_eq!(blockchain.get_chain()[1].get_data(), data);
}

#[test]
fn test_blockchain_mine_block() {
    let mut blockchain = Blockchain::new();
    let data = "Test Block".to_string();

    assert!(blockchain.mine_block(&data));
    assert_eq!(blockchain.get_chain().len(), 2);
    assert_eq!(blockchain.get_chain()[1].get_data(), data);
}

#[test]
fn test_blockchain_is_chain_valid() {
    let mut blockchain = Blockchain::new();
    let data = "Test Block".to_string();

    assert!(blockchain.mine_block(&data));
    assert!(blockchain.is_chain_valid(&blockchain.get_chain()));
}

#[test]
fn test_blockchain_is_block_valid() {
    let mut blockchain = Blockchain::new();
    let data = "Test Block".to_string();

    assert!(blockchain.mine_block(&data));
    assert!(blockchain.is_block_valid(&blockchain.get_chain()[1].get_hash()));
}

#[test]
fn test_block_getters() {
    let block = Blockchain::create_genesis_block();
    assert_eq!(block.get_index(), 0);
    assert_eq!(block.get_timestamp(), 0);
    assert_eq!(block.get_data(), "Genesis Block");
    assert_eq!(block.get_previous_hash(), "0");
    assert_eq!(block.get_hash(), "0");
    assert_eq!(block.get_nonce(), 0);
}
