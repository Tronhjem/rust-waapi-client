// Generated bindings for ak.wwise.ui.layout.removeLayout

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.removeLayout`.
///
/// Unregisters a temporary layout, previously registered with `ak.wwise.ui.layout.setLayout`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutRemoveLayoutArgs {
    /// The name of the layout to unregister.
    pub name: String,
}

impl UiLayoutRemoveLayoutArgs {
    /// Creates a new [`UiLayoutRemoveLayoutArgs`] with the required fields.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl From<String> for UiLayoutRemoveLayoutArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for UiLayoutRemoveLayoutArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for UiLayoutRemoveLayoutArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`UiLayoutRemoveLayoutArgs`].
pub type RemoveLayoutArgs = UiLayoutRemoveLayoutArgs;
