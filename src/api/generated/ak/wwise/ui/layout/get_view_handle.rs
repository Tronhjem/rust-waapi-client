// Generated bindings for ak.wwise.ui.layout.getViewHandle

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.getViewHandle`.
///
/// Gets the frontend handle of an instantiated view.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutGetViewHandleArgs {
    /// The ID of the view.
    #[serde(rename = "viewID")]
    pub view_id: String,
}

impl UiLayoutGetViewHandleArgs {
    /// Creates a new [`UiLayoutGetViewHandleArgs`] with the required fields.
    pub fn new(view_id: impl Into<String>) -> Self {
        Self {
            view_id: view_id.into(),
        }
    }
}

impl From<String> for UiLayoutGetViewHandleArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for UiLayoutGetViewHandleArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for UiLayoutGetViewHandleArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`UiLayoutGetViewHandleArgs`].
pub type GetViewHandleArgs = UiLayoutGetViewHandleArgs;
