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

/// GeneralRepoSettings : GeneralRepoSettings contains global repository settings exposed by API
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralRepoSettings {
    #[serde(rename = "http_git_disabled", skip_serializing_if = "Option::is_none")]
    pub http_git_disabled: Option<bool>,
    #[serde(rename = "lfs_disabled", skip_serializing_if = "Option::is_none")]
    pub lfs_disabled: Option<bool>,
    #[serde(
        rename = "migrations_disabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub migrations_disabled: Option<bool>,
    #[serde(rename = "mirrors_disabled", skip_serializing_if = "Option::is_none")]
    pub mirrors_disabled: Option<bool>,
    #[serde(rename = "stars_disabled", skip_serializing_if = "Option::is_none")]
    pub stars_disabled: Option<bool>,
    #[serde(
        rename = "time_tracking_disabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_tracking_disabled: Option<bool>,
}

impl GeneralRepoSettings {
    /// GeneralRepoSettings contains global repository settings exposed by API
    pub fn new() -> GeneralRepoSettings {
        GeneralRepoSettings {
            http_git_disabled: None,
            lfs_disabled: None,
            migrations_disabled: None,
            mirrors_disabled: None,
            stars_disabled: None,
            time_tracking_disabled: None,
        }
    }
}
