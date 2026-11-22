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
    pub fn new(
        object: impl Into<ObjectRef>,
        property: Property,
        platform: impl Into<PlatformRef>,
    ) -> Self {
        Self {
            object: object.into(),
            property,
            platform: platform.into(),
        }
    }
}

impl<T0: Into<ObjectRef>, T1: Into<Property>, T2: Into<PlatformRef>> From<(T0, T1, T2)>
    for CoreObjectIsLinkedArgs
{
    fn from((object, property, platform): (T0, T1, T2)) -> Self {
        Self::new(object, property.into(), platform)
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

/// Module-scoped alias for [`CoreObjectIsLinkedArgs`].
pub type IsLinkedArgs = CoreObjectIsLinkedArgs;

/// Module-scoped alias for [`CoreObjectIsLinkedResponse`].
pub type IsLinkedResponse = CoreObjectIsLinkedResponse;
