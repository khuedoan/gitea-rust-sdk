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

/// ChangeFileOperation : ChangeFileOperation for creating, updating or deleting a file
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeFileOperation {
    /// new or updated file content, must be base64 encoded
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// old path of the file to move
    #[serde(rename = "from_path", skip_serializing_if = "Option::is_none")]
    pub from_path: Option<String>,
    /// indicates what to do with the file
    #[serde(rename = "operation")]
    pub operation: Operation,
    /// path to the existing or new file
    #[serde(rename = "path")]
    pub path: String,
    /// sha is the SHA for the file that already exists, required for update or delete
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
}

impl ChangeFileOperation {
    /// ChangeFileOperation for creating, updating or deleting a file
    pub fn new(operation: Operation, path: String) -> ChangeFileOperation {
        ChangeFileOperation {
            content: None,
            from_path: None,
            operation,
            path,
            sha: None,
        }
    }
}
/// indicates what to do with the file
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
}

impl Default for Operation {
    fn default() -> Operation {
        Self::Create
    }
}
