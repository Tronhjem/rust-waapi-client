// Generated bindings for ak.wwise.core.audio.setConversionPlugin

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audio.setConversionPlugin`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreAudioSetConversionPluginArgs {
    /// The platform to which the settings apply. A platform is an ID (GUID) or a unique name.
    pub platform: PlatformRef,
    /// Name of the plug-in to use for future conversions.
    pub plugin: String,
    /// The object used for conversion. An object is an ID (GUID).
    pub conversion: ObjectRef,
}

impl CoreAudioSetConversionPluginArgs {
    pub fn new(conversion: ObjectRef, platform: PlatformRef, plugin: String) -> Self {
        Self {
            conversion,
            platform,
            plugin,
        }
    }
}
