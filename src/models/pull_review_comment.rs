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

/// PullReviewComment : PullReviewComment represents a comment on a pull request review
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullReviewComment {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "commit_id", skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "diff_hunk", skip_serializing_if = "Option::is_none")]
    pub diff_hunk: Option<String>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "original_commit_id", skip_serializing_if = "Option::is_none")]
    pub original_commit_id: Option<String>,
    #[serde(rename = "original_position", skip_serializing_if = "Option::is_none")]
    pub original_position: Option<i32>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(
        rename = "pull_request_review_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub pull_request_review_id: Option<i64>,
    #[serde(rename = "pull_request_url", skip_serializing_if = "Option::is_none")]
    pub pull_request_url: Option<String>,
    #[serde(rename = "resolver", skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Box<models::User>>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::User>>,
}

impl PullReviewComment {
    /// PullReviewComment represents a comment on a pull request review
    pub fn new() -> PullReviewComment {
        PullReviewComment {
            body: None,
            commit_id: None,
            created_at: None,
            diff_hunk: None,
            html_url: None,
            id: None,
            original_commit_id: None,
            original_position: None,
            path: None,
            position: None,
            pull_request_review_id: None,
            pull_request_url: None,
            resolver: None,
            updated_at: None,
            user: None,
        }
    }
}
