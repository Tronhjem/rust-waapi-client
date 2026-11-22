// Generated bindings for ak.soundengine.setGameObjectOutputBusVolume

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.soundengine.setGameObjectOutputBusVolume`.
///
/// Set the output bus volume (direct) to be used for the specified game object. See `AK::SoundEngine::SetGameObjectOutputBusVolume`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetGameObjectOutputBusVolumeArgs {
    /// Associated listener game object ID.
    pub listener: GameObjectRef,
    /// Associated emitter game object ID.
    pub emitter: GameObjectRef,
    /// A multiplier where 0 means silence and 1 means no change. Therefore, values between 0 and 1 attenuate the sound, and values greater than 1 amplify it.
    #[serde(rename = "controlValue")]
    pub control_value: f64,
}

impl SoundengineSetGameObjectOutputBusVolumeArgs {
    /// Creates a new [`SoundengineSetGameObjectOutputBusVolumeArgs`] with the required fields.
    pub fn new(
        emitter: impl Into<GameObjectRef>,
        listener: impl Into<GameObjectRef>,
        control_value: f64,
    ) -> Self {
        Self {
            emitter: emitter.into(),
            listener: listener.into(),
            control_value,
        }
    }
}

impl<T0: Into<GameObjectRef>, T1: Into<GameObjectRef>, T2: Into<f64>> From<(T0, T1, T2)>
    for SoundengineSetGameObjectOutputBusVolumeArgs
{
    fn from((emitter, listener, control_value): (T0, T1, T2)) -> Self {
        Self::new(emitter, listener, control_value.into())
    }
}

/// Module-scoped alias for [`SoundengineSetGameObjectOutputBusVolumeArgs`].
pub type SetGameObjectOutputBusVolumeArgs = SoundengineSetGameObjectOutputBusVolumeArgs;
