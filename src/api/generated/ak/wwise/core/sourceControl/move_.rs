// Generated bindings for ak.wwise.core.sourceControl.move

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sourceControl.move`.
///
/// Move or rename files in source control. Always pass the same number of elements in files and newFiles. Equivalent to Move for Perforce.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSourceControlMoveArgs {
    /// Array of files to rename or move. File paths must be absolute.
    pub files: Vec<String>,
    /// Array of new files. File paths must be absolute.
    #[serde(rename = "newFiles")]
    pub new_files: Vec<String>,
}

impl CoreSourceControlMoveArgs {
    /// Creates a new [`CoreSourceControlMoveArgs`] with the required fields.
    pub fn new(files: Vec<String>, new_files: Vec<String>) -> Self {
        Self { files, new_files }
    }
}

/// Response from `ak.wwise.core.sourceControl.move`.
///
/// Move or rename files in source control. Always pass the same number of elements in files and newFiles. Equivalent to Move for Perforce.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlMoveResponse {
    /// Array of log entries generated during the request.
    pub log: Vec<LogCentralItem>,
}
