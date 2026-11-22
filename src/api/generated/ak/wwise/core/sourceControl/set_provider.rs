// Generated bindings for ak.wwise.core.sourceControl.setProvider

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.sourceControl.setProvider`.
///
/// Change the source control provider and credentials. This is the same setting as the Source Control option in the Project Settings dialog in Wwise.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreSourceControlSetProviderArgs {
    /// Source control provider. Allowed values ​​are Perforce, Subversion, any other source control supported or an empty string. An empty value is the equivalent of choosing No Control Source in the UI.
    pub provider: String,
    /// Server name or address. Default value is localhost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// Server port. This value is required for Perforce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// User name. This value is required for Perforce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// User password or ticket. This value is required for Perforce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Workspace name. This value is required for Perforce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// Host name. This value is required for Perforce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl CoreSourceControlSetProviderArgs {
    /// Creates a new [`CoreSourceControlSetProviderArgs`] with the required fields.
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            server: None,
            port: None,
            username: None,
            password: None,
            workspace: None,
            host: None,
        }
    }

    /// Server name or address. Default value is localhost.
    pub fn server(mut self, server: impl Into<String>) -> Self {
        self.server = Some(server.into());
        self
    }

    /// Server port. This value is required for Perforce.
    pub fn port(mut self, port: impl Into<String>) -> Self {
        self.port = Some(port.into());
        self
    }

    /// User name. This value is required for Perforce.
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// User password or ticket. This value is required for Perforce.
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Workspace name. This value is required for Perforce.
    pub fn workspace(mut self, workspace: impl Into<String>) -> Self {
        self.workspace = Some(workspace.into());
        self
    }

    /// Host name. This value is required for Perforce.
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
}

impl From<String> for CoreSourceControlSetProviderArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreSourceControlSetProviderArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreSourceControlSetProviderArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Response from `ak.wwise.core.sourceControl.setProvider`.
///
/// Change the source control provider and credentials. This is the same setting as the Source Control option in the Project Settings dialog in Wwise.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreSourceControlSetProviderResponse {
    /// The entries of the log.
    pub log: Vec<LogCentralItem>,
}

/// Module-scoped alias for [`CoreSourceControlSetProviderArgs`].
pub type SetProviderArgs = CoreSourceControlSetProviderArgs;

/// Module-scoped alias for [`CoreSourceControlSetProviderResponse`].
pub type SetProviderResponse = CoreSourceControlSetProviderResponse;
