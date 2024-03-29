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

/// IssueMeta : IssueMeta basic issue information
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueMeta {
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "repo", skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
}

impl IssueMeta {
    /// IssueMeta basic issue information
    pub fn new() -> IssueMeta {
        IssueMeta {
            index: None,
            owner: None,
            repo: None,
        }
    }
}
