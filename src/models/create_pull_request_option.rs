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

/// CreatePullRequestOption : CreatePullRequestOption options when creating a pull request
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePullRequestOption {
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(rename = "assignees", skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
    #[serde(rename = "base", skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "due_date", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "head", skip_serializing_if = "Option::is_none")]
    pub head: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<i64>>,
    #[serde(rename = "milestone", skip_serializing_if = "Option::is_none")]
    pub milestone: Option<i64>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreatePullRequestOption {
    /// CreatePullRequestOption options when creating a pull request
    pub fn new() -> CreatePullRequestOption {
        CreatePullRequestOption {
            assignee: None,
            assignees: None,
            base: None,
            body: None,
            due_date: None,
            head: None,
            labels: None,
            milestone: None,
            title: None,
        }
    }
}
