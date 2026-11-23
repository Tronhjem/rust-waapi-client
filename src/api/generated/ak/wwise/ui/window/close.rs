// Generated bindings for ak.wwise.ui.window.close

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.window.close`.
///
/// Closes a window and unrealizes its widgets.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiWindowCloseArgs {
    /// The ID of the window.
    #[serde(rename = "windowID")]
    pub window_id: String,
}

impl UiWindowCloseArgs {
    /// Creates a new [`UiWindowCloseArgs`] with the required fields.
    pub fn new(window_id: impl Into<String>) -> Self {
        Self {
            window_id: window_id.into(),
        }
    }
}
