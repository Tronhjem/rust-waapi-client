// Generated bindings for ak.wwise.core.object.setStateGroups

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setStateGroups`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetStateGroupsArgs {
    /// An array of State Group objects to set.
    #[serde(rename = "stateGroups")]
    pub state_groups: Vec<ObjectRef>,
    /// The ID (GUID), name, or path of the object to which to add a State Group.
    pub object: ObjectRef,
}

impl CoreObjectSetStateGroupsArgs {
    pub fn new(object: ObjectRef, state_groups: Vec<ObjectRef>) -> Self {
        Self {
            object,
            state_groups,
        }
    }
}
