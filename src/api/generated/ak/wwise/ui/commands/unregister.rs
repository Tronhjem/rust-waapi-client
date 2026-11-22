// Generated bindings for ak.wwise.ui.commands.unregister

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.commands.unregister`.
///
/// Unregisters an array of add-on UI commands.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiCommandsUnregisterArgs {
    /// An array of commands to unregister.
    pub commands: Vec<String>,
}

impl UiCommandsUnregisterArgs {
    /// Creates a new [`UiCommandsUnregisterArgs`] with the required fields.
    pub fn new(commands: Vec<String>) -> Self {
        Self { commands }
    }
}

impl From<Vec<String>> for UiCommandsUnregisterArgs {
    fn from(value: Vec<String>) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`UiCommandsUnregisterArgs`].
pub type UnregisterArgs = UiCommandsUnregisterArgs;
