use std::{fs, path::PathBuf, sync::Arc};

use crate::config::ClientConfig;
use anyhow::{anyhow, Result};

pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];
struct Client<'a> {
    client_config: &'a ClientConfig,
}

impl<'a> Client<'a> {
    fn new(client_config: &'a ClientConfig) -> Self {
        Self { client_config }
    }

    async fn run(&self) -> Result<()> {
        let mut roots = rustls::RootCertStore::empty();
        let ca_path: PathBuf = self.client_config.cert.parse().unwrap();
        roots.add(&rustls::Certificate(fs::read(&ca_path)?))?;
        let mut client_crypto = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(roots)
            .with_no_client_auth();
        client_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();
        if self.client_config.key_log {
            client_crypto.key_log = Arc::new(rustls::KeyLogFile::new());
        }

        let mut endpoint = quinn::Endpoint::client("[::]:0".parse().unwrap())?;
        endpoint.set_default_client_config(quinn::ClientConfig::new(Arc::new(client_crypto)));

        let conn = endpoint
            .connect((&self.client_config.host).parse().unwrap(), &self.client_config.host)?
            .await
            .map_err(|e| anyhow!("failed to connect: {}", e))?;
        
        let (mut send, recv) = conn.connection
        .open_bi()
        .await
        .map_err(|e| anyhow!("failed to open stream: {}", e))?;
        Ok(())
    }
}
