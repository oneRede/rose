use crate::config::ServerConfig;
use anyhow::{bail, Result};
use quinn::ServerConfig as QuinnServerConfig;
use rustls;
use std::{fs, sync::Arc};

pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];
pub struct Server<'a> {
    pub server_config: &'a ServerConfig,
}

impl<'a> Server<'a> {
    fn new(server_config: &'a ServerConfig) -> Self {
        Self {
            server_config: server_config,
        }
    }

    fn run(&self) -> Result<()> {
        let (cert, key) = match fs::read(&self.server_config.cert)
            .and_then(|x| Ok((x, fs::read(&self.server_config.key)?)))
        {
            Ok(x) => x,
            Err(e) => {
                bail!("failed to read certificate: {}", e)
            }
        };
        let key = rustls::PrivateKey(key);
        let cert = rustls::Certificate(cert);
        let (certs, key) = (vec![cert], key);
        let mut server_crypto = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)?;
        server_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();
        if self.server_config.key_log {
            server_crypto.key_log = Arc::new(rustls::KeyLogFile::new());
        }
        let mut qsg = QuinnServerConfig::with_crypto(Arc::new(server_crypto));
        Arc::get_mut(&mut qsg.transport)
            .unwrap()
            .max_concurrent_uni_streams(0_u8.into());
        if self.server_config.stateless_retry {
            qsg.use_retry(true);
        }

        let (endpoint, mut incoming) = quinn::Endpoint::server(qsg, self.server_config.listen.parse().unwrap())?;
        Ok(())
    }
}
