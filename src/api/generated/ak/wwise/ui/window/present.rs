// Generated bindings for ak.wwise.ui.window.present

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.window.present`.
///
/// Presents a window and realizes its widgets.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiWindowPresentArgs {
    /// The ID of the window.
    #[serde(rename = "windowID")]
    pub window_id: String,
}

impl UiWindowPresentArgs {
    /// Creates a new [`UiWindowPresentArgs`] with the required fields.
    pub fn new(window_id: String) -> Self {
        Self { window_id }
    }
}
