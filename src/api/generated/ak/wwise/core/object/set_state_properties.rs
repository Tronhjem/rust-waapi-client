// Generated bindings for ak.wwise.core.object.setStateProperties

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setStateProperties`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetStatePropertiesArgs {
    #[serde(rename = "stateProperties")]
    pub state_properties: Vec<String>,
    /// The ID (GUID), name, or path of the object to which to set the state properties.
    pub object: ObjectRef,
}

impl CoreObjectSetStatePropertiesArgs {
    pub fn new(object: ObjectRef, state_properties: Vec<String>) -> Self {
        Self {
            object,
            state_properties,
        }
    }
}
