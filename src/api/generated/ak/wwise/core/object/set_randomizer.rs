// Generated bindings for ak.wwise.core.object.setRandomizer

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.object.setRandomizer`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreObjectSetRandomizerArgs {
    /// Minimum value that the randomizer can offset by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// Maximum value that the randomizer can offset by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// The ID (GUID), name, or path of the object owning the property.
    pub object: ObjectRef,
    /// The name of the property. For more information, refer to the property list of Wwise Objects listed in .
    pub property: Property,
    /// The ID (GUID) or unique name of the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    /// Enabled state of the randomizer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl CoreObjectSetRandomizerArgs {
    pub fn new(object: ObjectRef, property: Property) -> Self {
        Self {
            object,
            property,
            min: None,
            max: None,
            platform: None,
            enabled: None,
        }
    }

    pub fn min(mut self, min: impl Into<f64>) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max(mut self, max: impl Into<f64>) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
}
