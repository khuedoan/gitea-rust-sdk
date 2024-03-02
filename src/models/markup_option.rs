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

/// MarkupOption : MarkupOption markup options
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkupOption {
    /// Context to render  in: body
    #[serde(rename = "Context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// File path for detecting extension in file mode  in: body
    #[serde(rename = "FilePath", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    /// Mode to render (comment, gfm, markdown, file)  in: body
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Text markup to render  in: body
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Is it a wiki page ?  in: body
    #[serde(rename = "Wiki", skip_serializing_if = "Option::is_none")]
    pub wiki: Option<bool>,
}

impl MarkupOption {
    /// MarkupOption markup options
    pub fn new() -> MarkupOption {
        MarkupOption {
            context: None,
            file_path: None,
            mode: None,
            text: None,
            wiki: None,
        }
    }
}
