use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAllAppsResp {
    #[serde(rename = "items")]
    pub items: Vec<AppItem>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Domain {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AppItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "storageId")]
    pub storage_id: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
    #[serde(rename = "subdomain")]
    pub subdomain: String,
}