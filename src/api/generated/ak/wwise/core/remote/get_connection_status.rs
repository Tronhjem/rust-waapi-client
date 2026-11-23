// Generated bindings for ak.wwise.core.remote.getConnectionStatus

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.core.remote.getConnectionStatus`.
///
/// Retrieves the connection status.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreRemoteGetConnectionStatusResponse {
    /// The current connection status in text.
    pub status: String,
    /// Indicates if the Wwise Authoring application is connected to a Wwise Sound Engine process.
    #[serde(rename = "isConnected")]
    pub is_connected: bool,
    /// Describe the remote process.
    #[serde(default)]
    pub console: Option<serde_json::Value>,
}
