// Generated bindings for ak.wwise.core.profiler.getCursorTime

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.getCursorTime`.
///
/// Returns the current time of the specified profiler cursor, in milliseconds.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerGetCursorTimeArgs {
    /// Time Cursor from which to acquire the time. The User Time Cursor is the one that can be manipulated by the user, while the Capture Time Cursor represents the latest time of the current capture.
    pub cursor: String,
}

impl CoreProfilerGetCursorTimeArgs {
    /// Creates a new [`CoreProfilerGetCursorTimeArgs`] with the required fields.
    pub fn new(cursor: impl Into<String>) -> Self {
        Self {
            cursor: cursor.into(),
        }
    }
}

impl From<String> for CoreProfilerGetCursorTimeArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreProfilerGetCursorTimeArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreProfilerGetCursorTimeArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.profiler.getCursorTime`.
///
/// Returns the current time of the specified profiler cursor, in milliseconds.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerGetCursorTimeResponse {
    /// The current position of the specified Time Cursor, in ms.
    #[serde(rename = "return")]
    pub return_: i64,
}

/// Module-scoped alias for [`CoreProfilerGetCursorTimeArgs`].
pub type GetCursorTimeArgs = CoreProfilerGetCursorTimeArgs;

/// Module-scoped alias for [`CoreProfilerGetCursorTimeResponse`].
pub type GetCursorTimeResponse = CoreProfilerGetCursorTimeResponse;
