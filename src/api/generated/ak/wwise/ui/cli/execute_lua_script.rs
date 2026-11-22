// Generated bindings for ak.wwise.ui.cli.executeLuaScript

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.ui.cli.executeLuaScript`.
///
/// Execute a Lua script. Optionally, specify additional Lua search paths, additional modules, and additional Lua scripts to load prior to the main script. The script can return a value. All arguments will be passed to the Lua script in the "wa_args" global variable.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UiCliExecuteLuaScriptArgs {
    /// When no project is specified, no project is loaded. See `ak.wwise.console.project.open` for information on how to open a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Specify the file path of the main Lua script to load and execute.
    #[serde(rename = "lua-script")]
    pub lua_script: String,
    /// Specify a search path for Lua modules loaded. This option must be followed by a Lua path. Ex: '--lua-path C:\\module\\?.lua'.
    #[serde(rename = "lua-path", skip_serializing_if = "Option::is_none")]
    pub lua_path: Option<serde_json::Value>,
    /// Specify an array of Lua module to load at runtime using the require system. This option must be followed by a Lua module name. The module name must be found in the Lua path. Ex: '--require debug.lua'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require: Option<serde_json::Value>,
    /// Specify an array of Lua files to load before the main Lua script is loaded and executed. This option must be followed by a absolute file name.
    #[serde(rename = "do-file", skip_serializing_if = "Option::is_none")]
    pub do_file: Option<serde_json::Value>,
    /// Allow migration and save of the Wwise project before executing the operation.
    #[serde(rename = "allow-migration", skip_serializing_if = "Option::is_none")]
    pub allow_migration: Option<bool>,
}

impl UiCliExecuteLuaScriptArgs {
    /// Creates a new [`UiCliExecuteLuaScriptArgs`] with the required fields.
    pub fn new(lua_script: String) -> Self {
        Self {
            lua_script,
            project: None,
            lua_path: None,
            require: None,
            do_file: None,
            allow_migration: None,
        }
    }

    /// When no project is specified, no project is loaded. See `ak.wwise.console.project.open` for information on how to open a project.
    pub fn project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    /// Specify a search path for Lua modules loaded. This option must be followed by a Lua path. Ex: '--lua-path C:\\module\\?.lua'.
    pub fn lua_path(mut self, lua_path: impl Into<serde_json::Value>) -> Self {
        self.lua_path = Some(lua_path.into());
        self
    }

    /// Specify an array of Lua module to load at runtime using the require system. This option must be followed by a Lua module name. The module name must be found in the Lua path. Ex: '--require debug.lua'.
    pub fn require(mut self, require: impl Into<serde_json::Value>) -> Self {
        self.require = Some(require.into());
        self
    }

    /// Specify an array of Lua files to load before the main Lua script is loaded and executed. This option must be followed by a absolute file name.
    pub fn do_file(mut self, do_file: impl Into<serde_json::Value>) -> Self {
        self.do_file = Some(do_file.into());
        self
    }

    /// Allow migration and save of the Wwise project before executing the operation.
    pub fn allow_migration(mut self, allow_migration: impl Into<bool>) -> Self {
        self.allow_migration = Some(allow_migration.into());
        self
    }
}

/// Response from `ak.wwise.ui.cli.executeLuaScript`.
///
/// Execute a Lua script. Optionally, specify additional Lua search paths, additional modules, and additional Lua scripts to load prior to the main script. The script can return a value. All arguments will be passed to the Lua script in the "wa_args" global variable.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiCliExecuteLuaScriptResponse {
    /// Result returned by the Lua script.
    pub result: serde_json::Value,
}
