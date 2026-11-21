// Generated bindings for ak.wwise.ui.commands.execute

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.commands.execute`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiCommandsExecuteArgs {
    /// A value to pass to the command
    /// Some commands can take a value as an argument
    /// Refer to the commands for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<PropertyValue>,
    /// An array of platform
    /// Each platform is an ID (GUID) or a unique name
    /// Some commands can take platforms as arguments
    /// Refer to the commands for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<PlatformRef>>,
    /// The ID of the command to execute
    /// Refer to for the lists of commands.
    pub command: String,
    /// An array of objects
    /// Each object is an ID (GUID), name, or path of the object
    /// Some commands can take objects as arguments
    /// Refer to the commands for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<ObjectRef>>,
}

impl UiCommandsExecuteArgs {
    pub fn new(command: String) -> Self {
        Self {
            command,
            value: None,
            platforms: None,
            objects: None,
        }
    }

    pub fn value(mut self, value: impl Into<PropertyValue>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn platforms(mut self, platforms: impl Into<Vec<PlatformRef>>) -> Self {
        self.platforms = Some(platforms.into());
        self
    }

    pub fn objects(mut self, objects: impl Into<Vec<ObjectRef>>) -> Self {
        self.objects = Some(objects.into());
        self
    }
}
