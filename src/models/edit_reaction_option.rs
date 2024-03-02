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

/// EditReactionOption : EditReactionOption contain the reaction type
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditReactionOption {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl EditReactionOption {
    /// EditReactionOption contain the reaction type
    pub fn new() -> EditReactionOption {
        EditReactionOption { content: None }
    }
}
