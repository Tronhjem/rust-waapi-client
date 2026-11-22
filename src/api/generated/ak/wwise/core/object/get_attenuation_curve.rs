// Generated bindings for ak.wwise.core.object.getAttenuationCurve

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.getAttenuationCurve`.
///
/// Gets the specified attenuation curve for a given attenuation object.
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
    /// Creates a new [`CoreObjectGetAttenuationCurveArgs`] with the required fields.
    pub fn new(object: impl Into<ObjectRef>, curve_type: AttenuationCurveType) -> Self {
        Self {
            object: object.into(),
            curve_type,
            platform: None,
        }
    }

    /// The ID (GUID) or unique name of the platform to get curves. Set to null-guid for unlinked reference.
    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}

impl<T0: Into<ObjectRef>, T1: Into<AttenuationCurveType>> From<(T0, T1)>
    for CoreObjectGetAttenuationCurveArgs
{
    fn from((object, curve_type): (T0, T1)) -> Self {
        Self::new(object, curve_type.into())
    }
}

/// Response from `ak.wwise.core.object.getAttenuationCurve`.
///
/// Gets the specified attenuation curve for a given attenuation object.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreObjectGetAttenuationCurveResponse {
    /// The attenuation curve.
    pub points: Vec<CurvePoint2D>,
    /// Name of attenuation curve.
    #[serde(rename = "curveType")]
    pub curve_type: AttenuationCurveType,
    /// Defines whether the curve has no points, has its own set of points, uses those of the VolumeDryUsage curve, or uses the global curve defined in the Project Settings.
    #[serde(rename = "use")]
    pub use_: AttenuationCurveUse,
}

/// Module-scoped alias for [`CoreObjectGetAttenuationCurveArgs`].
pub type GetAttenuationCurveArgs = CoreObjectGetAttenuationCurveArgs;

/// Module-scoped alias for [`CoreObjectGetAttenuationCurveResponse`].
pub type GetAttenuationCurveResponse = CoreObjectGetAttenuationCurveResponse;
