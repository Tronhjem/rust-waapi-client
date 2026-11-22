// Generated bindings for ak.wwise.core.object.setName

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setName`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetNameArgs {
    /// The new name of the object.
    pub value: String,
    /// The ID (GUID), name, or path of the object to rename.
    pub object: ObjectRef,
}

impl CoreObjectSetNameArgs {
    pub fn new(object: ObjectRef, value: String) -> Self {
        Self { object, value }
    }
}
