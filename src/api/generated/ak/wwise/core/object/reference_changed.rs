// Generated bindings for topic ak.wwise.core.object.referenceChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.referenceChanged`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectReferenceChangedPayload {
    /// The object which had its reference changed.
    pub object: WwiseObject,
    /// The ID (GUID) of the platform if the reference was changed for a single platform. When not provided, the reference is changed for all platforms.
    #[serde(default)]
    pub platform: Option<String>,
    /// The name of the reference that was changed. See the object's description in wobjects.xml for an object's possible references.
    pub reference: String,
    /// The previous referenced object.
    pub old: WwiseObject,
    /// The new referenced object.
    pub new: WwiseObject,
}

/// Subscription options for `ak.wwise.core.object.referenceChanged`.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectReferenceChangedOptions {
    /// Data to be returned for the object, the old reference and the new reference.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectReferenceChangedOptions {
    pub fn new() -> Self {
        Self::default()
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
