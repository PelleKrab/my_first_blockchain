use my_first_blockchain::blockchain::Blockchain;

#[cfg(test)]
mod tests {

    use std::{
        sync::{Arc, Mutex},
        vec,
    };

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
        assert_eq!(
            genesis_block.get_data_raw().get(0).unwrap().get_sender(),
            "me"
        );
        assert_eq!(genesis_block.get_previous_hash(), "0");
        assert_eq!(genesis_block.get_hash(), hash);
        assert_eq!(genesis_block.get_nonce(), 0);
    }

    #[test]
    fn test_is_chain_valid() {
        let mut blockchain = Blockchain::new();
        let (prikey1, pubkey1) = generate_key_pair();
        let (prikey2, pubkey2) = generate_key_pair();
        let (prikey3, pubkey3) = generate_key_pair();
        let (prikey4, pubkey4) = generate_key_pair();
        let (prikey5, pubkey5) = generate_key_pair();
        let (prikey6, pubkey6) = generate_key_pair();

        let transactions = vec![
            vec![
                Transaction::new(
                    "sender1".to_string(),
                    "receiver1".to_string(),
                    10,
                    0,
                    sign_transaction(
                        prikey1,
                        public_key_to_address(&pubkey1),
                        public_key_to_address(&pubkey2),
                        10,
                        0,
                    ),
                ),
                Transaction::new(
                    "sender2".to_string(),
                    "receiver2".to_string(),
                    10,
                    0,
                    sign_transaction(
                        prikey2,
                        public_key_to_address(&pubkey2),
                        public_key_to_address(&pubkey3),
                        10,
                        0,
                    ),
                ),
                Transaction::new(
                    "sender3".to_string(),
                    "receiver3".to_string(),
                    10,
                    0,
                    sign_transaction(
                        prikey3,
                        public_key_to_address(&pubkey3),
                        public_key_to_address(&pubkey1),
                        10,
                        0,
                    ),
                ),
            ],
            vec![
                Transaction::new(
                    "sender1".to_string(),
                    "receiver1".to_string(),
                    10,
                    1,
                    sign_transaction(
                        prikey1,
                        public_key_to_address(&pubkey1),
                        public_key_to_address(&pubkey2),
                        10,
                        1,
                    ),
                ),
                Transaction::new(
                    "sender2".to_string(),
                    "receiver2".to_string(),
                    10,
                    1,
                    sign_transaction(
                        prikey2,
                        public_key_to_address(&pubkey2),
                        public_key_to_address(&pubkey3),
                        10,
                        1,
                    ),
                ),
                Transaction::new(
                    "sender3".to_string(),
                    "receiver3".to_string(),
                    0,
                    1,
                    sign_transaction(
                        prikey3,
                        public_key_to_address(&pubkey3),
                        public_key_to_address(&pubkey1),
                        10,
                        1,
                    ),
                ),
            ],
            vec![
                Transaction::new(
                    "sender1".to_string(),
                    "receiver1".to_string(),
                    10,
                    2,
                    sign_transaction(
                        prikey1,
                        public_key_to_address(&pubkey1),
                        public_key_to_address(&pubkey2),
                        10,
                        0,
                    ),
                ),
                Transaction::new(
                    "sender2".to_string(),
                    "receiver2".to_string(),
                    10,
                    2,
                    sign_transaction(
                        prikey2,
                        public_key_to_address(&pubkey2),
                        public_key_to_address(&pubkey3),
                        10,
                        0,
                    ),
                ),
                Transaction::new(
                    "sender3".to_string(),
                    "receiver3".to_string(),
                    10,
                    2,
                    sign_transaction(
                        prikey3,
                        public_key_to_address(&pubkey3),
                        public_key_to_address(&pubkey1),
                        10,
                        0,
                    ),
                ),
            ],
            vec![
                Transaction::new(
                    "sender4".to_string(),
                    "receiver4".to_string(),
                    10,
                    3,
                    sign_transaction(
                        prikey4,
                        public_key_to_address(&pubkey4),
                        public_key_to_address(&pubkey5),
                        10,
                        3,
                    ),
                ),
                Transaction::new(
                    "sender5".to_string(),
                    "receiver5".to_string(),
                    10,
                    3,
                    sign_transaction(
                        prikey5,
                        public_key_to_address(&pubkey5),
                        public_key_to_address(&pubkey6),
                        10,
                        3,
                    ),
                ),
                Transaction::new(
                    "sender6".to_string(),
                    "receiver6".to_string(),
                    10,
                    3,
                    sign_transaction(
                        prikey6,
                        public_key_to_address(&pubkey6),
                        public_key_to_address(&pubkey4),
                        10,
                        3,
                    ),
                ),
            ],
            vec![
                Transaction::new(
                    "sender4".to_string(),
                    "receiver4".to_string(),
                    10,
                    4,
                    sign_transaction(
                        prikey4,
                        public_key_to_address(&pubkey4),
                        public_key_to_address(&pubkey5),
                        10,
                        4,
                    ),
                ),
                Transaction::new(
                    "sender5".to_string(),
                    "receiver5".to_string(),
                    10,
                    4,
                    sign_transaction(
                        prikey5,
                        public_key_to_address(&pubkey5),
                        public_key_to_address(&pubkey6),
                        10,
                        4,
                    ),
                ),
                Transaction::new(
                    "sender6".to_string(),
                    "receiver6".to_string(),
                    10,
                    4,
                    sign_transaction(
                        prikey6,
                        public_key_to_address(&pubkey6),
                        public_key_to_address(&pubkey4),
                        10,
                        4,
                    ),
                ),
            ],
        ];

        let arc_blockchain = Arc::new(Mutex::new(blockchain.clone()));

        for data in transactions {
            let transactions = Arc::new(data.clone());
            blockchain.mine_block(transactions, Arc::clone(&arc_blockchain));
        }
        let updated_blockchain = arc_blockchain.lock().unwrap();
        assert_eq!(updated_blockchain.is_chain_valid(), true);
    }

    #[test]
    fn test_mine_block() {
        let (prikey1, pubkey1) = generate_key_pair();
        let (prikey2, pubkey2) = generate_key_pair();

        let mut blockchain = Blockchain::new();
        let data = vec![Transaction::new(
            "sender1".to_string(),
            "receiver1".to_string(),
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

        let arc_blockchain = Arc::new(Mutex::new(blockchain.clone()));
        let transactions = Arc::new(data.clone());
        let result = blockchain.mine_block(transactions, Arc::clone(&arc_blockchain));

        let blockchain = arc_blockchain.lock().unwrap();

        assert_eq!(result, true);
        assert_eq!(blockchain.get_chain().len(), 2);

        let new_block = &blockchain.get_chain()[1];
        assert_eq!(new_block.get_index(), 1);
        assert_eq!(
            new_block.get_data_raw()[0].get_signature(),
            data[0].get_signature()
        );
        assert_eq!(
            new_block.get_previous_hash(),
            blockchain.get_chain()[0].get_hash()
        );
        assert!(new_block.get_hash().starts_with(&"0".repeat(0)));
        assert_eq!(blockchain.is_chain_valid(), true);
    }

    #[test]
    fn test_transaction_sender_valid() {
        let (prikey1, pubkey1) = generate_key_pair();
        let (_, pubkey2) = generate_key_pair();

        let sender = public_key_to_address(&pubkey1);
        let receiver = public_key_to_address(&pubkey2);
        let amount = 100;
        let nonce = 0;

        let signature = sign_transaction(prikey1, sender.clone(), receiver.clone(), amount, nonce);

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

        assert_eq!(recovered_pubkey, Ok(pubkey1));

        assert_eq!(transaction.verify_signature(), true);
    }

    #[test]
    fn test_merkle_transaction_proof() {
        let tx_list = vec![
            Transaction::new("me".to_string(), "me".to_string(), 10, 0, "coinbase".into()),
            Transaction::new("you".to_string(), "me".to_string(), 5, 0, "transfer".into()),
            Transaction::new("me".to_string(), "you".to_string(), 3, 0, "transfer".into()),
        ];

        let fake_transaction = Transaction::new(
            "fake_sender".to_string(),
            "fake_receiver".to_string(),
            100,
            0,
            "fake_signature".into(),
        );

        let mut blockchain = Blockchain::new();
        let arc_blockchain = Arc::new(Mutex::new(blockchain.clone()));
        let transactions = Arc::new(tx_list.clone());
        blockchain.mine_block(transactions, Arc::clone(&arc_blockchain));

        let mut blockchain = arc_blockchain.lock().unwrap();

        let result1 = blockchain.check_transaction_validity(&tx_list[0]);
        let result2 = blockchain.check_transaction_validity(&tx_list[1]);
        let result3 = blockchain.check_transaction_validity(&tx_list[1]);
        let result4 = blockchain.check_transaction_validity(&fake_transaction);

        assert_eq!(result1, Ok(true));
        assert_eq!(result2, Ok(true));
        assert_eq!(result3, Ok(true));
        assert_ne!(result4, Ok(true))
    }
}
