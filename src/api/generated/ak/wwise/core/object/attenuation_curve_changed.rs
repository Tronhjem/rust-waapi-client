// Generated bindings for topic ak.wwise.core.object.attenuationCurveChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.attenuationCurveChanged`.
///
/// Sent when an attenuation curve is changed.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectAttenuationCurveChangedPayload {
    /// The curve type that changed.
    #[serde(rename = "curveType")]
    pub curve_type: AttenuationCurveType,
    /// The attenuation object owning the curve.
    pub attenuation: WwiseObject,
}

/// Subscription options for `ak.wwise.core.object.attenuationCurveChanged`.
///
/// Sent when an attenuation curve is changed.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectAttenuationCurveChangedOptions {
    /// Data to be returned for every attenuation curve change.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectAttenuationCurveChangedOptions {
    /// Creates a new [`CoreObjectAttenuationCurveChangedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for every attenuation curve change.
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
