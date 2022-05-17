use blake3;
use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

pub fn generate_keypair() -> (EphemeralSecret, PublicKey) {
    let secret_key = EphemeralSecret::new(OsRng);
    let public_key = PublicKey::from(&secret_key);
    (secret_key, public_key)
}

pub fn generate_shared_secret(
    secret_key: EphemeralSecret,
    receiver_public_key: PublicKey,
) -> SharedSecret {
    secret_key.diffie_hellman(&receiver_public_key)
}

pub fn blake3_hash(secret_key_bytes: &[u8]) -> blake3::Hash {
    blake3::hash(secret_key_bytes)
}
