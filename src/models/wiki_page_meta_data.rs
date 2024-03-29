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

/// WikiPageMetaData : WikiPageMetaData wiki page meta information
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WikiPageMetaData {
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "last_commit", skip_serializing_if = "Option::is_none")]
    pub last_commit: Option<Box<models::WikiCommit>>,
    #[serde(rename = "sub_url", skip_serializing_if = "Option::is_none")]
    pub sub_url: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl WikiPageMetaData {
    /// WikiPageMetaData wiki page meta information
    pub fn new() -> WikiPageMetaData {
        WikiPageMetaData {
            html_url: None,
            last_commit: None,
            sub_url: None,
            title: None,
        }
    }
}
