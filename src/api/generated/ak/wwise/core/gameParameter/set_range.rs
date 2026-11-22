// Generated bindings for ak.wwise.core.gameParameter.setRange

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.gameParameter.setRange`.
///
/// Sets the Min and Max properties on a Game Parameter. Modifies the RTPC curves and blend tracks that use this Game Parameter for their X axis.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreGameParameterSetRangeArgs {
    /// The minimum value of the Game Parameter.
    pub min: f64,
    /// The maximum value of the Game Parameter.
    pub max: f64,
    /// The ID (GUID), name, or path of the Game Parameter.
    pub object: ObjectRef,
    /// Modifying the Min or Max value of a Game Parameter affects the RTPC curves and blend tracks that use that Game Parameter for their X axis. Two actions are possible:
    /// stretch: All items are kept but their X positions might change as the curve/blend track is stretched or compressed to match the new range in X. Values of Set Game Parameter actions are also scaled.
    /// preserve_x: Maintains the X position of all the items, but those that fall outside the new range are deleted. Values of Set Game Parameter actions are clamped.
    #[serde(rename = "onCurveUpdate")]
    pub on_curve_update: OnCurveUpdate,
}

impl CoreGameParameterSetRangeArgs {
    /// Creates a new [`CoreGameParameterSetRangeArgs`] with the required fields.
    pub fn new(
        object: impl Into<ObjectRef>,
        min: f64,
        max: f64,
        on_curve_update: OnCurveUpdate,
    ) -> Self {
        Self {
            object: object.into(),
            min,
            max,
            on_curve_update,
        }
    }
}

/// Module-scoped alias for [`CoreGameParameterSetRangeArgs`].
pub type SetRangeArgs = CoreGameParameterSetRangeArgs;
