// Generated bindings for ak.wwise.core.object.setLinked

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setLinked`.
///
/// Link or unlink a property/reference or object list to a particular platform.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetLinkedArgs {
    /// The ID (GUID), name, or path of the object to set value.
    pub object: ObjectRef,
    /// The name of the property. For more information, refer to the property list of Wwise Objects listed in the Wwise Objects Reference.
    pub property: Property,
    /// The ID (GUID) or unique name of the platform.
    pub platform: PlatformRef,
    /// Is it linked.
    pub linked: bool,
}

impl CoreObjectSetLinkedArgs {
    /// Creates a new [`CoreObjectSetLinkedArgs`] with the required fields.
    pub fn new(object: ObjectRef, property: Property, platform: PlatformRef, linked: bool) -> Self {
        Self {
            object,
            property,
            platform,
            linked,
        }
    }
}
