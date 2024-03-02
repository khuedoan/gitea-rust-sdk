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

/// DeleteEmailOption : DeleteEmailOption options when deleting email addresses
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteEmailOption {
    /// email addresses to delete
    #[serde(rename = "emails", skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
}

impl DeleteEmailOption {
    /// DeleteEmailOption options when deleting email addresses
    pub fn new() -> DeleteEmailOption {
        DeleteEmailOption { emails: None }
    }
}
