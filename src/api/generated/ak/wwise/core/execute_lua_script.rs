// Generated bindings for ak.wwise.core.executeLuaScript

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.executeLuaScript`.
///
/// Execute Lua code from a file path (using `luaScript`) or directly as a string (using `luaString`). Optionally, specify additional Lua search paths, additional modules, and additional Lua scripts to load prior to the main script. The script can return a value, which can be any of the Lua types, including tables. All WAAPI arguments will be passed to the Lua script in the `wa_args` global variable, as a Lua table. Define your own key-values in the WAAPI arguments and access them from the Lua code. Use `wa_call` to call a WAAPI function from the Lua script. See `ak.wwise.waapi.getFunctions` and `ak.wwise.waapi.getSchema` to learn more about the functions available and their schema before using `wa_call`. If both `luaScript` and `luaString` are provided, the script file will be executed first, followed by the string code. Note that the Lua environment is reset at each call to this function.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CoreExecuteLuaScriptArgs {
    /// Defines the main Lua file path to load and execute. Use this when your Lua code is stored in a file. Use an absolute file path. When this is used, the directory containing the script is automatically added to the Lua module search path, allowing the script to use `require` to load additional modules from that directory.
    #[serde(rename = "luaScript", skip_serializing_if = "Option::is_none")]
    pub lua_script: Option<String>,
    /// Defines the main Lua code to execute as a string. Use this when you want to execute Lua code directly without creating a file, which is useful for simple scripts, dynamic code generation, or inline execution. The code can span multiple lines and can use `require` to load modules from the configured Lua paths. This parameter is mutually optional with `luaScript` - you can use either one or both. Note: Unlike `luaScript`, using `luaString` does not automatically modify the Lua module search path (`package.path`). If you need to load modules relative to a specific directory, use `luaPaths` to explicitly add search paths.
    #[serde(rename = "luaString", skip_serializing_if = "Option::is_none")]
    pub lua_string: Option<String>,
    /// Defines an array of paths to be used to search additional Lua modules using the `require` system. Each path should contain a ? placeholder that will be replaced with the module name. Example: `C:/path_to_folder/?.lua`. These paths are added to the Lua `package.path` before the main script or string is executed.
    #[serde(rename = "luaPaths", skip_serializing_if = "Option::is_none")]
    pub lua_paths: Option<Vec<String>>,
    /// Defines additional modules to be loaded at runtime using the `require` system. Each module will be loaded before the main script or string is executed. Note that the following folders are automatically added to the Lua module search path: `PROJECT/Add-ons/Lua`, `APPDATA/Audiokinetic/Wwise/Add-ons/Lua`, `INSTALLDIR/Authoring/Data/Add-ons/Lua`. You can place custom modules in these directories and reference them here by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<Vec<String>>,
    /// Defines additional Lua files or directories to load before the main Lua script or string is executed. If a file path is provided, that specific file will be executed. If a directory path is provided, all `.lua` files in that directory and its subdirectories will be loaded recursively. This is useful for loading helper functions, configuration, or setup code that the main script depends on.
    #[serde(rename = "doFiles", skip_serializing_if = "Option::is_none")]
    pub do_files: Option<Vec<String>>,
}

impl CoreExecuteLuaScriptArgs {
    /// Creates a new [`CoreExecuteLuaScriptArgs`] with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines the main Lua file path to load and execute. Use this when your Lua code is stored in a file. Use an absolute file path. When this is used, the directory containing the script is automatically added to the Lua module search path, allowing the script to use `require` to load additional modules from that directory.
    pub fn lua_script(mut self, lua_script: impl Into<String>) -> Self {
        self.lua_script = Some(lua_script.into());
        self
    }

    /// Defines the main Lua code to execute as a string. Use this when you want to execute Lua code directly without creating a file, which is useful for simple scripts, dynamic code generation, or inline execution. The code can span multiple lines and can use `require` to load modules from the configured Lua paths. This parameter is mutually optional with `luaScript` - you can use either one or both. Note: Unlike `luaScript`, using `luaString` does not automatically modify the Lua module search path (`package.path`). If you need to load modules relative to a specific directory, use `luaPaths` to explicitly add search paths.
    pub fn lua_string(mut self, lua_string: impl Into<String>) -> Self {
        self.lua_string = Some(lua_string.into());
        self
    }

    /// Defines an array of paths to be used to search additional Lua modules using the `require` system. Each path should contain a ? placeholder that will be replaced with the module name. Example: `C:/path_to_folder/?.lua`. These paths are added to the Lua `package.path` before the main script or string is executed.
    pub fn lua_paths(mut self, lua_paths: impl Into<Vec<String>>) -> Self {
        self.lua_paths = Some(lua_paths.into());
        self
    }

    /// Defines additional modules to be loaded at runtime using the `require` system. Each module will be loaded before the main script or string is executed. Note that the following folders are automatically added to the Lua module search path: `PROJECT/Add-ons/Lua`, `APPDATA/Audiokinetic/Wwise/Add-ons/Lua`, `INSTALLDIR/Authoring/Data/Add-ons/Lua`. You can place custom modules in these directories and reference them here by name.
    pub fn requires(mut self, requires: impl Into<Vec<String>>) -> Self {
        self.requires = Some(requires.into());
        self
    }

    /// Defines additional Lua files or directories to load before the main Lua script or string is executed. If a file path is provided, that specific file will be executed. If a directory path is provided, all `.lua` files in that directory and its subdirectories will be loaded recursively. This is useful for loading helper functions, configuration, or setup code that the main script depends on.
    pub fn do_files(mut self, do_files: impl Into<Vec<String>>) -> Self {
        self.do_files = Some(do_files.into());
        self
    }
}

/// Response from `ak.wwise.core.executeLuaScript`.
///
/// Execute Lua code from a file path (using `luaScript`) or directly as a string (using `luaString`). Optionally, specify additional Lua search paths, additional modules, and additional Lua scripts to load prior to the main script. The script can return a value, which can be any of the Lua types, including tables. All WAAPI arguments will be passed to the Lua script in the `wa_args` global variable, as a Lua table. Define your own key-values in the WAAPI arguments and access them from the Lua code. Use `wa_call` to call a WAAPI function from the Lua script. See `ak.wwise.waapi.getFunctions` and `ak.wwise.waapi.getSchema` to learn more about the functions available and their schema before using `wa_call`. If both `luaScript` and `luaString` are provided, the script file will be executed first, followed by the string code. Note that the Lua environment is reset at each call to this function.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoreExecuteLuaScriptResponse {
    /// Result returned by the Lua script. Use a return statement at the end of the script. In the case of a script that does not return any value, this field will be false.
    #[serde(rename = "return")]
    pub return_: serde_json::Value,
}
