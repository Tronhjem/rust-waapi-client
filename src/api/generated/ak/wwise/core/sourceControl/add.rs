// Generated bindings for ak.wwise.core.sourceControl.add

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sourceControl.add`.
///
/// Add files to source control. Equivalent to Mark for Add for Perforce.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSourceControlAddArgs {
    /// Array of files to add. File paths must be absolute.
    pub files: Vec<String>,
}

impl CoreSourceControlAddArgs {
    /// Creates a new [`CoreSourceControlAddArgs`] with the required fields.
    pub fn new(files: Vec<String>) -> Self {
        Self { files }
    }
}

impl From<Vec<String>> for CoreSourceControlAddArgs {
    fn from(value: Vec<String>) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.sourceControl.add`.
///
/// Add files to source control. Equivalent to Mark for Add for Perforce.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlAddResponse {
    /// Array of log entries generated during the request.
    pub log: Vec<LogCentralItem>,
}

/// Module-scoped alias for [`CoreSourceControlAddArgs`].
pub type AddArgs = CoreSourceControlAddArgs;

/// Module-scoped alias for [`CoreSourceControlAddResponse`].
pub type AddResponse = CoreSourceControlAddResponse;
