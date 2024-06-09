use pqcrypto::kem::kyber512::{keypair, encapsulate, decapsulate, PublicKey, SecretKey, Ciphertext, SharedSecret};

pub struct KeyPair {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

pub fn generate_keypair() -> KeyPair {
    let (public_key, secret_key) = keypair();
    KeyPair { public_key, secret_key }
}

pub fn encapsulate_key(public_key: &PublicKey) -> (SharedSecret, Ciphertext) {
    encapsulate(public_key)
}

pub fn decapsulate_key(ciphertext: &Ciphertext, secret_key: &SecretKey) -> SharedSecret {
    decapsulate(ciphertext, secret_key)
}
