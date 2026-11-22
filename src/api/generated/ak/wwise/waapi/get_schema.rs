// Generated bindings for ak.wwise.waapi.getSchema

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.waapi.getSchema`.
///
/// Retrieves the JSON schema of a Waapi Function or Topic URI.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WaapiGetSchemaArgs {
    /// Function or Topic URI. See `ak.wwise.waapi.getFunctions` and `ak.wwise.waapi.getTopics` to get the lists of functions and topics.
    pub uri: String,
    /// Include the examples provided in the schema. Default value is false.
    #[serde(rename = "includeExamples", skip_serializing_if = "Option::is_none")]
    pub include_examples: Option<bool>,
}

impl WaapiGetSchemaArgs {
    /// Creates a new [`WaapiGetSchemaArgs`] with the required fields.
    pub fn new(uri: String) -> Self {
        Self {
            uri,
            include_examples: None,
        }
    }

    /// Include the examples provided in the schema. Default value is false.
    pub fn include_examples(mut self, include_examples: impl Into<bool>) -> Self {
        self.include_examples = Some(include_examples.into());
        self
    }
}

/// Response from `ak.wwise.waapi.getSchema`.
///
/// Retrieves the JSON schema of a Waapi Function or Topic URI.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WaapiGetSchemaResponse {
    /// The argument schema. Only possible for functions.
    #[serde(rename = "argsSchema", default)]
    pub args_schema: Option<serde_json::Value>,
    /// The option schema.
    #[serde(rename = "optionsSchema", default)]
    pub options_schema: Option<serde_json::Value>,
    /// The result schema. Only possible for functions.
    #[serde(rename = "resultSchema", default)]
    pub result_schema: Option<serde_json::Value>,
    /// The publish schema. Only possible for topics.
    #[serde(rename = "publishSchema", default)]
    pub publish_schema: Option<serde_json::Value>,
    /// The description of the function or topic.
    #[serde(default)]
    pub description: Option<String>,
    /// The examples provided in the schema.
    #[serde(default)]
    pub examples: Vec<serde_json::Value>,
}
