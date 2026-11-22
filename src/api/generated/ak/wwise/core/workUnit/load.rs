// Generated bindings for ak.wwise.core.workUnit.load

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.workUnit.load`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreWorkUnitLoadArgs {
    /// The ID (GUID), name, or path of the Work Unit to load.
    pub object: ObjectRef,
}

impl CoreWorkUnitLoadArgs {
    pub fn new(object: ObjectRef) -> Self {
        Self { object }
    }
}
