use my_first_blockchain::blockchain::Blockchain;
use std::time::{SystemTime, UNIX_EPOCH};
use my_first_blockchain::blockchain::Block;

#[cfg(test)]
mod tests { 


    use super::*;

    #[test]
    fn test_create_genesis_block() {
        let blockchain = Blockchain::new();
        let genesis_block = &blockchain.get_chain()[0];
        let hash = genesis_block.calculate_hash();

        assert_eq!(genesis_block.get_index(), 0);
        assert_eq!(genesis_block.get_data(), "Genesis Block");
        assert_eq!(genesis_block.get_previous_hash(), "0");
        assert_eq!(genesis_block.get_hash(), hash);
        assert_eq!(genesis_block.get_nonce(), 0);
    }

    #[test]
    fn test_is_chain_valid() {
        let mut blockchain = Blockchain::new();
        let data = "Test Block".to_string();
        
        for _ in 0..5 {
            blockchain.mine_block(data.clone());
        }

        assert_eq!(blockchain.is_chain_valid(blockchain.get_chain()), true);
    }

    #[test]
    fn test_mine_block() {
        let mut blockchain = Blockchain::new();
        let data = "Test Block".to_string();

        let result = blockchain.mine_block(data.clone());

        assert_eq!(result, true);
        assert_eq!(blockchain.get_chain().len(), 2);

        let new_block = &blockchain.get_chain()[1];
        assert_eq!(new_block.get_index(), 1);
        assert_eq!(new_block.get_data(), data);
        assert_eq!(new_block.get_previous_hash(), blockchain.get_chain()[0].get_hash());
        assert_ne!(new_block.get_hash(), "0");
        assert_ne!(new_block.get_nonce(), 0);
    }

    
}