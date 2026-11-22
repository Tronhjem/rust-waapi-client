// Generated bindings for ak.wwise.core.object.setStateGroups

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setStateGroups`.
///
/// Sets the State Group objects associated with an object. Note, this will remove any previously associated State Group.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetStateGroupsArgs {
    /// An array of State Group objects to set.
    #[serde(rename = "stateGroups")]
    pub state_groups: Vec<ObjectRef>,
    /// The ID (GUID), name, or path of the object to which to add a State Group.
    pub object: ObjectRef,
}

impl CoreObjectSetStateGroupsArgs {
    /// Creates a new [`CoreObjectSetStateGroupsArgs`] with the required fields.
    pub fn new(object: impl Into<ObjectRef>, state_groups: Vec<ObjectRef>) -> Self {
        Self {
            object: object.into(),
            state_groups,
        }
    }
}

impl<T0: Into<ObjectRef>, T1: Into<Vec<ObjectRef>>> From<(T0, T1)>
    for CoreObjectSetStateGroupsArgs
{
    fn from((object, state_groups): (T0, T1)) -> Self {
        Self::new(object, state_groups.into())
    }
}

/// Module-scoped alias for [`CoreObjectSetStateGroupsArgs`].
pub type SetStateGroupsArgs = CoreObjectSetStateGroupsArgs;
