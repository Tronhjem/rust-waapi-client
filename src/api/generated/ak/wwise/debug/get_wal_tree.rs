// Generated bindings for ak.wwise.debug.getWalTree

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.debug.getWalTree`.
///
/// Retrieves the WAL tree, which describes the nodes that are synchronized in the Sound Engine. Private use only.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DebugGetWalTreeResponse {
    /// Result returned by the dump of WAL.
    #[serde(rename = "return")]
    pub return_: serde_json::Value,
}
