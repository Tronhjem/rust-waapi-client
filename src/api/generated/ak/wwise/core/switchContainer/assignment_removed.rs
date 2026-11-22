// Generated bindings for topic ak.wwise.core.switchContainer.assignmentRemoved

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.switchContainer.assignmentRemoved`.
///
/// Sent when an assignment is removed from a Switch Container.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSwitchContainerAssignmentRemovedPayload {
    /// The child object that was assigned.
    pub child: WwiseObject,
    /// The State or Switch to which the child object was assigned.
    #[serde(rename = "stateOrSwitch")]
    pub state_or_switch: WwiseObject,
    /// The Switch Container which had the assignment removed.
    #[serde(rename = "switchContainer")]
    pub switch_container: WwiseObject,
}

/// Subscription options for `ak.wwise.core.switchContainer.assignmentRemoved`.
///
/// Sent when an assignment is removed from a Switch Container.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreSwitchContainerAssignmentRemovedOptions {
    /// Data to be returned for the Switch Container and the assignment (pair of objects).
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
}

impl CoreSwitchContainerAssignmentRemovedOptions {
    /// Creates a new [`CoreSwitchContainerAssignmentRemovedOptions`] with all fields unset.
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

impl From<()> for CoreSwitchContainerAssignmentRemovedOptions {
    fn from((): ()) -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[ObjectReturnField; N]> for CoreSwitchContainerAssignmentRemovedOptions {
    fn from(return_fields: [ObjectReturnField; N]) -> Self {
        Self::new().return_fields(return_fields)
    }
}

impl From<Vec<ObjectReturnField>> for CoreSwitchContainerAssignmentRemovedOptions {
    fn from(return_fields: Vec<ObjectReturnField>) -> Self {
        Self::new().return_fields(return_fields)
    }
}

/// Module-scoped alias for [`CoreSwitchContainerAssignmentRemovedPayload`].
pub type AssignmentRemovedPayload = CoreSwitchContainerAssignmentRemovedPayload;

/// Module-scoped alias for [`CoreSwitchContainerAssignmentRemovedOptions`].
pub type AssignmentRemovedOptions = CoreSwitchContainerAssignmentRemovedOptions;
