
use secp256k1::{SecretKey, Secp256k1, Message, ecdsa::RecoveryId, ecdsa::RecoverableSignature, PublicKey, rand}; 
use sha3::{Digest, Keccak256};


pub fn sign_transaction(key: SecretKey, sender: String, receiver: String, amount: u64, nonce: u64) -> Vec<u8> {
    let message = format!("{}{}{}{}", sender, receiver, amount, nonce);
    let message_hash = Keccak256::digest(message.as_bytes());
    let secp = Secp256k1::new();

    let message_hash = Message::from_digest_slice(&message_hash).expect("Failed to convert message hash");

    let recoverable_sig = secp.sign_ecdsa_recoverable(&message_hash, &key);
    let (rec_id, sig_bytes) = recoverable_sig.serialize_compact();

    let mut signature = sig_bytes.to_vec();
    signature.push(rec_id.to_i32() as u8 + 27); 
    signature
}

pub fn public_key_to_address(public_key: &secp256k1::PublicKey) -> String {
    let serialized_public_key = public_key.serialize_uncompressed();
    let mut hasher = Keccak256::new();
    hasher.update(&serialized_public_key[1..]);
    let hash = hasher.finalize();
    let hash = &hash[12..];
    let mut address = String::from("0x");
    for byte in hash {
        address.push_str(&format!("{:x}", byte));
    }
    address
}

pub fn recover_public_key(
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

pub fn generate_key_pair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
    (secret_key, public_key)
}