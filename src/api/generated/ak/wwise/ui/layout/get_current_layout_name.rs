// Generated bindings for ak.wwise.ui.layout.getCurrentLayoutName

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.ui.layout.getCurrentLayoutName`.
///
/// Retrieves the current layout name.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiLayoutGetCurrentLayoutNameResponse {
    /// The current layout name, an empty string means no layout.
    #[serde(default)]
    pub name: Option<String>,
}

impl UiLayoutGetCurrentLayoutNameResponse {
    /// Returns the object name (`name`), if present in the response.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// Module-scoped alias for [`UiLayoutGetCurrentLayoutNameResponse`].
pub type GetCurrentLayoutNameResponse = UiLayoutGetCurrentLayoutNameResponse;
