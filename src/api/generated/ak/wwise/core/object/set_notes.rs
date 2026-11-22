// Generated bindings for ak.wwise.core.object.setNotes

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setNotes`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetNotesArgs {
    /// The new notes of the object.
    pub value: String,
    /// The ID (GUID), name, or path of the object to set value.
    pub object: ObjectRef,
}

impl CoreObjectSetNotesArgs {
    pub fn new(object: ObjectRef, value: String) -> Self {
        Self { object, value }
    }
}
