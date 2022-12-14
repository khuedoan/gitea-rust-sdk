/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.18.0+dev-283-gefaa9958b
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateAccessTokenOption : CreateAccessTokenOption options when create access token

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateAccessTokenOption {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateAccessTokenOption {
    /// CreateAccessTokenOption options when create access token
    pub fn new() -> CreateAccessTokenOption {
        CreateAccessTokenOption { name: None }
    }
}
