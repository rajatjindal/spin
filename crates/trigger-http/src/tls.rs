use anyhow::Context;
use rustls_pemfile::private_key;
use std::{
    fs, io,
    io::Cursor,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio_rustls::{rustls, TlsAcceptor};

/// TLS configuration for the server.
#[derive(Clone)]
pub struct TlsConfig {
    /// Path to TLS certificate.
    pub cert_path: PathBuf,
    /// Path to TLS key.
    pub key_path: PathBuf,
}

impl TlsConfig {
    // Creates a TLS acceptor from server config.
    pub(super) fn server_config(&self) -> anyhow::Result<TlsAcceptor> {
        let certs = load_certs(&self.cert_path)?;
        let private_key = load_keys(&self.key_path)?;

        let cfg = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, private_key)
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        Ok(Arc::new(cfg).into())
    }
}

// load_certs parse and return the certs from the provided file
pub fn load_certs(
    path: impl AsRef<Path>,
) -> anyhow::Result<Vec<rustls_pki_types::CertificateDer<'static>>> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut custom_root_ca_cursor = Cursor::new(contents);

    Ok(rustls_pemfile::certs(&mut custom_root_ca_cursor)
        .into_iter()
        .map(|certs| certs.unwrap())
        .collect())
}

// load_keys parse and return the first private key from the provided file
pub fn load_keys(
    path: impl AsRef<Path>,
) -> anyhow::Result<rustls_pki_types::PrivateKeyDer<'static>> {
    private_key(&mut io::BufReader::new(
        fs::File::open(path).context("loading private key")?,
    ))
    .map_err(|_| anyhow::anyhow!("invalid input"))
    .map(|keys| keys.unwrap())
}
