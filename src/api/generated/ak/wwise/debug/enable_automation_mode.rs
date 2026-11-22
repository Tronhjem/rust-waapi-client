// Generated bindings for ak.wwise.debug.enableAutomationMode

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.debug.enableAutomationMode`.
///
/// Enables or disables the automation mode for Wwise. This reduces the potential interruptions caused by message boxes and dialogs. For instance, enabling the automation mode silently accepts: project migration, project load log, EULA acceptance, project licence display and generic message boxes.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DebugEnableAutomationModeArgs {
    /// When set to true, the automation mode reduces the blocking of dialogs and popups. When set to false, it reverts to the normal mode.
    pub enable: bool,
}

impl DebugEnableAutomationModeArgs {
    /// Creates a new [`DebugEnableAutomationModeArgs`] with the required fields.
    pub fn new(enable: bool) -> Self {
        Self { enable }
    }
}

impl From<bool> for DebugEnableAutomationModeArgs {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`DebugEnableAutomationModeArgs`].
pub type EnableAutomationModeArgs = DebugEnableAutomationModeArgs;
