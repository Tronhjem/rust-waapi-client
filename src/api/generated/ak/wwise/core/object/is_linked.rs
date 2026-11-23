// Generated bindings for ak.wwise.core.object.isLinked

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.isLinked`.
///
/// Indicates whether a property, reference, or object list is bound to a particular platform or to all platforms.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectIsLinkedArgs {
    /// The ID (GUID), name, or path of the object on which to query the link or unlink status for the specified properties.
    pub object: ObjectRef,
    /// The name of the property. For more information, refer to the list of Wwise Object properties in the Wwise Objects Reference.
    pub property: Property,
    /// The ID (GUID) or unique name of the platform.
    pub platform: PlatformRef,
}

impl CoreObjectIsLinkedArgs {
    /// Creates a new [`CoreObjectIsLinkedArgs`] with the required fields.
    pub fn new(object: ObjectRef, property: Property, platform: PlatformRef) -> Self {
        Self {
            object,
            property,
            platform,
        }
    }
}

/// Response from `ak.wwise.core.object.isLinked`.
///
/// Indicates whether a property, reference, or object list is bound to a particular platform or to all platforms.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectIsLinkedResponse {
    /// Indicates whether the object is linked for the specified property.
    #[serde(default)]
    pub linked: Option<bool>,
}
