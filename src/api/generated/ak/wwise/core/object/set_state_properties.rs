// Generated bindings for ak.wwise.core.object.setStateProperties

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setStateProperties`.
///
/// Set the state properties of an object. Note, this will remove any previous state property, including the default ones.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetStatePropertiesArgs {
    /// An array of state properties to set.
    #[serde(rename = "stateProperties")]
    pub state_properties: Vec<String>,
    /// The ID (GUID), name, or path of the object to which to set the state properties.
    pub object: ObjectRef,
}

impl CoreObjectSetStatePropertiesArgs {
    /// Creates a new [`CoreObjectSetStatePropertiesArgs`] with the required fields.
    pub fn new(object: impl Into<ObjectRef>, state_properties: Vec<String>) -> Self {
        Self {
            object: object.into(),
            state_properties,
        }
    }
}

impl<T0: Into<ObjectRef>, T1: Into<Vec<String>>> From<(T0, T1)>
    for CoreObjectSetStatePropertiesArgs
{
    fn from((object, state_properties): (T0, T1)) -> Self {
        Self::new(object, state_properties.into())
    }
}

/// Module-scoped alias for [`CoreObjectSetStatePropertiesArgs`].
pub type SetStatePropertiesArgs = CoreObjectSetStatePropertiesArgs;
