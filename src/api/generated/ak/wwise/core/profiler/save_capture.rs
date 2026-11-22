// Generated bindings for ak.wwise.core.profiler.saveCapture

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.saveCapture`.
///
/// Saves profiler as a.prof file according to the given file path.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerSaveCaptureArgs {
    /// File path to a.prof file.
    pub file: String,
}

impl CoreProfilerSaveCaptureArgs {
    /// Creates a new [`CoreProfilerSaveCaptureArgs`] with the required fields.
    pub fn new(file: impl Into<String>) -> Self {
        Self { file: file.into() }
    }
}

impl From<String> for CoreProfilerSaveCaptureArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreProfilerSaveCaptureArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreProfilerSaveCaptureArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`CoreProfilerSaveCaptureArgs`].
pub type SaveCaptureArgs = CoreProfilerSaveCaptureArgs;
