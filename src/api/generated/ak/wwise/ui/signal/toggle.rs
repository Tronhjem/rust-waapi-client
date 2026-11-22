// Generated bindings for topic ak.wwise.ui.signal.toggle

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.ui.signal.toggle`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiSignalTogglePayload {
    /// The ID of the handle.
    #[serde(rename = "handleID")]
    pub handle_id: String,
    /// The ID of the widget.
    #[serde(rename = "widgetID")]
    pub widget_id: String,
}

/// Subscription options for `ak.wwise.ui.signal.toggle`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiSignalToggleOptions {
    /// The ID of the handle.
    #[serde(rename = "handleID")]
    pub handle_id: String,
    /// The ID of the widget.
    #[serde(rename = "widgetID")]
    pub widget_id: String,
}

impl UiSignalToggleOptions {
    pub fn new(handle_id: String, widget_id: String) -> Self {
        Self {
            handle_id,
            widget_id,
        }
    }
}
