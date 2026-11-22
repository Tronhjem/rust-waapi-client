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
    pub fn new(files: Vec<String>, message: impl Into<String>) -> Self {
        Self {
            files,
            message: message.into(),
        }
    }
}

impl<T0: Into<Vec<String>>, T1: Into<String>> From<(T0, T1)> for CoreSourceControlCommitArgs {
    fn from((files, message): (T0, T1)) -> Self {
        Self::new(files.into(), message)
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

/// Module-scoped alias for [`CoreSourceControlCommitArgs`].
pub type CommitArgs = CoreSourceControlCommitArgs;

/// Module-scoped alias for [`CoreSourceControlCommitResponse`].
pub type CommitResponse = CoreSourceControlCommitResponse;
