// Generated bindings for ak.wwise.core.object.setProperty

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setProperty`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetPropertyArgs {
    /// The ID (GUID), name, or path of the object to set value.
    pub object: ObjectRef,
    /// The value of the object.
    pub value: PropertyValue,
    /// The name of the property
    /// For more information, refer to the property list of Wwise Objects listed in .
    pub property: Property,
    /// The ID (GUID) or unique name of the platform
    /// This is used to set values for unlinked properties
    /// This is an optional argument
    /// When not specified, the current platform is used to set the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
}

impl CoreObjectSetPropertyArgs {
    pub fn new(object: ObjectRef, property: Property, value: PropertyValue) -> Self {
        Self {
            object,
            property,
            value,
            platform: None,
        }
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
