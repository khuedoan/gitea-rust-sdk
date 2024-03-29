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

/// DeleteFileOptions : DeleteFileOptions options for deleting files (used for other File structs below) Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteFileOptions {
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::Identity>>,
    /// branch (optional) to base this file from. if not given, the default branch is used
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(rename = "committer", skip_serializing_if = "Option::is_none")]
    pub committer: Option<Box<models::Identity>>,
    #[serde(rename = "dates", skip_serializing_if = "Option::is_none")]
    pub dates: Option<Box<models::CommitDateOptions>>,
    /// message (optional) for the commit of this file. if not supplied, a default message will be used
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// new_branch (optional) will make a new branch from `branch` before creating the file
    #[serde(rename = "new_branch", skip_serializing_if = "Option::is_none")]
    pub new_branch: Option<String>,
    /// sha is the SHA for the file that already exists
    #[serde(rename = "sha")]
    pub sha: String,
    /// Add a Signed-off-by trailer by the committer at the end of the commit log message.
    #[serde(rename = "signoff", skip_serializing_if = "Option::is_none")]
    pub signoff: Option<bool>,
}

impl DeleteFileOptions {
    /// DeleteFileOptions options for deleting files (used for other File structs below) Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
    pub fn new(sha: String) -> DeleteFileOptions {
        DeleteFileOptions {
            author: None,
            branch: None,
            committer: None,
            dates: None,
            message: None,
            new_branch: None,
            sha,
            signoff: None,
        }
    }
}
