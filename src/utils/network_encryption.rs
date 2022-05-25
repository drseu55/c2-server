use blake3;
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use rand::rngs::OsRng;
use rand::RngCore;
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};

pub fn generate_keypair() -> (StaticSecret, PublicKey) {
    let secret_key = StaticSecret::new(OsRng);
    let public_key = PublicKey::from(&secret_key);
    (secret_key, public_key)
}

pub fn generate_shared_secret(
    secret_key: StaticSecret,
    receiver_public_key: PublicKey,
) -> SharedSecret {
    secret_key.diffie_hellman(&receiver_public_key)
}

pub fn blake3_hash_key(secret_key_bytes: &[u8]) -> blake3::Hash {
    blake3::hash(secret_key_bytes)
}

pub fn xchacha20poly1305_encrypt_message(
    blake3_hashed_key: blake3::Hash,
    message: &[u8],
) -> (Vec<u8>, [u8; 24]) {
    let key = Key::from_slice(blake3_hashed_key.as_bytes());
    let aead = XChaCha20Poly1305::new(key);

    // Generate random nonce
    let mut rng = OsRng::default();
    let mut nonce = [0u8; 24];
    rng.fill_bytes(&mut nonce);

    let xnonce = XNonce::from_slice(&nonce);

    let ciphertext = aead.encrypt(xnonce, message).expect("Encryption failure");

    (ciphertext, nonce)
}
