// Generated bindings for ak.wwise.core.remote.connect

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.remote.connect`.
///
/// Connects the Wwise Authoring application to a Wwise Sound Engine running executable or to a saved profile file. The host must be running code with communication enabled. If only "host" is provided, Wwise connects to the first Sound Engine instance found. To distinguish between different instances, you can also provide the name of the application to connect to.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreRemoteConnectArgs {
    /// The host to connect to. The host can be a computer name, an IPv4 address, an IP:PORT pair, or a full path to a saved capture (.prof file). Use 127.0.0.1 to connect to localhost.
    pub host: String,
    /// The value in the Application Name column from the Remote Connection dialog in Wwise, or from from ak.wwise.core.remote.getAvailableConsoles. If you are running more than one Sound Engine instance, you can specify the name of the application to connect to.
    #[serde(rename = "appName", skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// The command port. If you are running two or more Sound Engine instances that use the same application name, you can specify the command port to distinguish between different applications sharing the same name. You don't need to use this if the application name is unique. When using this, you must also provide appName. You can obtain this information from ak.wwise.core.remote.getAvailableConsoles.
    #[serde(rename = "commandPort", skip_serializing_if = "Option::is_none")]
    pub command_port: Option<u16>,
    /// Unused.
    #[serde(rename = "notificationPort", skip_serializing_if = "Option::is_none")]
    pub notification_port: Option<u16>,
}

impl CoreRemoteConnectArgs {
    /// Creates a new [`CoreRemoteConnectArgs`] with the required fields.
    pub fn new(host: String) -> Self {
        Self {
            host,
            app_name: None,
            command_port: None,
            notification_port: None,
        }
    }

    /// The value in the Application Name column from the Remote Connection dialog in Wwise, or from from ak.wwise.core.remote.getAvailableConsoles. If you are running more than one Sound Engine instance, you can specify the name of the application to connect to.
    pub fn app_name(mut self, app_name: impl Into<String>) -> Self {
        self.app_name = Some(app_name.into());
        self
    }

    /// The command port. If you are running two or more Sound Engine instances that use the same application name, you can specify the command port to distinguish between different applications sharing the same name. You don't need to use this if the application name is unique. When using this, you must also provide appName. You can obtain this information from ak.wwise.core.remote.getAvailableConsoles.
    pub fn command_port(mut self, command_port: impl Into<u16>) -> Self {
        self.command_port = Some(command_port.into());
        self
    }

    /// Unused.
    pub fn notification_port(mut self, notification_port: impl Into<u16>) -> Self {
        self.notification_port = Some(notification_port.into());
        self
    }
}
