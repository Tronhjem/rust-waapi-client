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
    pub fn new(files: Vec<String>) -> Self {
        Self { files }
    }
}

impl From<Vec<String>> for CoreSourceControlCheckOutArgs {
    fn from(value: Vec<String>) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.sourceControl.checkOut`.
///
/// Check out files from source control. Equivalent to Check Out for Perforce.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlCheckOutResponse {
    /// Array of log entries generated during the request.
    pub log: Vec<LogCentralItem>,
}

/// Module-scoped alias for [`CoreSourceControlCheckOutArgs`].
pub type CheckOutArgs = CoreSourceControlCheckOutArgs;

/// Module-scoped alias for [`CoreSourceControlCheckOutResponse`].
pub type CheckOutResponse = CoreSourceControlCheckOutResponse;
