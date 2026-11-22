// Generated bindings for ak.wwise.core.audio.setConversionPlugin

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.audio.setConversionPlugin`.
///
/// Changes the plug-in to use for audio file conversion.
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
    /// Creates a new [`CoreAudioSetConversionPluginArgs`] with the required fields.
    pub fn new(
        conversion: impl Into<ObjectRef>,
        platform: impl Into<PlatformRef>,
        plugin: impl Into<String>,
    ) -> Self {
        Self {
            conversion: conversion.into(),
            platform: platform.into(),
            plugin: plugin.into(),
        }
    }
}

impl<T0: Into<ObjectRef>, T1: Into<PlatformRef>, T2: Into<String>> From<(T0, T1, T2)>
    for CoreAudioSetConversionPluginArgs
{
    fn from((conversion, platform, plugin): (T0, T1, T2)) -> Self {
        Self::new(conversion, platform, plugin)
    }
}

/// Module-scoped alias for [`CoreAudioSetConversionPluginArgs`].
pub type SetConversionPluginArgs = CoreAudioSetConversionPluginArgs;
