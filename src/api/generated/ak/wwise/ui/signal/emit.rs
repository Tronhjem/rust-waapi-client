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
    pub fn new(
        handle_id: impl Into<String>,
        widget_id: impl Into<String>,
        signal: impl Into<String>,
    ) -> Self {
        Self {
            handle_id: handle_id.into(),
            widget_id: widget_id.into(),
            signal: signal.into(),
        }
    }
}

impl<T0: Into<String>, T1: Into<String>, T2: Into<String>> From<(T0, T1, T2)> for UiSignalEmitArgs {
    fn from((handle_id, widget_id, signal): (T0, T1, T2)) -> Self {
        Self::new(handle_id, widget_id, signal)
    }
}

/// Module-scoped alias for [`UiSignalEmitArgs`].
pub type EmitArgs = UiSignalEmitArgs;
