// Generated bindings for topic ak.wwise.core.object.curveChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.object.curveChanged`.
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
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreObjectCurveChangedOptions {
    /// Data to be returned for every curve change.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_: Option<Vec<ObjectReturnField>>,
    /// The ID (GUID) of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl CoreObjectCurveChangedOptions {
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
