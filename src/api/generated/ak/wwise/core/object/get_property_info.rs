// Generated bindings for ak.wwise.core.object.getPropertyInfo



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.wwise.core.object.getPropertyInfo`.
///
/// Retrieves information about an object property. Note that this function does not return the value of a property. To retrieve the value of a property, see `ak.wwise.core.object.get` and the WAAPI query return options documentation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectGetPropertyInfoArgs {
    /// The ID (class ID) of the object to retrieve the property from.
    #[serde(rename = "classId", skip_serializing_if = "Option::is_none")]
    pub class_id: Option<u32>,
    /// The ID (GUID), name, or path of the object to watch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectRef>,
    /// The name of the property to retrieve. For more information, refer to the property list of Wwise Objects listed in the Wwise Objects Reference.
    pub property: Property,
}

impl CoreObjectGetPropertyInfoArgs {
    /// Creates a new [`CoreObjectGetPropertyInfoArgs`] with the required fields.
    pub fn new(property: Property) -> Self {
        Self {
            property,
            class_id: None,
            object: None,
        }
    }

    /// The ID (class ID) of the object to retrieve the property from.
    pub fn class_id(mut self, class_id: impl Into<u32>) -> Self {
        self.class_id = Some(class_id.into());
        self
    }

    /// The ID (GUID), name, or path of the object to watch.
    pub fn object(mut self, object: impl Into<ObjectRef>) -> Self {
        self.object = Some(object.into());
        self
    }
}

/// Response from `ak.wwise.core.object.getPropertyInfo`.
///
/// Retrieves information about an object property. Note that this function does not return the value of a property. To retrieve the value of a property, see `ak.wwise.core.object.get` and the WAAPI query return options documentation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectGetPropertyInfoResponse {
    /// The name of the property.
    #[serde(default)]
    pub name: Option<String>,
    /// The type of the property. For more information, refer to the property list of Wwise Objects found in the Wwise Objects Reference.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    /// The default value of the property.
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    /// The ID (audio engine ID) of the property.
    #[serde(rename = "audioEngineId", default)]
    pub audio_engine_id: Option<u32>,
    /// The features supported by the property.
    #[serde(default)]
    pub supports: Option<PropertyInfoSupports>,
    /// Displays information related to the property.
    #[serde(default)]
    pub display: Option<serde_json::Value>,
    /// List of dependencies on other properties with their related action and conditions.
    #[serde(default)]
    pub dependencies: Vec<serde_json::Value>,
    /// Restricts the value of the property.
    #[serde(default)]
    pub restriction: Option<serde_json::Value>,
    /// Information related to how the property is displayed.
    #[serde(default)]
    pub ui: Option<serde_json::Value>,
}

