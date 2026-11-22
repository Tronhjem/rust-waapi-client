// Generated bindings for ak.soundengine.setObjectObstructionAndOcclusion



#![allow(unused_imports)]

#![allow(dead_code)]



use crate::api::generated::primitives::*;



/// Arguments for `ak.soundengine.setObjectObstructionAndOcclusion`.
///
/// Set a game object's obstruction and occlusion levels. This function is used to affect how an object should be heard by a specific listener. See `AK::SoundEngine::SetObjectObstructionAndOcclusion`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundengineSetObjectObstructionAndOcclusionArgs {
    /// Listener game object ID.
    pub listener: GameObjectRef,
    /// Emitter game object ID.
    pub emitter: GameObjectRef,
    /// ObstructionLevel: `[0.0f..1.0f]`.
    #[serde(rename = "obstructionLevel")]
    pub obstruction_level: f64,
    /// OcclusionLevel: `[0.0f..1.0f]`.
    #[serde(rename = "occlusionLevel")]
    pub occlusion_level: f64,
}

impl SoundengineSetObjectObstructionAndOcclusionArgs {
    /// Creates a new [`SoundengineSetObjectObstructionAndOcclusionArgs`] with the required fields.
    pub fn new(emitter: GameObjectRef, listener: GameObjectRef, obstruction_level: f64, occlusion_level: f64) -> Self {
        Self {
            emitter,
            listener,
            obstruction_level,
            occlusion_level,
        }
    }
}
