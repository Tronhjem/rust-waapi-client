// Generated bindings for ak.wwise.ui.layout.getLayout

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.getLayout`.
///
/// Serializes a specific layout into a JSON format.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutGetLayoutArgs {
    /// The name of the layout to serialize.
    pub name: String,
}

impl UiLayoutGetLayoutArgs {
    /// Creates a new [`UiLayoutGetLayoutArgs`] with the required fields.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// Response from `ak.wwise.ui.layout.getLayout`.
///
/// Serializes a specific layout into a JSON format.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiLayoutGetLayoutResponse {
    /// The layout description in JSON format.
    #[serde(rename = "Layout", default)]
    pub layout: Option<serde_json::Value>,
}
