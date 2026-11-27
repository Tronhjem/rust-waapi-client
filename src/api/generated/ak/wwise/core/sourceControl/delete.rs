// Generated bindings for ak.wwise.core.sourceControl.delete

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sourceControl.delete`.
///
/// Delete files from source control. Equivalent to Mark for Delete for Perforce.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSourceControlDeleteArgs {
    /// Array of files to delete. File paths must be absolute.
    pub files: Vec<String>,
}

impl CoreSourceControlDeleteArgs {
    /// Creates a new [`CoreSourceControlDeleteArgs`] with the required fields.
    pub fn new(files: impl Into<Vec<String>>) -> Self {
        Self {
            files: files.into(),
        }
    }
}

/// Response from `ak.wwise.core.sourceControl.delete`.
///
/// Delete files from source control. Equivalent to Mark for Delete for Perforce.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlDeleteResponse {
    /// Array of log entries generated during the request.
    #[serde(default)]
    pub log: Vec<LogCentralItem>,
}
