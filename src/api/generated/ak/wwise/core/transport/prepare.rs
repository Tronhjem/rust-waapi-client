// Generated bindings for ak.wwise.core.transport.prepare

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.transport.prepare`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreTransportPrepareArgs {
    /// The ID (GUID), name, or path of the object to control via the transport object.
    pub object: ObjectRef,
}

impl CoreTransportPrepareArgs {
    pub fn new(object: ObjectRef) -> Self {
        Self { object }
    }
}
