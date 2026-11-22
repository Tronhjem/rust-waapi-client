// Generated bindings for ak.wwise.ui.layout.getViewTypes

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.ui.layout.getViewTypes`.
///
/// Retrieves a list of all view types registered in Wwise.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiLayoutGetViewTypesResponse {
    /// Array of all view types registered in Wwise.
    #[serde(rename = "viewTypes", default)]
    pub view_types: Vec<serde_json::Value>,
}

/// Module-scoped alias for [`UiLayoutGetViewTypesResponse`].
pub type GetViewTypesResponse = UiLayoutGetViewTypesResponse;
