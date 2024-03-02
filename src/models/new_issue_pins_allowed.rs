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

/// NewIssuePinsAllowed : NewIssuePinsAllowed represents an API response that says if new Issue Pins are allowed
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewIssuePinsAllowed {
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<bool>,
    #[serde(rename = "pull_requests", skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<bool>,
}

impl NewIssuePinsAllowed {
    /// NewIssuePinsAllowed represents an API response that says if new Issue Pins are allowed
    pub fn new() -> NewIssuePinsAllowed {
        NewIssuePinsAllowed {
            issues: None,
            pull_requests: None,
        }
    }
}
