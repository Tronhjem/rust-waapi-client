// Generated bindings for topic ak.wwise.core.switchContainer.assignmentAdded

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.switchContainer.assignmentAdded`.
///
/// Sent when an assignment is added to a Switch Container.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSwitchContainerAssignmentAddedPayload {
    /// The child object that was assigned.
    pub child: WwiseObject,
    /// The State or Switch to which the child object was assigned.
    #[serde(rename = "stateOrSwitch")]
    pub state_or_switch: WwiseObject,
    /// The Switch Container to which the assignment was added.
    #[serde(rename = "switchContainer")]
    pub switch_container: WwiseObject,
}

/// Subscription options for `ak.wwise.core.switchContainer.assignmentAdded`.
///
/// Sent when an assignment is added to a Switch Container.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreSwitchContainerAssignmentAddedOptions {
    /// Data to be returned for the Switch Container and the assignment (pair of objects).
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
}

impl CoreSwitchContainerAssignmentAddedOptions {
    /// Creates a new [`CoreSwitchContainerAssignmentAddedOptions`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Data to be returned for the Switch Container and the assignment (pair of objects).
    pub fn return_fields<I, T>(mut self, return_fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields = Some(return_fields.into_iter().map(Into::into).collect());
        self
    }

    /// Alias for [`Self::return_fields`].
    pub fn return_<I, T>(self, return_fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields(return_fields)
    }
}

impl From<()> for CoreSwitchContainerAssignmentAddedOptions {
    fn from((): ()) -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[ObjectReturnField; N]> for CoreSwitchContainerAssignmentAddedOptions {
    fn from(return_fields: [ObjectReturnField; N]) -> Self {
        Self::new().return_fields(return_fields)
    }
}

impl From<Vec<ObjectReturnField>> for CoreSwitchContainerAssignmentAddedOptions {
    fn from(return_fields: Vec<ObjectReturnField>) -> Self {
        Self::new().return_fields(return_fields)
    }
}

/// Module-scoped alias for [`CoreSwitchContainerAssignmentAddedPayload`].
pub type AssignmentAddedPayload = CoreSwitchContainerAssignmentAddedPayload;

/// Module-scoped alias for [`CoreSwitchContainerAssignmentAddedOptions`].
pub type AssignmentAddedOptions = CoreSwitchContainerAssignmentAddedOptions;
