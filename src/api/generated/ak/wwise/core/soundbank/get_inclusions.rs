// Generated bindings for ak.wwise.core.soundbank.getInclusions

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.soundbank.getInclusions`.
///
/// Retrieves a SoundBank's inclusion list.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSoundbankGetInclusionsArgs {
    /// The ID (GUID), name, or path of the SoundBank to add an inclusion to.
    pub soundbank: ObjectRef,
}

impl CoreSoundbankGetInclusionsArgs {
    /// Creates a new [`CoreSoundbankGetInclusionsArgs`] with the required fields.
    pub fn new(soundbank: impl Into<ObjectRef>) -> Self {
        Self {
            soundbank: soundbank.into(),
        }
    }
}

impl From<ObjectRef> for CoreSoundbankGetInclusionsArgs {
    fn from(value: ObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreSoundbankGetInclusionsArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CoreSoundbankGetInclusionsArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreSoundbankGetInclusionsArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.soundbank.getInclusions`.
///
/// Retrieves a SoundBank's inclusion list.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSoundbankGetInclusionsResponse {
    /// An array of inclusions.
    #[serde(default)]
    pub inclusions: Vec<SoundBankInclusion>,
}

/// Module-scoped alias for [`CoreSoundbankGetInclusionsArgs`].
pub type GetInclusionsArgs = CoreSoundbankGetInclusionsArgs;

/// Module-scoped alias for [`CoreSoundbankGetInclusionsResponse`].
pub type GetInclusionsResponse = CoreSoundbankGetInclusionsResponse;
