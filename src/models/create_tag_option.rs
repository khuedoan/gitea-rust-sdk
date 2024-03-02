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

/// CreateTagOption : CreateTagOption options when creating a tag
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTagOption {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl CreateTagOption {
    /// CreateTagOption options when creating a tag
    pub fn new(tag_name: String) -> CreateTagOption {
        CreateTagOption {
            message: None,
            tag_name,
            target: None,
        }
    }
}
