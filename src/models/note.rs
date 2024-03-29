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

/// Note : Note contains information related to a git note
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<Box<models::Commit>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Note {
    /// Note contains information related to a git note
    pub fn new() -> Note {
        Note {
            commit: None,
            message: None,
        }
    }
}
