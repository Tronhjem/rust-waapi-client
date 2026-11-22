// Generated bindings for ak.wwise.core.soundbank.setInclusions

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.soundbank.setInclusions`.
///
/// Modifies a SoundBank's inclusion list. The 'operation' argument determines how the 'inclusions' argument modifies the SoundBank's inclusion list; 'inclusions' may be added to / removed from / replace the SoundBank's inclusion list.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSoundbankSetInclusionsArgs {
    /// Determines how the 'inclusions' argument is used to modify the SoundBank's inclusion list; 'inclusions' may be used to add to / remove from / replace the SoundBank's inclusion list.
    pub operation: SoundBankInclusionOperation,
    /// The ID (GUID), name, or path of the SoundBank to add an inclusion to.
    pub soundbank: ObjectRef,
    /// An array of SoundBank inclusions.
    pub inclusions: Vec<SoundBankInclusion>,
}

impl CoreSoundbankSetInclusionsArgs {
    /// Creates a new [`CoreSoundbankSetInclusionsArgs`] with the required fields.
    pub fn new(
        soundbank: impl Into<ObjectRef>,
        inclusions: Vec<SoundBankInclusion>,
        operation: SoundBankInclusionOperation,
    ) -> Self {
        Self {
            soundbank: soundbank.into(),
            inclusions,
            operation,
        }
    }
}

impl<T0: Into<ObjectRef>, T1: Into<Vec<SoundBankInclusion>>, T2: Into<SoundBankInclusionOperation>>
    From<(T0, T1, T2)> for CoreSoundbankSetInclusionsArgs
{
    fn from((soundbank, inclusions, operation): (T0, T1, T2)) -> Self {
        Self::new(soundbank, inclusions.into(), operation.into())
    }
}

/// Module-scoped alias for [`CoreSoundbankSetInclusionsArgs`].
pub type SetInclusionsArgs = CoreSoundbankSetInclusionsArgs;
