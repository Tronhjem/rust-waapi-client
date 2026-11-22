// Generated bindings for topic ak.wwise.debug.assertFailed

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.debug.assertFailed`.
///
/// Sent when an assert has failed. This is only available in Debug builds.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DebugAssertFailedPayload {
    /// An optional explanatory message accomapnying the assert.
    #[serde(default)]
    pub message: Option<String>,
    /// The name of the source file.
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// The expression that failed.
    pub expression: String,
    /// The line number.
    #[serde(rename = "lineNumber")]
    pub line_number: f64,
    /// The callstack from the location of the assert.
    pub callstack: String,
}
