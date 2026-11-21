// Generated bindings for ak.wwise.core.object.diff

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.diff`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectDiffArgs {
    /// The ID (GUID), name, or path of the source object.
    pub source: ObjectRef,
    /// The ID (GUID), name, or path of the target object.
    pub target: ObjectRef,
}

impl CoreObjectDiffArgs {
    pub fn new(source: ObjectRef, target: ObjectRef) -> Self {
        Self { source, target }
    }
}

/// Response from `ak.wwise.core.object.diff`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectDiffResponse {
    /// Array of names of properties and references that differ.
    #[serde(default)]
    pub properties: Vec<String>,
    /// Array of names of lists that differ.
    #[serde(default)]
    pub lists: Vec<String>,
}
