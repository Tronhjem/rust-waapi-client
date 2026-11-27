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
    #[serde(default)]
    pub status: Option<String>,
    /// Indicates if the Wwise Authoring application is connected to a Wwise Sound Engine process.
    #[serde(rename = "isConnected", default)]
    pub is_connected: Option<bool>,
    /// Describe the remote process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub console: Option<ConnectedConsoleInfo>,
}
