use serde_derive::Deserialize;

use std::{fs::File, io::prelude::*};

const CONFIG_PATH: &str = "/Users/redeone/Git/rose/Config.toml";

#[derive(Deserialize, Debug)]
pub struct EncryptInfo {
    pub key: [u8; 32],
    pub iv: [u8; 16],
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub encrypt_info: EncryptInfo,
    pub sever_config: ServerConfig,
    pub client_config: ClientConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub key_log: bool,
    pub key: String,
    pub cert: String,
    pub stateless_retry: bool,
    pub listen: String,
}

#[derive(Deserialize, Debug)]
pub struct ClientConfig {
    pub key_log: bool,
    pub host: String,
    pub port: u32,
    pub cert: String,
    pub rebind: bool,
}

impl Config {
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
        config
    }

    pub fn get_encrypt_info(&self) -> &EncryptInfo {
        return &self.encrypt_info;
    }

    pub fn get_server_config(&self) -> &ServerConfig {
        return &self.sever_config;
    }

    pub fn get_client_config(&self) -> &ClientConfig {
        return &self.client_config
    }
}
