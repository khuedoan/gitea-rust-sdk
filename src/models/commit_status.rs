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

/// CommitStatus : CommitStatus holds a single status of a single Commit
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitStatus {
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<Box<models::User>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// CommitStatusState holds the state of a CommitStatus It can be \"pending\", \"success\", \"error\" and \"failure\"
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "target_url", skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CommitStatus {
    /// CommitStatus holds a single status of a single Commit
    pub fn new() -> CommitStatus {
        CommitStatus {
            context: None,
            created_at: None,
            creator: None,
            description: None,
            id: None,
            status: None,
            target_url: None,
            updated_at: None,
            url: None,
        }
    }
}
