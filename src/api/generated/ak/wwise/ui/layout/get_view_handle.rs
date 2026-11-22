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
    pub fn new(view_id: String) -> Self {
        Self { view_id }
    }
}
