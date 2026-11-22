// Generated bindings for ak.wwise.core.object.getAttenuationCurve

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.getAttenuationCurve`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectGetAttenuationCurveArgs {
    /// The ID (GUID), name, or path of attenuation object.
    pub object: ObjectRef,
    /// The ID (GUID) or unique name of the platform to get curves. Set to null-guid for unlinked reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// Type of attenuation curve.
    #[serde(rename = "curveType")]
    pub curve_type: AttenuationCurveType,
}

impl CoreObjectGetAttenuationCurveArgs {
    pub fn new(object: ObjectRef, curve_type: AttenuationCurveType) -> Self {
        Self {
            object,
            curve_type,
            platform: None,
        }
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}

/// Response from `ak.wwise.core.object.getAttenuationCurve`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectGetAttenuationCurveResponse {
    /// The attenuation curve.
    pub points: Vec<CurvePoint2D>,
    /// Name of attenuation curve.
    #[serde(rename = "curveType")]
    pub curve_type: AttenuationCurveType,
    /// Defines whether the curve has no points, has its own set of points, or uses those of the VolumeDryUsage curve.
    #[serde(rename = "use")]
    pub use_: AttenuationCurveUse,
}
