use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use tokio::fs;

// this ensures we don't use user's default creds
pub const FERMYON_DEPLOYMENT_ENVIRONMENT: &str = "cloud-e2e-tests";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub url: String,
    pub danger_accept_invalid_certs: bool,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
}

pub async fn read_config() -> Result<Configuration> {
    // we are using cloud-e2e-tests.json to ensure we don't use
    // default personal token for users trying to run the tests
    let path = dirs::config_dir()
        .ok_or_else(|| anyhow!("unable to find config directory"))?
        .join("fermyon")
        .join(format!("{}.json", FERMYON_DEPLOYMENT_ENVIRONMENT));

    let config = serde_json::from_str::<Configuration>(
        &fs::read_to_string(path.clone())
            .await
            .with_context(|| format!("error reading {}", path.display()))?,
    )?;

    Ok(config)
}
