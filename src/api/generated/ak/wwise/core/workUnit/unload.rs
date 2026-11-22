// Generated bindings for ak.wwise.core.workUnit.unload

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.workUnit.unload`.
///
/// Unload a Work Unit. No object contained in the Work Unit will be available after this call. If the Work Unit is modified and not saved, the function will return error. The Undo history will be cleared.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreWorkUnitUnloadArgs {
    /// The ID (GUID), name, or path of the Work Unit to unload.
    pub object: ObjectRef,
}

impl CoreWorkUnitUnloadArgs {
    /// Creates a new [`CoreWorkUnitUnloadArgs`] with the required fields.
    pub fn new(object: impl Into<ObjectRef>) -> Self {
        Self {
            object: object.into(),
        }
    }
}

impl From<ObjectRef> for CoreWorkUnitUnloadArgs {
    fn from(value: ObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreWorkUnitUnloadArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CoreWorkUnitUnloadArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreWorkUnitUnloadArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`CoreWorkUnitUnloadArgs`].
pub type UnloadArgs = CoreWorkUnitUnloadArgs;
