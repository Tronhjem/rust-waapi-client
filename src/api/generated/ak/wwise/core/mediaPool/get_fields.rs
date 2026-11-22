// Generated bindings for ak.wwise.core.mediaPool.getFields

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.core.mediaPool.getFields`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreMediaPoolGetFieldsResponse {
    /// The list of fields available to use in Media Pool queries.
    #[serde(rename = "return", default)]
    pub return_: Vec<String>,
}
