// Generated bindings for ak.wwise.ui.layout.getViewInstances

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.getViewInstances`.
///
/// Retrieves a list of all view instances of a layout.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutGetViewInstancesArgs {
    /// The name of the layout to inspect.
    pub name: String,
}

impl UiLayoutGetViewInstancesArgs {
    /// Creates a new [`UiLayoutGetViewInstancesArgs`] with the required fields.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Response from `ak.wwise.ui.layout.getViewInstances`.
///
/// Retrieves a list of all view instances of a layout.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiLayoutGetViewInstancesResponse {
    /// Array of all view instances of a layout.
    #[serde(rename = "viewInstances", default)]
    pub view_instances: Vec<serde_json::Value>,
}
