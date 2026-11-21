// Generated bindings for ak.wwise.ui.commands.getCommands

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Response from `ak.wwise.ui.commands.getCommands`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiCommandsGetCommandsResponse {
    /// An array of command ids.
    #[serde(default)]
    pub commands: Vec<String>,
}
