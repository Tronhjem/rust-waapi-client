// Generated bindings for ak.wwise.ui.model.destroyHandle

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.model.destroyHandle`.
///
/// Destroys a frontend handle and its widgets.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiModelDestroyHandleArgs {
    /// The ID of the handle.
    #[serde(rename = "handleID")]
    pub handle_id: String,
}

impl UiModelDestroyHandleArgs {
    /// Creates a new [`UiModelDestroyHandleArgs`] with the required fields.
    pub fn new(handle_id: impl Into<String>) -> Self {
        Self {
            handle_id: handle_id.into(),
        }
    }
}
