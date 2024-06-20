

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub struct OutboundHttpOpts {
    pub hello: String,
}