//! Type-safe wrapper for `ak.wwise.core.object.set`.
//!
//! Schema reference: `tools/waapi_schemas/ak.wwise.core.object.set.json`
//!
//! Schema → Rust mapping:
//!   `argsSchema.required`              → non-`Option` fields
//!   `argsSchema.properties[x].enum`   → enum variants
//!   `$ref objectArg`                   → [`ObjectRef`]
//!   `$ref platformArg`                 → [`PlatformRef`]
//!   `patternProperties "^@..."`        → `Vec<(String, PropertyValue)>`
//!   `optionsSchema objectReturnOptions`→ [`ObjectReturnOptions`]
//!   `resultSchema`                     → [`ObjectSetResponse`]

use std::collections::HashMap;

use super::primitives::{ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, PropertyValue};
use crate::types::{ReturnType, WaapiArgs, WaapiOptions, WaapiValue};
use crate::waapi_client::{WaapiClient, WaapiError, WaapiResponse, WaapiResult};
use crate::waapi_function_api::ak;

// ─────────────────────────────────────────────────────────────────────────────
// Per-object specification (one entry in the top-level `objects` array)
// ─────────────────────────────────────────────────────────────────────────────

/// Describes what to set on a single existing Wwise object.
///
/// Maps to `argsSchema.properties.objects.items`.
/// Only `object` is required by the schema; everything else is optional.
#[derive(Debug)]
pub struct ObjectSetItem {
    /// The target object to modify.
    pub object:           ObjectRef,
    /// Rename the object.
    pub name:             Option<String>,
    /// Set or replace the object's notes/comments.
    pub notes:            Option<String>,
    /// Per-item platform override (takes precedence over the top-level default).
    pub platform:         Option<PlatformRef>,
    /// Per-item name-conflict override.
    pub on_name_conflict: Option<OnNameConflict>,
    /// Per-item list-mode override.
    pub list_mode:        Option<ListMode>,
    /// Child objects to create under this object.
    pub children:         Vec<ChildObject>,
    /// `@PropertyName` fields to set. Keys are bare names without the `@` prefix.
    pub properties:       Vec<(String, PropertyValue)>,
}

impl ObjectSetItem {
    pub fn new(object: ObjectRef) -> Self {
        ObjectSetItem {
            object,
            name:             None,
            notes:            None,
            platform:         None,
            on_name_conflict: None,
            list_mode:        None,
            children:         Vec::new(),
            properties:       Vec::new(),
        }
    }

    pub fn rename(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn platform(mut self, platform: PlatformRef) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn on_name_conflict(mut self, conflict: OnNameConflict) -> Self {
        self.on_name_conflict = Some(conflict);
        self
    }

    pub fn list_mode(mut self, mode: ListMode) -> Self {
        self.list_mode = Some(mode);
        self
    }

    /// Set a property, reference, or list field.
    /// Pass the bare name without the `@` prefix, e.g. `"Volume"` not `"@Volume"`.
    pub fn property(mut self, name: impl Into<String>, value: PropertyValue) -> Self {
        self.properties.push((name.into(), value));
        self
    }

    pub fn child(mut self, child: ChildObject) -> Self {
        self.children.push(child);
        self
    }

    fn to_map(&self) -> HashMap<String, WaapiValue> {
        let mut map = HashMap::new();

        map.insert("object".into(), self.object.to_waapi_value());

        if let Some(name) = &self.name {
            map.insert("name".into(), WaapiValue::String(name.clone()));
        }
        if let Some(notes) = &self.notes {
            map.insert("notes".into(), WaapiValue::String(notes.clone()));
        }
        if let Some(platform) = &self.platform {
            map.insert("platform".into(), platform.to_waapi_value());
        }
        if let Some(conflict) = &self.on_name_conflict {
            map.insert("onNameConflict".into(), WaapiValue::String(conflict.as_str().into()));
        }
        if let Some(mode) = &self.list_mode {
            map.insert("listMode".into(), WaapiValue::String(mode.as_str().into()));
        }
        if !self.children.is_empty() {
            map.insert("children".into(), WaapiValue::List(
                self.children.iter().map(|c| c.to_waapi_value()).collect(),
            ));
        }
        for (key, value) in &self.properties {
            map.insert(format!("@{key}"), value.to_waapi_value());
        }

        map
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Top-level args
// ─────────────────────────────────────────────────────────────────────────────

/// Arguments for `ak.wwise.core.object.set`.
///
/// `objects` is the only field required by the schema.
/// `platform`, `on_name_conflict`, and `list_mode` act as defaults for all
/// items; individual [`ObjectSetItem`]s can override them.
#[derive(Debug)]
pub struct ObjectSetArgs {
    /// The objects to modify. At least one entry required.
    pub objects:                    Vec<ObjectSetItem>,
    /// Default platform for all items.
    pub platform:                   Option<PlatformRef>,
    /// Default name-conflict resolution for all items.
    pub on_name_conflict:           Option<OnNameConflict>,
    /// Default list mode for all items.
    pub list_mode:                  Option<ListMode>,
    /// Whether Wwise auto-adds affected work units to source control.
    /// Defaults to `true` when not specified.
    pub auto_add_to_source_control: Option<bool>,
}

impl ObjectSetArgs {
    pub fn new(objects: Vec<ObjectSetItem>) -> Self {
        ObjectSetArgs {
            objects,
            platform:                   None,
            on_name_conflict:           None,
            list_mode:                  None,
            auto_add_to_source_control: None,
        }
    }

    pub fn platform(mut self, platform: PlatformRef) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn on_name_conflict(mut self, conflict: OnNameConflict) -> Self {
        self.on_name_conflict = Some(conflict);
        self
    }

    pub fn list_mode(mut self, mode: ListMode) -> Self {
        self.list_mode = Some(mode);
        self
    }

    pub fn auto_add_to_source_control(mut self, value: bool) -> Self {
        self.auto_add_to_source_control = Some(value);
        self
    }
}

impl From<ObjectSetArgs> for WaapiArgs {
    fn from(args: ObjectSetArgs) -> WaapiArgs {
        let mut waapi_args = WaapiArgs::new();

        waapi_args.insert_mut("objects", WaapiValue::List(
            args.objects.iter().map(|item| WaapiValue::Dict(item.to_map())).collect(),
        ));

        if let Some(platform) = &args.platform {
            waapi_args.insert_mut("platform", platform.to_waapi_value());
        }
        if let Some(conflict) = &args.on_name_conflict {
            waapi_args.insert_mut("onNameConflict", WaapiValue::String(conflict.as_str().into()));
        }
        if let Some(mode) = &args.list_mode {
            waapi_args.insert_mut("listMode", WaapiValue::String(mode.as_str().into()));
        }
        if let Some(add) = args.auto_add_to_source_control {
            waapi_args.insert_mut("autoAddToSourceControl", WaapiValue::Boolean(add));
        }

        waapi_args
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Return options  (optionsSchema → objectReturnOptions)
// ─────────────────────────────────────────────────────────────────────────────

/// Controls which fields are returned for objects affected by the call.
///
/// Maps to `optionsSchema` → `objectReturnOptions`.
#[derive(Debug, Default)]
pub struct ObjectReturnOptions {
    pub fields:   Vec<ReturnType>,
    pub platform: Option<PlatformRef>,
    pub language: Option<String>,
}

impl ObjectReturnOptions {
    pub fn new(fields: Vec<ReturnType>) -> Self {
        ObjectReturnOptions { fields, platform: None, language: None }
    }

    pub fn platform(mut self, platform: PlatformRef) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
}

impl From<ObjectReturnOptions> for WaapiOptions {
    fn from(opts: ObjectReturnOptions) -> WaapiOptions {
        let mut waapi_opts = WaapiOptions::new(opts.fields);
        if let Some(platform) = opts.platform {
            let s = match platform { PlatformRef::Name(s) | PlatformRef::Guid(s) => s };
            waapi_opts = waapi_opts.set_platform(s);
        }
        if let Some(lang) = opts.language {
            waapi_opts = waapi_opts.set_language(lang);
        }
        waapi_opts
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Response types  (resultSchema)
// ─────────────────────────────────────────────────────────────────────────────

/// An object entry in the response — the modified parent or a newly created
/// child.
///
/// Fields are `Option` because they depend on what was requested via
/// [`ObjectReturnOptions`].
#[derive(Debug)]
pub struct ReturnedObject {
    pub id:       Option<String>,
    pub name:     Option<String>,
    pub children: Vec<ReturnedObject>,
}

/// Response from `ak.wwise.core.object.set`.
///
/// One entry per item in the original `objects` array, each describing the
/// parent and any children that were created.
#[derive(Debug)]
pub struct ObjectSetResponse {
    pub objects: Vec<ReturnedObject>,
}

impl TryFrom<WaapiResponse> for ObjectSetResponse {
    type Error = WaapiError;

    fn try_from(response: WaapiResponse) -> Result<Self, WaapiError> {
        let objects = match response.kwargs.get("objects") {
            Some(WaapiValue::List(list)) => list
                .iter()
                .filter_map(|v| {
                    if let WaapiValue::Dict(map) = v { Some(parse_returned_object(map)) } else { None }
                })
                .collect(),
            _ => Vec::new(),
        };

        Ok(ObjectSetResponse { objects })
    }
}

fn parse_returned_object(map: &HashMap<String, WaapiValue>) -> ReturnedObject {
    let id = match map.get("id") {
        Some(WaapiValue::String(s)) => Some(s.clone()),
        _ => None,
    };
    let name = match map.get("name") {
        Some(WaapiValue::String(s)) => Some(s.clone()),
        _ => None,
    };
    let children = match map.get("children") {
        Some(WaapiValue::List(list)) => list
            .iter()
            .filter_map(|v| {
                if let WaapiValue::Dict(m) = v { Some(parse_returned_object(m)) } else { None }
            })
            .collect(),
        _ => Vec::new(),
    };

    ReturnedObject { id, name, children }
}

// ─────────────────────────────────────────────────────────────────────────────
// Typed method on WaapiClient
// ─────────────────────────────────────────────────────────────────────────────

impl WaapiClient {
    /// Set properties, rename, or create children on one or more existing
    /// Wwise objects in a single call.
    ///
    /// Corresponds to `ak.wwise.core.object.set`.
    pub async fn object_set(
        &mut self,
        args: ObjectSetArgs,
        options: ObjectReturnOptions,
    ) -> WaapiResult<ObjectSetResponse> {
        let response = self
            .call(
                ak::wwise::core::object::set,
                Some(WaapiArgs::from(args)),
                Some(WaapiOptions::from(options)),
            )
            .await?;

        ObjectSetResponse::try_from(response)
    }
}
