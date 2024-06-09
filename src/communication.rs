use crate::key_management::{encapsulate_key, decapsulate_key, KeyPair};
use pqcrypto::kem::kyber512::{SharedSecret, Ciphertext};

pub struct Message {
    pub content: String,
    pub shared_secret: SharedSecret,
}

pub fn send_message(receiver_keypair: &KeyPair, content: &str) -> (Message, Ciphertext) {
    let (shared_secret, ciphertext) = encapsulate_key(&receiver_keypair.public_key);
    let message = Message {
        content: content.to_string(),
        shared_secret,
    };
    (message, ciphertext)
}

pub fn receive_message(receiver_keypair: &KeyPair, message: &Message, ciphertext: &Ciphertext) -> bool {
    let shared_secret = decapsulate_key(ciphertext, &receiver_keypair.secret_key);
    shared_secret == message.shared_secret
}
