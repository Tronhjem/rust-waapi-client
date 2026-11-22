// Generated bindings for ak.wwise.ui.model.registerWafm

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.model.registerWafm`.
///
/// Registers WAFM files and inline widget descriptors.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiModelRegisterWafmArgs {
    /// `wafm`.
    pub wafm: serde_json::Value,
}

impl UiModelRegisterWafmArgs {
    /// Creates a new [`UiModelRegisterWafmArgs`] with the required fields.
    pub fn new(wafm: serde_json::Value) -> Self {
        Self { wafm }
    }
}

impl From<serde_json::Value> for UiModelRegisterWafmArgs {
    fn from(value: serde_json::Value) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.ui.model.registerWafm`.
///
/// Registers WAFM files and inline widget descriptors.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiModelRegisterWafmResponse {
    /// Success of the operation.
    #[serde(default)]
    pub success: Option<bool>,
}

/// Module-scoped alias for [`UiModelRegisterWafmArgs`].
pub type RegisterWafmArgs = UiModelRegisterWafmArgs;

/// Module-scoped alias for [`UiModelRegisterWafmResponse`].
pub type RegisterWafmResponse = UiModelRegisterWafmResponse;
