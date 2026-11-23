// Generated bindings for ak.wwise.core.sourceControl.commit

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sourceControl.commit`.
///
/// Commit files to source control. Equivalent to Submit Changes for Perforce.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSourceControlCommitArgs {
    /// Array of files to commit. File paths must be absolute.
    pub files: Vec<String>,
    /// Description message for the commit.
    pub message: String,
}

impl CoreSourceControlCommitArgs {
    /// Creates a new [`CoreSourceControlCommitArgs`] with the required fields.
    pub fn new(files: impl Into<Vec<String>>, message: impl Into<String>) -> Self {
        Self {
            files: files.into(),
            message: message.into(),
        }
    }
}

/// Response from `ak.wwise.core.sourceControl.commit`.
///
/// Commit files to source control. Equivalent to Submit Changes for Perforce.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlCommitResponse {
    /// Array of log entries generated during the request.
    pub log: Vec<LogCentralItem>,
}
