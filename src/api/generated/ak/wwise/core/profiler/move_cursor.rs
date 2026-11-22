// Generated bindings for ak.wwise.core.profiler.moveCursor

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.moveCursor`.
///
/// Moves the user time cursor of the profiler to a specific position (first, last, next, previous), and returns the new time. This sets the cursor position in the profiler views.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerMoveCursorArgs {
    /// The position to move the cursor to.
    pub position: String,
}

impl CoreProfilerMoveCursorArgs {
    /// Creates a new [`CoreProfilerMoveCursorArgs`] with the required fields.
    pub fn new(position: impl Into<String>) -> Self {
        Self {
            position: position.into(),
        }
    }
}

impl From<String> for CoreProfilerMoveCursorArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreProfilerMoveCursorArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreProfilerMoveCursorArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.profiler.moveCursor`.
///
/// Moves the user time cursor of the profiler to a specific position (first, last, next, previous), and returns the new time. This sets the cursor position in the profiler views.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreProfilerMoveCursorResponse {
    /// The new position of the user Time Cursor, in ms.
    #[serde(rename = "return")]
    pub return_: i64,
}

/// Module-scoped alias for [`CoreProfilerMoveCursorArgs`].
pub type MoveCursorArgs = CoreProfilerMoveCursorArgs;

/// Module-scoped alias for [`CoreProfilerMoveCursorResponse`].
pub type MoveCursorResponse = CoreProfilerMoveCursorResponse;
