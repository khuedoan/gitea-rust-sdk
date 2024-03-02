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

/// GpgKey : GPGKey a user GPG key to sign commit and tag in repository
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpgKey {
    #[serde(rename = "can_certify", skip_serializing_if = "Option::is_none")]
    pub can_certify: Option<bool>,
    #[serde(rename = "can_encrypt_comms", skip_serializing_if = "Option::is_none")]
    pub can_encrypt_comms: Option<bool>,
    #[serde(
        rename = "can_encrypt_storage",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_encrypt_storage: Option<bool>,
    #[serde(rename = "can_sign", skip_serializing_if = "Option::is_none")]
    pub can_sign: Option<bool>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "emails", skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<models::GpgKeyEmail>>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "key_id", skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "primary_key_id", skip_serializing_if = "Option::is_none")]
    pub primary_key_id: Option<String>,
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "subkeys", skip_serializing_if = "Option::is_none")]
    pub subkeys: Option<Vec<models::GpgKey>>,
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl GpgKey {
    /// GPGKey a user GPG key to sign commit and tag in repository
    pub fn new() -> GpgKey {
        GpgKey {
            can_certify: None,
            can_encrypt_comms: None,
            can_encrypt_storage: None,
            can_sign: None,
            created_at: None,
            emails: None,
            expires_at: None,
            id: None,
            key_id: None,
            primary_key_id: None,
            public_key: None,
            subkeys: None,
            verified: None,
        }
    }
}
