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

/// TrackedTime : TrackedTime worked time for an issue / pr
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedTime {
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "issue", skip_serializing_if = "Option::is_none")]
    pub issue: Option<Box<models::Issue>>,
    /// deprecated (only for backwards compatibility)
    #[serde(rename = "issue_id", skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<i64>,
    /// Time in seconds
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// deprecated (only for backwards compatibility)
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "user_name", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl TrackedTime {
    /// TrackedTime worked time for an issue / pr
    pub fn new() -> TrackedTime {
        TrackedTime {
            created: None,
            id: None,
            issue: None,
            issue_id: None,
            time: None,
            user_id: None,
            user_name: None,
        }
    }
}
