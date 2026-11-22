// Generated bindings for ak.wwise.core.object.setAttenuationCurve

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setAttenuationCurve`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetAttenuationCurveArgs {
    /// The attenuation curve.
    pub points: Vec<CurvePoint2D>,
    /// The ID (GUID), name, or path of attenuation object.
    pub object: ObjectRef,
    /// The ID (GUID) or unique name of the platform to set curves. Set to null-guid for unlinked curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// The type of attenuation curve.
    #[serde(rename = "curveType")]
    pub curve_type: AttenuationCurveType,
    /// Defines if the curve has no points, has its own set of points, uses those of the VolumeDryUsage curve, or uses the global curve defined in the Project Settings.
    #[serde(rename = "use")]
    pub use_: AttenuationCurveUse,
}

impl CoreObjectSetAttenuationCurveArgs {
    pub fn new(
        object: ObjectRef,
        curve_type: AttenuationCurveType,
        use_: AttenuationCurveUse,
        points: Vec<CurvePoint2D>,
    ) -> Self {
        Self {
            object,
            curve_type,
            use_,
            points,
            platform: None,
        }
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}
