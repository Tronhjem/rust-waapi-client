// Generated bindings for ak.wwise.core.getInfo

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;


/// Arguments for `ak.wwise.core.getInfo`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct WwiseCoreGetinfoArgs {
}

impl WwiseCoreGetinfoArgs {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Options for `ak.wwise.core.getInfo`.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct WwiseCoreGetinfoOptions {
}

impl WwiseCoreGetinfoOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Response from `ak.wwise.core.getInfo`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseCoreGetinfoResponse {
    /// Wwise display name.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Platform on which Wwise was built.
    pub platform: String,
    /// Wwise version information.
    pub version: VersionInfo,
    /// Wwise session id.
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// Version of the Wwise Authoring API.
    #[serde(rename = "apiVersion")]
    pub api_version: f64,
    /// Branch built.
    pub branch: String,
    /// Copyright information.
    pub copyright: String,
    /// Release or debug build.
    pub configuration: String,
    /// True if Wwise is running in command line.
    #[serde(rename = "isCommandLine")]
    pub is_command_line: bool,
    /// The process identifier of Wwise.
    #[serde(rename = "processId")]
    pub process_id: u32,
    /// The process path of Wwise.
    #[serde(rename = "processPath")]
    pub process_path: String,
    /// Collection of directories used by Wwise.
    pub directories: DirectoriesInfo,
}
