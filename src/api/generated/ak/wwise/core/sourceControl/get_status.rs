// Generated bindings for ak.wwise.core.sourceControl.getStatus

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sourceControl.getStatus`.
///
/// Get the source control status of the specified files.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSourceControlGetStatusArgs {
    /// Array of files for which to retrieve source control status. File paths must be absolute.
    pub files: Vec<String>,
}

impl CoreSourceControlGetStatusArgs {
    /// Creates a new [`CoreSourceControlGetStatusArgs`] with the required fields.
    pub fn new(files: Vec<String>) -> Self {
        Self { files }
    }
}

impl From<Vec<String>> for CoreSourceControlGetStatusArgs {
    fn from(value: Vec<String>) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.sourceControl.getStatus`.
///
/// Get the source control status of the specified files.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlGetStatusResponse {
    /// Array of log entries generated during the request.
    pub log: Vec<LogCentralItem>,
    /// Array of source control statuses.
    pub result: Vec<serde_json::Value>,
}

/// Module-scoped alias for [`CoreSourceControlGetStatusArgs`].
pub type GetStatusArgs = CoreSourceControlGetStatusArgs;

/// Module-scoped alias for [`CoreSourceControlGetStatusResponse`].
pub type GetStatusResponse = CoreSourceControlGetStatusResponse;
