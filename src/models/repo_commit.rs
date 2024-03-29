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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepoCommit {
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::CommitUser>>,
    #[serde(rename = "committer", skip_serializing_if = "Option::is_none")]
    pub committer: Option<Box<models::CommitUser>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "tree", skip_serializing_if = "Option::is_none")]
    pub tree: Option<Box<models::CommitMeta>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "verification", skip_serializing_if = "Option::is_none")]
    pub verification: Option<Box<models::PayloadCommitVerification>>,
}

impl RepoCommit {
    pub fn new() -> RepoCommit {
        RepoCommit {
            author: None,
            committer: None,
            message: None,
            tree: None,
            url: None,
            verification: None,
        }
    }
}
