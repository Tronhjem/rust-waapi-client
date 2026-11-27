// Generated bindings for ak.wwise.ui.commands.register

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.commands.register`.
///
/// Registers an array of add-on commands. Registered commands remain until the Wwise process is terminated. See the Wwise custom commands documentation for more information about registering commands. Also see `ak.wwise.ui.commands.executed`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiCommandsRegisterArgs {
    /// Data for the commands to be registered.
    pub commands: Vec<AddOnCommand>,
}

impl UiCommandsRegisterArgs {
    /// Creates a new [`UiCommandsRegisterArgs`] with the required fields.
    pub fn new(commands: impl Into<Vec<AddOnCommand>>) -> Self {
        Self {
            commands: commands.into(),
        }
    }
}
