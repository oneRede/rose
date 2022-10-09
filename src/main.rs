mod config;
mod encrypt;

use crate::config::EncryptInfo;
use crate::encrypt::{decrypt, encrypt};

fn main() {
    let message = "Hello World!";

    let encrypt_info = EncryptInfo::new();
    let key: [u8; 32] = encrypt_info.key;
    let iv: [u8; 16] = encrypt_info.iv;

    let encrypted_data = encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
    let decrypted_data = decrypt(&encrypted_data[..], &key, &iv).ok().unwrap();
    assert!(message.as_bytes() == &decrypted_data[..]);
}