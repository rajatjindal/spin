

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub struct OutboundHttpOpts {
    pub host: String,
    pub custom_root_ca_file: Option<String>,
    pub client_cert_auth: Option<ClientCertAuth>,
}

#[derive(Debug, serde::Deserialize)]
/// Configuration for client cert auth.
pub struct ClientCertAuth {
    /// The auth cert chain to use for client-auth
    pub cert_chain_file: String,
    /// The private key to use for client-auth
    pub private_key_file: String,
}
