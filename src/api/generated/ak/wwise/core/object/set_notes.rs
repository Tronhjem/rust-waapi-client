// Generated bindings for ak.wwise.core.object.setNotes

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setNotes`.
///
/// Sets the object's notes.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetNotesArgs {
    /// The new notes of the object.
    pub value: String,
    /// The ID (GUID), name, or path of the object to set value.
    pub object: ObjectRef,
}

impl CoreObjectSetNotesArgs {
    /// Creates a new [`CoreObjectSetNotesArgs`] with the required fields.
    pub fn new(object: impl Into<ObjectRef>, value: impl Into<String>) -> Self {
        Self {
            object: object.into(),
            value: value.into(),
        }
    }
}

impl<T0: Into<ObjectRef>, T1: Into<String>> From<(T0, T1)> for CoreObjectSetNotesArgs {
    fn from((object, value): (T0, T1)) -> Self {
        Self::new(object, value)
    }
}

/// Module-scoped alias for [`CoreObjectSetNotesArgs`].
pub type SetNotesArgs = CoreObjectSetNotesArgs;
