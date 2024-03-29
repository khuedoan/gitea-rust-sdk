/*
 * Gitea API
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.22.0+dev-892-g9de5e39e2
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// WatchInfo : WatchInfo represents an API watch status of one repository
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchInfo {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "ignored", skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<serde_json::Value>,
    #[serde(rename = "repository_url", skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[serde(rename = "subscribed", skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl WatchInfo {
    /// WatchInfo represents an API watch status of one repository
    pub fn new() -> WatchInfo {
        WatchInfo {
            created_at: None,
            ignored: None,
            reason: None,
            repository_url: None,
            subscribed: None,
            url: None,
        }
    }
}
