// Generated bindings for ak.wwise.core.transport.executeAction

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.transport.executeAction`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportExecuteActionArgs {
    /// The transport object ID to be used with all other ak.wwise.core.transport functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<u32>,
    /// The action to execute.
    pub action: TransportAction,
}

impl CoreTransportExecuteActionArgs {
    pub fn new(action: TransportAction) -> Self {
        Self {
            action,
            transport: None,
        }
    }

    pub fn transport(mut self, transport: impl Into<u32>) -> Self {
        self.transport = Some(transport.into());
        self
    }
}
