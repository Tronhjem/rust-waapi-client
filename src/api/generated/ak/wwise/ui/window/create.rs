// Generated bindings for ak.wwise.ui.window.create

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.window.create`.
///
/// Creates a window in which a frontend handle can be instantiated.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiWindowCreateArgs {
    /// The size of the window.
    pub size: serde_json::Value,
    /// The name of the window.
    pub title: String,
}

impl UiWindowCreateArgs {
    /// Creates a new [`UiWindowCreateArgs`] with the required fields.
    pub fn new(title: impl Into<String>, size: serde_json::Value) -> Self {
        Self {
            title: title.into(),
            size,
        }
    }
}

/// Response from `ak.wwise.ui.window.create`.
///
/// Creates a window in which a frontend handle can be instantiated.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiWindowCreateResponse {
    /// The ID of the window.
    #[serde(rename = "windowID", default)]
    pub window_id: Option<String>,
}
