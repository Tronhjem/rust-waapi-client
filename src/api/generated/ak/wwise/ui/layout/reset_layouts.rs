// Generated bindings for ak.wwise.ui.layout.resetLayouts

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.layout.resetLayouts`.
///
/// Reset layouts to their default state.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct UiLayoutResetLayoutsArgs {
    /// An array of names of the layout to reset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layouts: Option<Vec<String>>,
}

impl UiLayoutResetLayoutsArgs {
    /// Creates a new [`UiLayoutResetLayoutsArgs`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// An array of names of the layout to reset.
    pub fn layouts(mut self, layouts: impl Into<Vec<String>>) -> Self {
        self.layouts = Some(layouts.into());
        self
    }
}

/// Module-scoped alias for [`UiLayoutResetLayoutsArgs`].
pub type ResetLayoutsArgs = UiLayoutResetLayoutsArgs;
