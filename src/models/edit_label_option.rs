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

/// EditLabelOption : EditLabelOption options for editing a label
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditLabelOption {
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "exclusive", skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
    #[serde(rename = "is_archived", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl EditLabelOption {
    /// EditLabelOption options for editing a label
    pub fn new() -> EditLabelOption {
        EditLabelOption {
            color: None,
            description: None,
            exclusive: None,
            is_archived: None,
            name: None,
        }
    }
}
