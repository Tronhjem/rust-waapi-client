// Generated bindings for ak.wwise.ui.layout.closeView

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.closeView`.
///
/// Requests to close a view by its unique id. The view might not yet be closed when returning from this function.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutCloseViewArgs {
    /// The unique id of the view to close.
    #[serde(rename = "viewID")]
    pub view_id: String,
}

impl UiLayoutCloseViewArgs {
    /// Creates a new [`UiLayoutCloseViewArgs`] with the required fields.
    pub fn new(view_id: impl Into<String>) -> Self {
        Self {
            view_id: view_id.into(),
        }
    }
}
