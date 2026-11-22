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

impl<T0: Into<String>, T1: Into<serde_json::Value>> From<(T0, T1)> for UiWindowCreateArgs {
    fn from((title, size): (T0, T1)) -> Self {
        Self::new(title, size.into())
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

/// Module-scoped alias for [`UiWindowCreateArgs`].
pub type CreateArgs = UiWindowCreateArgs;

/// Module-scoped alias for [`UiWindowCreateResponse`].
pub type CreateResponse = UiWindowCreateResponse;
