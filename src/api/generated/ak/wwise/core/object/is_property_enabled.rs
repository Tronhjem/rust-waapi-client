// Generated bindings for ak.wwise.core.object.isPropertyEnabled

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.isPropertyEnabled`.
///
/// Returns true if a property is enabled based on the values of the properties it depends on.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectIsPropertyEnabledArgs {
    /// The ID (GUID), name, or path of the object to check.
    pub object: ObjectRef,
    /// The name of the property. For more information, refer to the property list of Wwise Objects listed in the Wwise Objects Reference.
    pub property: Property,
    /// The ID (GUID) or unique name of the platform to link the reference. Set to null-guid for unlinked reference.
    pub platform: PlatformRef,
}

impl CoreObjectIsPropertyEnabledArgs {
    /// Creates a new [`CoreObjectIsPropertyEnabledArgs`] with the required fields.
    pub fn new(object: ObjectRef, platform: PlatformRef, property: Property) -> Self {
        Self {
            object,
            platform,
            property,
        }
    }
}

/// Response from `ak.wwise.core.object.isPropertyEnabled`.
///
/// Returns true if a property is enabled based on the values of the properties it depends on.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectIsPropertyEnabledResponse {
    /// Indicates if the property is enabled.
    #[serde(rename = "return", default)]
    pub enabled: Option<bool>,
}
