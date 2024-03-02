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

/// Milestone : Milestone milestone is a collection of issues on one repository
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Milestone {
    #[serde(rename = "closed_at", skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    #[serde(rename = "closed_issues", skip_serializing_if = "Option::is_none")]
    pub closed_issues: Option<i64>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "due_on", skip_serializing_if = "Option::is_none")]
    pub due_on: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "open_issues", skip_serializing_if = "Option::is_none")]
    pub open_issues: Option<i64>,
    /// StateType issue state type
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl Milestone {
    /// Milestone milestone is a collection of issues on one repository
    pub fn new() -> Milestone {
        Milestone {
            closed_at: None,
            closed_issues: None,
            created_at: None,
            description: None,
            due_on: None,
            id: None,
            open_issues: None,
            state: None,
            title: None,
            updated_at: None,
        }
    }
}
