// Generated bindings for topic ak.wwise.core.object.referenceChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.referenceChanged`.
///
/// Sent when an object reference is changed.
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
///
/// Sent when an object reference is changed.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectReferenceChangedOptions {
    /// Data to be returned for the object, the old reference and the new reference.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectReferenceChangedOptions {
    /// Creates a new [`CoreObjectReferenceChangedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for the object, the old reference and the new reference.
    pub fn return_fields<I, T>(mut self, return_fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields = Some(return_fields.into_iter().map(Into::into).collect());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_<I, T>(self, return_fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields(return_fields)
    }

    /// The ID (GUID) of the platform.
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}

impl From<()> for CoreObjectReferenceChangedOptions {
    fn from((): ()) -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[ObjectReturnField; N]> for CoreObjectReferenceChangedOptions {
    fn from(return_fields: [ObjectReturnField; N]) -> Self {
        Self::new().return_fields(return_fields)
    }
}

impl From<Vec<ObjectReturnField>> for CoreObjectReferenceChangedOptions {
    fn from(return_fields: Vec<ObjectReturnField>) -> Self {
        Self::new().return_fields(return_fields)
    }
}

/// Module-scoped alias for [`CoreObjectReferenceChangedPayload`].
pub type ReferenceChangedPayload = CoreObjectReferenceChangedPayload;

/// Module-scoped alias for [`CoreObjectReferenceChangedOptions`].
pub type ReferenceChangedOptions = CoreObjectReferenceChangedOptions;
