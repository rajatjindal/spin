use crate::app_item::{AppItem, Domain};
use anyhow::Result;
use std::str;


pub fn get_app_by_name(name: &str) -> Result<AppItem> {
    Ok(AppItem{
        id: "some-id".to_string(),
        name: name.to_string(),
        storage_id: "some-storage".to_string(),
        description: Some("some description".to_string()),
        domain: Some(Domain{
            name: "some-domain".to_string(),
        }),
        subdomain: "some subdomain".to_string(),
    })
}
