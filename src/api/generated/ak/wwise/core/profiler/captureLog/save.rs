// Generated bindings for ak.wwise.core.profiler.captureLog.save

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.captureLog.save`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerCaptureLogSaveArgs {
    /// File path to a .txt file.
    pub file: String,
}

impl CoreProfilerCaptureLogSaveArgs {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}
