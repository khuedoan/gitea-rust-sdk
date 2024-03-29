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

/// TopicName : TopicName a list of repo topic names
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicName {
    #[serde(rename = "topics", skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
}

impl TopicName {
    /// TopicName a list of repo topic names
    pub fn new() -> TopicName {
        TopicName { topics: None }
    }
}
