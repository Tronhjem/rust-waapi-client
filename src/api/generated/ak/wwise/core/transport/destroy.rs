// Generated bindings for ak.wwise.core.transport.destroy

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.transport.destroy`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportDestroyArgs {
    /// The transport object ID to be used with all other ak.wwise.core.transport functions.
    pub transport: u32,
}

impl CoreTransportDestroyArgs {
    pub fn new(transport: u32) -> Self {
        Self { transport }
    }
}
