// Generated bindings for ak.wwise.core.workUnit.load

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.workUnit.load`.
///
/// Load a Work Unit that was previously unloaded. The Undo history will be cleared.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreWorkUnitLoadArgs {
    /// The ID (GUID), name, or path of the Work Unit to load.
    pub object: ObjectRef,
}

impl CoreWorkUnitLoadArgs {
    /// Creates a new [`CoreWorkUnitLoadArgs`] with the required fields.
    pub fn new(object: impl Into<ObjectRef>) -> Self {
        Self {
            object: object.into(),
        }
    }
}

impl From<ObjectRef> for CoreWorkUnitLoadArgs {
    fn from(value: ObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreWorkUnitLoadArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CoreWorkUnitLoadArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreWorkUnitLoadArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`CoreWorkUnitLoadArgs`].
pub type LoadArgs = CoreWorkUnitLoadArgs;
