use zssh::{AuthMethod, PublicKey::Ed25519};
use zssh::ed25519_dalek::{VerifyingKey, PUBLIC_KEY_LENGTH};

const FULL_KEY: &[u8] = b"AAAAC3NzaC1lZDI1NTE5AAAAICHAM1KLDKxCvqUmGSNsKjc3/rGue0OHHBkX/NWSQ8n5";

pub fn main() {
    let decoded = &(FULL_KEY)[15..47];  // 32 bytes
    let actual_key: [u8; PUBLIC_KEY_LENGTH] = decoded.try_into().unwrap();
    let auth = AuthMethod::PublicKey(Ed25519 { public_key: VerifyingKey::from_bytes(&actual_key).unwrap() });
}