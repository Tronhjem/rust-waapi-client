// Generated bindings for ak.wwise.debug.validateCall

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.debug.validateCall`.
///
/// Validate the arguments, options and result of a WAAPI function call. Does not actually call the function. This is only available with Debug builds.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DebugValidateCallArgs {
    /// The ID of the function.
    pub id: String,
    /// The result returned from the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// The arguments passed to the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
    /// The options passed to the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

impl DebugValidateCallArgs {
    /// Creates a new [`DebugValidateCallArgs`] with the required fields.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            result: None,
            args: None,
            options: None,
        }
    }

    /// The result returned from the function.
    pub fn result(mut self, result: serde_json::Value) -> Self {
        self.result = Some(result);
        self
    }

    /// The arguments passed to the function.
    pub fn args(mut self, args: serde_json::Value) -> Self {
        self.args = Some(args);
        self
    }

    /// The options passed to the function.
    pub fn options(mut self, options: serde_json::Value) -> Self {
        self.options = Some(options);
        self
    }
}
