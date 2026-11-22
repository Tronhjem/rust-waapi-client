// Generated bindings for ak.wwise.core.object.setProperty

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setProperty`.
///
/// Sets a property value of an object for a specific platform. See the Wwise Objects Reference for more information on the properties available on each object type. See `ak.wwise.core.object.setReference` to set a reference to an object. See `ak.wwise.core.object.get` to obtain the value of a property for an object.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetPropertyArgs {
    /// The value of the object.
    pub value: PropertyValue,
    /// The ID (GUID), name, or path of the object to set value.
    pub object: ObjectRef,
    /// The name of the property. For more information, refer to the property list of Wwise Objects listed in the Wwise Objects Reference.
    pub property: Property,
    /// The ID (GUID) or unique name of the platform. This is used to set values for unlinked properties. This is an optional argument. When not specified, the current platform is used to set the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
}

impl CoreObjectSetPropertyArgs {
    /// Creates a new [`CoreObjectSetPropertyArgs`] with the required fields.
    pub fn new(object: ObjectRef, property: Property, value: PropertyValue) -> Self {
        Self {
            object,
            property,
            value,
            platform: None,
        }
    }

    /// The ID (GUID) or unique name of the platform. This is used to set values for unlinked properties. This is an optional argument. When not specified, the current platform is used to set the property.
    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
