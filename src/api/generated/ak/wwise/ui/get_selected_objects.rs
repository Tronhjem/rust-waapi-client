// Generated bindings for ak.wwise.ui.getSelectedObjects

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Options for `ak.wwise.ui.getSelectedObjects`.
pub type UiGetSelectedObjectsOptions = ObjectReturnOptions;

/// Response from `ak.wwise.ui.getSelectedObjects`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiGetSelectedObjectsResponse {
    /// The selected objects, in the format specified in the options. Can be empty if no object is selected.
    pub objects: Vec<WwiseObject>,
}
