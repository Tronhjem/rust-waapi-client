// Generated bindings for ak.wwise.core.sourceControl.checkOut

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sourceControl.checkOut`.
///
/// Check out files from source control. Equivalent to Check Out for Perforce.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSourceControlCheckOutArgs {
    /// Array of files to check out. File paths must be absolute.
    pub files: Vec<String>,
}

impl CoreSourceControlCheckOutArgs {
    /// Creates a new [`CoreSourceControlCheckOutArgs`] with the required fields.
    pub fn new(files: impl Into<Vec<String>>) -> Self {
        Self {
            files: files.into(),
        }
    }
}

/// Response from `ak.wwise.core.sourceControl.checkOut`.
///
/// Check out files from source control. Equivalent to Check Out for Perforce.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlCheckOutResponse {
    /// Array of log entries generated during the request.
    #[serde(default)]
    pub log: Vec<LogCentralItem>,
}
