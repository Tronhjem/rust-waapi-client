// Generated bindings for ak.wwise.core.object.setName

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setName`.
///
/// Renames an object.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetNameArgs {
    /// The new name of the object.
    pub value: String,
    /// The ID (GUID), name, or path of the object to rename.
    pub object: ObjectRef,
}

impl CoreObjectSetNameArgs {
    /// Creates a new [`CoreObjectSetNameArgs`] with the required fields.
    pub fn new(object: ObjectRef, value: String) -> Self {
        Self { object, value }
    }
}
