mod config;
mod encrypt;
mod server;
mod client;

use crate::config::Config;
use crate::encrypt::{decrypt, encrypt};

fn main() {
    let message = "Hello World!";
    
    let config = Config::new();
    let encrypt_info = config.get_encrypt_info();
    let key: [u8; 32] = encrypt_info.key;
    let iv: [u8; 16] = encrypt_info.iv;

    let encrypted_data = encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
    let decrypted_data = decrypt(&encrypted_data[..], &key, &iv).ok().unwrap();
    assert!(message.as_bytes() == &decrypted_data[..]);
}
