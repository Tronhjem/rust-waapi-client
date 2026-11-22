// Generated bindings for topic ak.wwise.core.object.preDeleted

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.preDeleted`.
///
/// Sent prior to an object's deletion. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectPreDeletedPayload {
    /// The object to be deleted.
    pub object: WwiseObject,
}

/// Subscription options for `ak.wwise.core.object.preDeleted`.
///
/// Sent prior to an object's deletion. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectPreDeletedOptions {
    /// Data to be returned for every object prior to its deletion.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectPreDeletedOptions {
    /// Creates a new [`CoreObjectPreDeletedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every object prior to its deletion.
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

impl From<()> for CoreObjectPreDeletedOptions {
    fn from((): ()) -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[ObjectReturnField; N]> for CoreObjectPreDeletedOptions {
    fn from(return_fields: [ObjectReturnField; N]) -> Self {
        Self::new().return_fields(return_fields)
    }
}

impl From<Vec<ObjectReturnField>> for CoreObjectPreDeletedOptions {
    fn from(return_fields: Vec<ObjectReturnField>) -> Self {
        Self::new().return_fields(return_fields)
    }
}

/// Module-scoped alias for [`CoreObjectPreDeletedPayload`].
pub type PreDeletedPayload = CoreObjectPreDeletedPayload;

/// Module-scoped alias for [`CoreObjectPreDeletedOptions`].
pub type PreDeletedOptions = CoreObjectPreDeletedOptions;
