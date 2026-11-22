// Generated bindings for ak.wwise.core.object.diff

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.diff`.
///
/// Compares properties and lists of the source object with those in the target object.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectDiffArgs {
    /// The ID (GUID), name, or path of the source object.
    pub source: ObjectRef,
    /// The ID (GUID), name, or path of the target object.
    pub target: ObjectRef,
}

impl CoreObjectDiffArgs {
    /// Creates a new [`CoreObjectDiffArgs`] with the required fields.
    pub fn new(source: impl Into<ObjectRef>, target: impl Into<ObjectRef>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
}

impl<T0: Into<ObjectRef>, T1: Into<ObjectRef>> From<(T0, T1)> for CoreObjectDiffArgs {
    fn from((source, target): (T0, T1)) -> Self {
        Self::new(source, target)
    }
}

/// Response from `ak.wwise.core.object.diff`.
///
/// Compares properties and lists of the source object with those in the target object.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectDiffResponse {
    /// Array of names of properties and references that differ.
    #[serde(default)]
    pub properties: Vec<String>,
    /// Array of names of lists that differ.
    #[serde(default)]
    pub lists: Vec<String>,
}

/// Module-scoped alias for [`CoreObjectDiffArgs`].
pub type DiffArgs = CoreObjectDiffArgs;

/// Module-scoped alias for [`CoreObjectDiffResponse`].
pub type DiffResponse = CoreObjectDiffResponse;
