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

/// GitTreeResponse : GitTreeResponse returns a git tree
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitTreeResponse {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "tree", skip_serializing_if = "Option::is_none")]
    pub tree: Option<Vec<models::GitEntry>>,
    #[serde(rename = "truncated", skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl GitTreeResponse {
    /// GitTreeResponse returns a git tree
    pub fn new() -> GitTreeResponse {
        GitTreeResponse {
            page: None,
            sha: None,
            total_count: None,
            tree: None,
            truncated: None,
            url: None,
        }
    }
}
