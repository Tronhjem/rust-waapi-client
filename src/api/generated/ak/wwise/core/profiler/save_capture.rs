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
    pub fn new(file: String) -> Self {
        Self { file }
    }
}
