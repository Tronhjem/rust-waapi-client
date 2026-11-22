// Generated bindings for ak.wwise.core.mediaPool.getFields



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Response from `ak.wwise.core.mediaPool.getFields`.
///
/// Retrieve all fields present in the Media Pool. You can then use the fields to query the Media Pool. The Media Pool discovers some fields when it scans audio files. Others, such as WAV fields, are always available.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreMediaPoolGetFieldsResponse {
    /// The list of fields available to use in Media Pool queries.
    #[serde(rename = "return", default)]
    pub return_: Vec<String>,
}

