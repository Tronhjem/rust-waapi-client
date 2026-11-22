// Generated bindings for ak.wwise.ui.layout.getLayoutNames

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.ui.layout.getLayoutNames`.
///
/// Retrieves a list of the factory layout names.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiLayoutGetLayoutNamesResponse {
    /// An array of the factory layout names.
    #[serde(default)]
    pub layouts: Vec<String>,
}

/// Module-scoped alias for [`UiLayoutGetLayoutNamesResponse`].
pub type GetLayoutNamesResponse = UiLayoutGetLayoutNamesResponse;
