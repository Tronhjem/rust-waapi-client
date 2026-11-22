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
    pub fn new(window_id: impl Into<String>) -> Self {
        Self {
            window_id: window_id.into(),
        }
    }
}

impl From<String> for UiWindowPresentArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for UiWindowPresentArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for UiWindowPresentArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`UiWindowPresentArgs`].
pub type PresentArgs = UiWindowPresentArgs;
