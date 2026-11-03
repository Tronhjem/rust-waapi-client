// Generated bindings for ak.wwise.ui.getSelectedObjects

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Options for `ak.wwise.ui.getSelectedObjects`.

pub type WwiseUiGetSelectedObjectsOptions = ObjectReturnOptions;

/// Response from `ak.wwise.ui.getSelectedObjects`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WwiseUiGetSelectedObjectsResponse {
    /// The selected objects, in the format specified in the options
    /// Can be empty if no object is selected.
    pub objects: Vec<WwiseObject>,
}

pub type WwiseUiGetselectedobjectsOptions = WwiseUiGetSelectedObjectsOptions;

pub type WwiseUiGetselectedobjectsResponse = WwiseUiGetSelectedObjectsResponse;
