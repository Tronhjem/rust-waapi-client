// Generated bindings for ak.wwise.core.getInfo

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.core.getInfo`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseCoreGetInfoResponse {
    /// Wwise display name.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Indicates the platform on which Wwise was built.
    pub platform: String,
    /// Wwise version object.
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
    /// Indicates a release or debug build.
    pub configuration: String,
    /// Indicates if Wwise is running in command line.
    #[serde(rename = "isCommandLine")]
    pub is_command_line: bool,
    /// The process identifier of Wwise.
    #[serde(rename = "processId")]
    pub process_id: i64,
    /// The process path of Wwise.
    #[serde(rename = "processPath")]
    pub process_path: String,
    /// Collection of directories used by Wwise.
    pub directories: DirectoriesInfo,
}

pub type WwiseCoreGetinfoResponse = WwiseCoreGetInfoResponse;
