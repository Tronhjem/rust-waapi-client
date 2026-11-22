// Generated bindings for topic ak.wwise.ui.signal.click



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Payload for `ak.wwise.ui.signal.click`.
///
/// Emitted when a push button or a radio button gets clicked.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiSignalClickPayload {
    /// The ID of the handle.
    #[serde(rename = "handleID")]
    pub handle_id: String,
    /// The ID of the widget.
    #[serde(rename = "widgetID")]
    pub widget_id: String,
}


/// Subscription options for `ak.wwise.ui.signal.click`.
///
/// Emitted when a push button or a radio button gets clicked.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiSignalClickOptions {
    /// The ID of the handle.
    #[serde(rename = "handleID")]
    pub handle_id: String,
    /// The ID of the widget.
    #[serde(rename = "widgetID")]
    pub widget_id: String,
}

impl UiSignalClickOptions {
    /// Creates a new [`UiSignalClickOptions`] with the required fields.
    pub fn new(handle_id: String, widget_id: String) -> Self {
        Self {
            handle_id,
            widget_id,
        }
    }
}
