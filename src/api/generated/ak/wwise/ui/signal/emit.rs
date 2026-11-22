// Generated bindings for ak.wwise.ui.signal.emit

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.signal.emit`.
///
/// Emits a signal on a widget.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiSignalEmitArgs {
    /// The ID of the handle.
    #[serde(rename = "handleID")]
    pub handle_id: String,
    /// The ID of the widget.
    #[serde(rename = "widgetID")]
    pub widget_id: String,
    /// The topic URI of the signal.
    pub signal: String,
}

impl UiSignalEmitArgs {
    /// Creates a new [`UiSignalEmitArgs`] with the required fields.
    pub fn new(handle_id: String, widget_id: String, signal: String) -> Self {
        Self {
            handle_id,
            widget_id,
            signal,
        }
    }
}
