// Generated bindings for ak.wwise.core.workUnit.unload

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.workUnit.unload`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreWorkUnitUnloadArgs {
    /// The ID (GUID), name, or path of the Work Unit to unload.
    pub object: ObjectRef,
}

impl CoreWorkUnitUnloadArgs {
    pub fn new(object: ObjectRef) -> Self {
        Self { object }
    }
}
