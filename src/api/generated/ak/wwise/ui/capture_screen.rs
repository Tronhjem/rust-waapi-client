// Generated bindings for ak.wwise.ui.captureScreen

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.captureScreen`.
///
/// Captures a part of the Wwise UI relative to a view.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UiCaptureScreenArgs {
    /// The name of the view. By default, the whole UI is captured.
    #[serde(rename = "viewName", skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    /// The selection channel of the view. Can be a value of 1, 2, 3 or 4. By default, the current selection channel of the view is detected automatically.
    #[serde(
        rename = "viewSelectionChannel",
        skip_serializing_if = "Option::is_none"
    )]
    pub view_selection_channel: Option<i64>,
    /// The capture region. By default, the whole view is captured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rect: Option<serde_json::Value>,
}

impl UiCaptureScreenArgs {
    /// Creates a new [`UiCaptureScreenArgs`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// The name of the view. By default, the whole UI is captured.
    pub fn view_name(mut self, view_name: impl Into<String>) -> Self {
        self.view_name = Some(view_name.into());
        self
    }

    /// The selection channel of the view. Can be a value of 1, 2, 3 or 4. By default, the current selection channel of the view is detected automatically.
    pub fn view_selection_channel(mut self, view_selection_channel: i64) -> Self {
        self.view_selection_channel = Some(view_selection_channel);
        self
    }

    /// The capture region. By default, the whole view is captured.
    pub fn rect(mut self, rect: serde_json::Value) -> Self {
        self.rect = Some(rect);
        self
    }
}

/// Response from `ak.wwise.ui.captureScreen`.
///
/// Captures a part of the Wwise UI relative to a view.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiCaptureScreenResponse {
    /// The underlying image data format (e.g. image/png).
    #[serde(rename = "contentType", default)]
    pub content_type: Option<String>,
    /// The encoded image data (Base64).
    #[serde(rename = "contentBase64", default)]
    pub content_base64: Option<String>,
}
