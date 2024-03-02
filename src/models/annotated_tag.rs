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

/// AnnotatedTag : AnnotatedTag represents an annotated tag
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnotatedTag {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<models::AnnotatedTagObject>>,
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "tagger", skip_serializing_if = "Option::is_none")]
    pub tagger: Option<Box<models::CommitUser>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "verification", skip_serializing_if = "Option::is_none")]
    pub verification: Option<Box<models::PayloadCommitVerification>>,
}

impl AnnotatedTag {
    /// AnnotatedTag represents an annotated tag
    pub fn new() -> AnnotatedTag {
        AnnotatedTag {
            message: None,
            object: None,
            sha: None,
            tag: None,
            tagger: None,
            url: None,
            verification: None,
        }
    }
}
