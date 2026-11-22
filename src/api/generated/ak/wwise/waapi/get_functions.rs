// Generated bindings for ak.wwise.waapi.getFunctions

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.waapi.getFunctions`.
///
/// Retrieves the list of functions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WaapiGetFunctionsResponse {
    /// An array of Function URIs.
    #[serde(default)]
    pub functions: Vec<String>,
}

/// Module-scoped alias for [`WaapiGetFunctionsResponse`].
pub type GetFunctionsResponse = WaapiGetFunctionsResponse;
