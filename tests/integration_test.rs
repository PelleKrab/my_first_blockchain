use my_first_blockchain::blockchain::Block;
use my_first_blockchain::blockchain::Blockchain;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
mod tests {

    use std::vec;

    use my_first_blockchain::{
        transaction::Transaction,
        utils::{generate_key_pair, public_key_to_address, recover_public_key, sign_transaction},
    };
    use secp256k1::Message;
    use sha3::{Digest, Keccak256};

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
        //sender: String,
        // receiver: String,
        // amount: u64,
        // nonce: u64,
        // signature: Vec<u8>
        let data = vec![
            Transaction::new(
                "sender1".to_string(),
                "receiver1".to_string(),
                10,
                0,
                "signature1".to_vec(),
            ),
            Transaction::new(
                "sender2".to_string(),
                "receiver2".to_string(),
                20,
                1,
                "signature2".to_string(),
            ),
            Transaction::new(
                "sender3".to_string(),
                "receiver3".to_string(),
                30,
                2,
                "signature3".to_string(),
            ),
        ];

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
        assert_eq!(
            new_block.get_previous_hash(),
            blockchain.get_chain()[0].get_hash()
        );
        assert_ne!(new_block.get_hash(), "0");
        assert_ne!(new_block.get_nonce(), 0);
    }

    #[test]
    fn test_transaction_sender_valid() {
        #[test]
        fn test_transaction_sender_valid() {
            let (prikey, pubkey) = generate_key_pair();

            let sender = public_key_to_address(&pubkey);
            let receiver = "0x5678".to_string();
            let amount = 100;
            let nonce = 0;

            let signature =
                sign_transaction(prikey, sender.clone(), receiver.clone(), amount, nonce);

            let message = format!("{}{}{}{}", sender, receiver, amount, nonce);
            let message_hash = Keccak256::digest(message.as_bytes());
            let message_hash =
                Message::from_digest_slice(&message_hash).expect("Failed to convert message hash");

            let transaction = Transaction::new(
                sender.clone(),
                receiver.clone(),
                amount,
                nonce,
                signature.clone(),
            );

            let recovered_pubkey = recover_public_key(&message_hash, &signature);

            assert_eq!(recovered_pubkey, Ok(pubkey));

            assert_eq!(transaction.verify_signature(), true);
        }
    }
}
