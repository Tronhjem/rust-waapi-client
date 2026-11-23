// Generated bindings for ak.wwise.core.ping

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.core.ping`.
///
/// Verify if WAAPI is currently available.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CorePingResponse {
    /// Indicates if WAAPI is available. Note that WAAPI is not available when a modal dialog is shown.
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
}
