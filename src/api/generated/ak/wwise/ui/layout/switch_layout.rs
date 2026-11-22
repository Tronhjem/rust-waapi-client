// Generated bindings for ak.wwise.ui.layout.switchLayout

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.switchLayout`.
///
/// Switches the current layout.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiLayoutSwitchLayoutArgs {
    /// The name of the layout to load.
    pub name: String,
}

impl UiLayoutSwitchLayoutArgs {
    /// Creates a new [`UiLayoutSwitchLayoutArgs`] with the required fields.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
