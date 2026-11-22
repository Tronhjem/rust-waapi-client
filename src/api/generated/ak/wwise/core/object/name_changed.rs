// Generated bindings for topic ak.wwise.core.object.nameChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.nameChanged`.
///
/// Sent when an object is renamed. Publishes the renamed object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectNameChangedPayload {
    /// The object that was renamed.
    pub object: WwiseObject,
    /// The new name of the object.
    #[serde(rename = "newName")]
    pub new_name: String,
    /// The previous name of the object.
    #[serde(rename = "oldName")]
    pub old_name: String,
}

/// Subscription options for `ak.wwise.core.object.nameChanged`.
///
/// Sent when an object is renamed. Publishes the renamed object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectNameChangedOptions {
    /// Data to be returned for every name change.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectNameChangedOptions {
    /// Creates a new [`CoreObjectNameChangedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every name change.
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

impl From<()> for CoreObjectNameChangedOptions {
    fn from((): ()) -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[ObjectReturnField; N]> for CoreObjectNameChangedOptions {
    fn from(return_fields: [ObjectReturnField; N]) -> Self {
        Self::new().return_fields(return_fields)
    }
}

impl From<Vec<ObjectReturnField>> for CoreObjectNameChangedOptions {
    fn from(return_fields: Vec<ObjectReturnField>) -> Self {
        Self::new().return_fields(return_fields)
    }
}

/// Module-scoped alias for [`CoreObjectNameChangedPayload`].
pub type NameChangedPayload = CoreObjectNameChangedPayload;

/// Module-scoped alias for [`CoreObjectNameChangedOptions`].
pub type NameChangedOptions = CoreObjectNameChangedOptions;
