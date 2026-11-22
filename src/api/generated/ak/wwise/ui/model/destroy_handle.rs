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

impl From<String> for UiModelDestroyHandleArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for UiModelDestroyHandleArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for UiModelDestroyHandleArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`UiModelDestroyHandleArgs`].
pub type DestroyHandleArgs = UiModelDestroyHandleArgs;
