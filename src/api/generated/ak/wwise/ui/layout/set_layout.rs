// Generated bindings for ak.wwise.ui.layout.setLayout

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.setLayout`.
///
/// Registers a new layout from a JSON format.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutSetLayoutArgs {
    /// The name of the layout to register.
    pub name: String,
    /// The layout description in JSON format.
    pub layout: serde_json::Value,
}

impl UiLayoutSetLayoutArgs {
    /// Creates a new [`UiLayoutSetLayoutArgs`] with the required fields.
    pub fn new(name: impl Into<String>, layout: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            layout,
        }
    }
}

impl<T0: Into<String>, T1: Into<serde_json::Value>> From<(T0, T1)> for UiLayoutSetLayoutArgs {
    fn from((name, layout): (T0, T1)) -> Self {
        Self::new(name, layout.into())
    }
}

/// Module-scoped alias for [`UiLayoutSetLayoutArgs`].
pub type SetLayoutArgs = UiLayoutSetLayoutArgs;
