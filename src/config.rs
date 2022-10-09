use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

const CONFIG_PATH: &str = "/home/rede/rede/rose/config.toml";

#[derive(Deserialize, Debug)]
pub struct EncryptInfo {
    pub key: [u8; 32],
    pub iv: [u8; 16],
}

#[derive(Deserialize, Debug)]
struct Config {
    encrypt_info: EncryptInfo,
}

impl EncryptInfo {
    pub fn new() -> Self {
        let mut file = match File::open(CONFIG_PATH) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", CONFIG_PATH, e),
        };

        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e),
        };
        let config: Config = toml::from_str(&str_val).unwrap();
        config.encrypt_info
    }
}
