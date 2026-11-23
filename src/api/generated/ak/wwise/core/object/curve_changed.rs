// Generated bindings for topic ak.wwise.core.object.curveChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.curveChanged`.
///
/// Sent when one or many curves are changed.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectCurveChangedPayload {
    /// The owner object.
    pub owner: WwiseObject,
    /// The type of change applied to the curve.
    pub notification: String,
    /// The curve object that was modified.
    pub curve: WwiseObject,
}

/// Subscription options for `ak.wwise.core.object.curveChanged`.
///
/// Sent when one or many curves are changed.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectCurveChangedOptions {
    /// Data to be returned for every curve change.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectCurveChangedOptions {
    /// Creates a new [`CoreObjectCurveChangedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every curve change.
    pub fn return_fields(mut self, return_fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields = Some(return_fields.into());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_(self, return_fields: impl Into<Vec<ObjectReturnField>>) -> Self {
        self.return_fields(return_fields)
    }

    /// The ID (GUID) of the platform.
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
