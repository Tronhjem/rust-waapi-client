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
    pub fn new(files: Vec<String>) -> Self {
        Self { files }
    }
}

impl From<Vec<String>> for CoreSourceControlDeleteArgs {
    fn from(value: Vec<String>) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.sourceControl.delete`.
///
/// Delete files from source control. Equivalent to Mark for Delete for Perforce.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlDeleteResponse {
    /// Array of log entries generated during the request.
    pub log: Vec<LogCentralItem>,
}

/// Module-scoped alias for [`CoreSourceControlDeleteArgs`].
pub type DeleteArgs = CoreSourceControlDeleteArgs;

/// Module-scoped alias for [`CoreSourceControlDeleteResponse`].
pub type DeleteResponse = CoreSourceControlDeleteResponse;
