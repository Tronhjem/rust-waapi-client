// Generated bindings for topic ak.wwise.core.object.propertyChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.propertyChanged`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectPropertyChangedPayload {
    /// The watched object.
    pub object: WwiseObject,
    /// The watched property of the watched object.
    pub property: Property,
    /// The ID (GUID) of the platform for which the change occurred.
    pub platform: String,
    /// The previous value.
    pub old: serde_json::Value,
    /// The new value.
    pub new: serde_json::Value,
}

/// Subscription options for `ak.wwise.core.object.propertyChanged`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectPropertyChangedOptions {
    /// Data to be returned for every change on "property" of "object". See for more information on the properties available.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID), name, or path of the object to watch.
    pub object: ObjectRef,
    /// The property of the watched object to watch.
    pub property: Property,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectPropertyChangedOptions {
    pub fn new(object: ObjectRef, property: Property) -> Self {
        Self {
            object,
            property,
            return_: None,
            platform: None,
        }
    }

    pub fn return_(mut self, return_: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_ = Some(return_.into());
        self
    }

    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
