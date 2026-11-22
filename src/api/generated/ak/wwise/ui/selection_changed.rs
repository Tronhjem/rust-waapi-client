// Generated bindings for topic ak.wwise.ui.selectionChanged

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.ui.selectionChanged`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiSelectionChangedPayload {
    /// The selected objects, in the format specified in the options. Can be empty if no object is selected.
    pub objects: Vec<WwiseObject>,
}

/// Subscription options for `ak.wwise.ui.selectionChanged`.
pub type UiSelectionChangedOptions = ObjectReturnOptions;
