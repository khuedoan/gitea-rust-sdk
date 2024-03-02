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

/// AddCollaboratorOption : AddCollaboratorOption options when adding a user as a collaborator of a repository
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddCollaboratorOption {
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

impl AddCollaboratorOption {
    /// AddCollaboratorOption options when adding a user as a collaborator of a repository
    pub fn new() -> AddCollaboratorOption {
        AddCollaboratorOption { permission: None }
    }
}
