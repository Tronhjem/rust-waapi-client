// Generated bindings for ak.wwise.core.transport.executeAction

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.transport.executeAction`.
///
/// Executes an action on the given transport object, or all transport objects if none is specified.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportExecuteActionArgs {
    /// The transport object ID to be used with all other ak.wwise.core.transport functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<u32>,
    /// The action to execute.
    pub action: TransportAction,
}

impl CoreTransportExecuteActionArgs {
    /// Creates a new [`CoreTransportExecuteActionArgs`] with the required fields.
    pub fn new(action: TransportAction) -> Self {
        Self {
            action,
            transport: None,
        }
    }

    /// The transport object ID to be used with all other ak.wwise.core.transport functions.
    pub fn transport(mut self, transport: impl Into<u32>) -> Self {
        self.transport = Some(transport.into());
        self
    }
}

impl From<TransportAction> for CoreTransportExecuteActionArgs {
    fn from(value: TransportAction) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`CoreTransportExecuteActionArgs`].
pub type ExecuteActionArgs = CoreTransportExecuteActionArgs;
