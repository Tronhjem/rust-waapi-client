// Generated bindings for ak.wwise.core.sourceControl.revert

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sourceControl.revert`.
///
/// Revert changes to files in source control.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSourceControlRevertArgs {
    /// Single file or array of files to revert. File paths must be absolute.
    pub files: Vec<String>,
}

impl CoreSourceControlRevertArgs {
    /// Creates a new [`CoreSourceControlRevertArgs`] with the required fields.
    pub fn new(files: Vec<String>) -> Self {
        Self { files }
    }
}

/// Response from `ak.wwise.core.sourceControl.revert`.
///
/// Revert changes to files in source control.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlRevertResponse {
    /// Array of log entries generated during the request.
    pub log: Vec<LogCentralItem>,
}
