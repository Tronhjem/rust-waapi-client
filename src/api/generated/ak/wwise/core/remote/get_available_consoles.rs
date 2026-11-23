// Generated bindings for ak.wwise.core.remote.getAvailableConsoles

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.core.remote.getAvailableConsoles`.
///
/// Retrieves all consoles available for connecting Wwise Authoring to a Sound Engine instance.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreRemoteGetAvailableConsolesResponse {
    /// An array of remote consoles available.
    pub consoles: Vec<serde_json::Value>,
}
