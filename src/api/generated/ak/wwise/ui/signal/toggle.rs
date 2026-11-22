// Generated bindings for topic ak.wwise.ui.signal.toggle

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.ui.signal.toggle`.
///
/// Emitted when a toggle button or a checkbox gets toggled.
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
///
/// Emitted when a toggle button or a checkbox gets toggled.
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
    /// Creates a new [`UiSignalToggleOptions`] with the required fields.
    pub fn new(handle_id: impl Into<String>, widget_id: impl Into<String>) -> Self {
        Self {
            handle_id: handle_id.into(),
            widget_id: widget_id.into(),
        }
    }
}

/// Module-scoped alias for [`UiSignalTogglePayload`].
pub type TogglePayload = UiSignalTogglePayload;

/// Module-scoped alias for [`UiSignalToggleOptions`].
pub type ToggleOptions = UiSignalToggleOptions;
