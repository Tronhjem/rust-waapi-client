# wwise-waapi

[![Docs (GitHub Pages)](https://img.shields.io/badge/docs-github%20pages-blue)](https://tronhjem.github.io/rust-waapi-client/wwise_waapi/)
[![docs.rs](https://docs.rs/wwise-waapi/badge.svg)](https://docs.rs/wwise-waapi)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Async, typed Rust client for the Wwise Authoring API (WAAPI).
- **API Documentation (Rustdoc):** https://tronhjem.github.io/rust-waapi-client/wwise_waapi/
- **Wwise WAAPI Reference:** https://www.audiokinetic.com/en/public-library/2025.1.4_9062/?source=SDK&id=waapi.html

This crate ships with a small, private WAMP v2 client that speaks JSON over WebSocket. It is no longer dependent on an external `wampire` submodule.

## Basic Usage

`wwise-waapi` is async and built on `tokio`. Import `wwise_waapi::prelude::*` for the most common client, query, object, property, and SoundEngine types.

### Connecting & Calling the Typed API

Typed endpoints are exposed via a **chained namespace API** on `WaapiClient` that mirrors WAAPI URI dot-notation (`client.ak().wwise()...`, plus top-level shortcuts `client.wwise()`, `client.core()`, `client.object()`, `client.ui()`, and `client.soundengine()`). All client and namespace handles take `&self` (`Copy`), so they can be stored and reused freely:

```rust
use wwise_waapi::api::ak::wwise::core::object::{CreateArgs, GetOptions};
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    // Connect to WAAPI (default: ws://127.0.0.1:8080/waapi)
    let client = WaapiClient::connect().await?;

    let info = client.core().get_info().await?;
    println!("Connected to {} ({})", info.display_name, info.version.display_name);

    // Query objects by path, GUID, ObjectType, or ObjectQuery.
    // Options accepts GetOptions (or ObjectReturnOptions::basic(), arrays of ObjectReturnField, or ()).
    let response = client
        .object()
        .get(
            r"\Containers\Default Work Unit",
            GetOptions::new().return_fields([
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Type,
                ObjectReturnField::Property(Property::Volume),
            ]),
        )
        .await?;

    for obj in &response.objects {
        println!(
            "{:?} ({:?}): id={:?} volume={:?}",
            obj.name(),
            obj.object_type(),
            obj.id(),
            obj.property_f64(Property::Volume),
        );
    }

    // Create a Sound object
    let created = client
        .object()
        .create(
            CreateArgs::new(
                r"\Containers\Default Work Unit",
                ObjectType::Sound,
                "MyNewSound",
            )
            .on_name_conflict(OnNameConflict::Replace)
            .property(Property::Volume, -6.0),
        )
        .await?;
    println!("Created sound id: {:?}", created.id());

    // Always shutdown when done
    client.shutdown().await?;
    Ok(())
}
```

Use `WaapiClient::connect()` for the default localhost endpoint (`ws://127.0.0.1:8080/waapi`), `WaapiClient::connect_to("127.0.0.1:8080")` for a custom host/port or WebSocket URL, or `WaapiClient::new(Option<&str>)`.

### Setting Object Properties

See `examples/object_set.rs` for setting properties on Wwise objects using the generated builders:

```rust
use wwise_waapi::api::ak::wwise::core::object::{SetItem, SetOptions};
use wwise_waapi::prelude::*;

let client = WaapiClient::connect().await?;

let item = SetItem::new(r"\Containers\Default Work Unit\MySound")
    .name("MyRenamedSound")
    .property(Property::Volume, -6.0);

let options = SetOptions::new().return_fields([
    ObjectReturnField::Id,
    ObjectReturnField::Name,
    ObjectReturnField::Property(Property::Volume),
]);

let response = client.object().set(item, options).await?;
client.shutdown().await?;
```

### Typed Subscriptions

Subscribe to WAAPI topics using the typed subscription methods (`subscribe_*`):

```rust
use wwise_waapi::api::ak::wwise::core::object::CreatedOptions;
use wwise_waapi::prelude::*;

let options = CreatedOptions::new()
    .return_fields([ObjectReturnField::Id, ObjectReturnField::Name]);
let (id, mut events) = client
    .object()
    .subscribe_created(Some(options))
    .await?;

while let Some(payload) = events.recv().await {
    println!(
        "Created: id={:?} name={:?}",
        payload.object.id(),
        payload.object.name()
    );
}

client.unsubscribe(id).await?;
```

### Raw JSON Escape Hatch

For dynamic queries or custom untyped payloads, use `client.call(WaapiFunction, args, options)` with `serde_json::Value` (or `call_raw` if you need to pass an arbitrary `&str` URI). The `json!` macro is re-exported from the crate root, so applications can build JSON kwargs without adding a direct `serde_json` dependency:

```rust
use wwise_waapi::{WaapiFunction, json};

let args = json!({
    "from": {
        "path": [r"\Containers\Default Work Unit"]
    }
});

let options = json!({
    "return": ["id", "name", "type"]
});

let response = client
    .call(WaapiFunction::ak_wwise_core_object_get, Some(args), Some(options))
    .await?;
println!("{}", wwise_waapi::serde_json::to_string_pretty(&response)?);
```

---

## Typed API Design & JSON Schema Mapping

All generated bindings in `src/waapi_function_api.rs`, `src/waapi_topics_api.rs`, and `src/api/generated/` are derived offline from the Wwise JSON schemas stored in `tools/waapi_schemas/<version>/` (fetched from a running Wwise instance via `tools/get_waapi_definitions.py`).

### 1. URI-to-Chain and Module Mapping

Every WAAPI URI is split on `.`:
- The **prefix segments** map 1-to-1 to both the Rust module path (`wwise_waapi::api::generated::ak::...`) and zero-cost chained borrower structs (`AkClient<'a>`, `WwiseClient<'a>`, `WwiseCoreClient<'a>`, `WwiseCoreObjectClient<'a>`, etc.) in `src/api/generated/clients.rs`.
- The **final segment** becomes the `snake_case` async method on that namespace client.
- `client.wwise()`, `client.core()`, `client.object()`, `client.ui()`, and `client.soundengine()` are provided on `WaapiClient` as convenient shortcuts for `client.ak().wwise()`, `client.wwise().core()`, `client.wwise().core().object()`, `client.wwise().ui()`, and `client.ak().soundengine()`.

| WAAPI URI | Chained Call | Module Path |
|---|---|---|
| `ak.wwise.core.getInfo` | `client.core().get_info()` / `client.wwise().core().get_info()` | `api::ak::wwise::core` |
| `ak.wwise.core.object.get` | `client.object().get(args, options)` / `client.wwise().core().object().get(args, options)` | `api::ak::wwise::core::object` |
| `ak.soundengine.postEvent` | `client.soundengine().post_event(args)` | `api::ak::soundengine` |

### 2. Type Naming Convention (`<Base>{Args,Options,Response}` and Short Module Aliases)

For each endpoint URI, the generator strips the leading `ak.wwise.` prefix (or `ak.` for non-`wwise` namespaces like `ak.soundengine`), splits camelCase words, joins all remaining segments in `PascalCase` as `<Base>`, and emits up to three types along with short module-scoped aliases based on the final URI segment:
- `<Base>Args` (alias `<Leaf>Args`) — request keyword arguments (`ArgumentsKw`)
- `<Base>Options` (alias `<Leaf>Options`) — request WAMP call/subscribe options (`Options`)
- `<Base>Response` (alias `<Leaf>Response`) — deserialized response payload
- `<Base>Payload` (alias `<Leaf>Payload`) — deserialized subscription topic payload

For example:
- `ak.wwise.core.getInfo` → `CoreGetInfoResponse` (or `core::GetInfoResponse`)
- `ak.wwise.core.object.get` → `CoreObjectGetArgs` / `object::GetArgs`, `CoreObjectGetOptions` / `object::GetOptions`, `CoreObjectGetResponse` / `object::GetResponse`
- `ak.wwise.core.object.set` → `CoreObjectSetArgs` / `object::SetArgs`, `CoreObjectSetItem` / `object::SetItem`, `CoreObjectSetOptions` / `object::SetOptions`, `CoreObjectSetResponse` / `object::SetResponse`
- `ak.wwise.ui.getSelectedObjects` → `UiGetSelectedObjectsOptions` / `ui::GetSelectedObjectsOptions`, `UiGetSelectedObjectsResponse` / `ui::GetSelectedObjectsResponse`
- `ak.soundengine.setRTPCValue` → `SoundengineSetRtpcValueArgs` / `soundengine::SetRtpcValueArgs`

Constructors (`Args::new(...)`) accept `impl Into<T>` for string and reference types (`ObjectRef`, `GameObjectRef`, `SoundEngineObjectRef`, `PlatformRef`, `LanguageRef`, `PropertyValue`, `ObjectQuery`), and client methods accept `impl Into<Args>` (including direct paths/GUIDs for 1-field `Args` like `delete` / `get`, a single `SetItem` for `set`, and tuples for 2/3-field `Args`).

### 3. How Wwise JSON Schema Sections Map to Rust

Each `tools/waapi_schemas/<version>/ak.*.json` file contains either RPC function schemas (`argsSchema`, `optionsSchema`, `resultSchema`) or topic schemas (`publishSchema`, `optionsSchema`):

| JSON Schema Section | Rust Output | Rules |
|---|---|---|
| `argsSchema` | `<Base>Args` (`#[derive(Serialize)]`) | **Required properties** (`required` array) become positional parameters on `<Base>Args::new(...)`. **Optional properties** become `Option<T>` fields with `#[serde(skip_serializing_if = "Option::is_none")]` and chainable builder methods `.field(val)`. If `argsSchema` has no properties, no `<Base>Args` type is emitted and the client method takes no `args` parameter. |
| `optionsSchema` | `<Base>Options` (`#[derive(Serialize)]`) | Generated with `::new()` (`Default`) and chainable builder methods. When `optionsSchema` is `{"$ref": "waapi_definitions.json#/definitions/objectReturnOptions"}`, `<Base>Options` is emitted as `pub type <Base>Options = ObjectReturnOptions;`. If `optionsSchema` is empty, no `<Base>Options` type is emitted and the client method takes no `options` parameter. |
| `resultSchema` | `<Base>Response` (`#[derive(Serialize, Deserialize)]`) | Response fields are generated from `resultSchema.properties`. When present, the chained method calls `WaapiClient::call_typed` and returns `WaapiResult<<Base>Response>`. When `resultSchema` has no properties, no `<Base>Response` is emitted and the method calls `WaapiClient::call_typed_void`, returning `WaapiResult<()>`. |
| `publishSchema` | `WaapiTopic` + topic event payload | Identifies the schema file as a **subscription topic** (35 topics) rather than an RPC function (180 functions). `tools/generate_waapi_uri.py` emits it into `WaapiTopic` (`src/waapi_topics_api.rs`). Subscriptions currently use `client.subscribe(WaapiTopic::..., ...)` or `client.subscribe_typed(WaapiTopic::..., Some(options))`, and `publishSchema` + `optionsSchema` define the schema for typed topic subscriptions. |

### 4. Shared `$ref` Definitions, Return Fields, and `@Property` Extensions

- **`Definitions/*.json` `$ref` resolution**: Shared references across `common_definitions.json`, `waapi_definitions.json`, `waapi_ui_definitions.json`, and `wafm_definitions.json` map to shared primitives in `src/api/primitives.rs` and `src/api/generated/primitives.rs` (`ObjectRef`, `PlatformRef`, `SoundEngineObjectRef`, `GameObjectRef`, `LanguageRef`, `WwiseObject`, `WwiseObjectRef`, `OnNameConflict`, `ListMode`, `ImportOperation`, `LogSeverity`, `PropertyValue`, `ChildObject`).
- **Endpoint-domain enums and structs**: Schemas with inline string enums or nested objects map to generated types in `src/api/generated/primitives.rs`, including `TransportAction` / `TransportState` / `TransportObject` (`transport.*`), `AttenuationCurveType` / `AttenuationCurveUse` / `CurvePoint2D` / `CurveShape` (`object.{get,set}AttenuationCurve`), `PasteMode` (`object.pasteProperties`), `SwitchContainerAssignment` (`switchContainer.getAssignments`), `ProjectMigrationAction` (`ui.project.open`), and `ProjectPlatformInfo` / `ProjectLanguageInfo` / `ProjectDirectoriesInfo` (`core.getProjectInfo`).
- **Domain-specific return field enums**: Rather than untyped strings, `waapi_definitions.json` return expressions are generated as strongly-typed enums:
  - `ObjectReturnField` (wrapped by `ObjectReturnOptions`) — all built-in Wwise object accessors (`Id`, `Name`, `Notes`, `Type`, `Path`, `Parent`, `ShortId`, etc.), `ObjectReturnField::Property(Property)` (which serializes as `"@Property"`), plus `ObjectReturnField::Custom(String)` (and `From<&str>` / `From<String>`) for arbitrary dot-accessor queries.
  - `BusReturnField`, `AudioObjectReturnField`, and `VoiceReturnField` for profiler queries.
- **Generated `ObjectType` enum (`WObjects.xml`)**: `src/api/generated/object_types.rs` (re-exported as `wwise_waapi::ObjectType` and `wwise_waapi::api::ObjectType`) is generated from the `<WwiseObject>` definitions in `Definitions/WObjects.xml` and contains all Wwise object types (e.g. `ObjectType::Sound`, `ObjectType::ActorMixer`/`ObjectType::PropertyContainer`, `ObjectType::Event`, `ObjectType::Folder`, `ObjectType::RandomSequenceContainer`). It is used in `CoreObjectCreateArgs`, `ChildObject`, `WwiseObject`, `ObjectQuery::OfType`, `ImportItem`, and `ImportDefaults`, and provides `as_str()`, `class_id()`, `from_class_id()`, `FromStr`, `AsRef<str>`, `Display`, `Serialize`, and `Deserialize`. The union of Wwise 2024 + 2025 `WObjects.xml` keeps `ActorMixer` (2024) and `PropertyContainer` (2025) as distinct variants sharing `class_id()` `524304`.
- **Generated `Property` enum (`WObjects.xml`)**: `src/api/generated/properties.rs` (re-exported as `wwise_waapi::Property` and `wwise_waapi::api::Property`) is generated from `Definitions/WObjects.xml` and contains all 464 Wwise object properties, references, and object lists (e.g. `Property::Volume`, `Property::Pitch`, `Property::Lowpass`, `Property::OutputBus`, `Property::RTPC`). `Property::as_str()` returns the bare `&'static str` (`"Volume"`), `Property::as_at_str()` returns the `@`-prefixed `&'static str` (`"@Volume"`), and `Property` implements `AsRef<str>`, `Display`, `Serialize`, and `Deserialize`.
- **Dynamic `@Property` fields (`patternProperties`)**: Schemas that accept or return open-ended `@`-prefixed properties (such as `CoreObjectSetItem`, `CoreObjectCreateArgs`, `ImportItem`, `ImportDefaults`, `WwiseObject`, and `CoreObjectSetObject`) include `#[serde(flatten)] pub extra: HashMap<String, serde_json::Value>` along with `.property(Property::Volume, value)` / `.extra(key, value)` builder methods on request structs and `.property(Property::Volume) -> Option<&serde_json::Value>` lookup helpers on response structs.
- **Field naming**: JSON `camelCase` wire names are converted to Rust `snake_case` with `#[serde(rename = "...")]` emitted automatically whenever the wire name differs (e.g. `type` → `object_type`, `return` → `objects` / `playing_id` / `return_fields`).

### 5. Generated Typed Endpoints (180 Functions & 35 Topics)

All **180 WAAPI functions** and **35 subscription topics** across the union of `tools/waapi_schemas/<version>/` (currently 2024.1.7 and 2025.1.12) are automatically discovered and generated into typed structs, chained call methods, and typed `subscribe_*` methods under `wwise_waapi::api::generated` (configured via `tools/overrides.json` and `tools/generator/`).

> **Multi-version union & optionality.** The bindings are generated from the *union* of all Wwise schema versions under `tools/waapi_schemas/`, so a single API serves both Wwise 2024 and Wwise 2025+. Renamed object types are kept as aliases (`ObjectType::ActorMixer` and `ObjectType::PropertyContainer` share `class_id()` `524304`, with `PropertyContainer` as the canonical `from_class_id` result), and 2024-only return fields (`ObjectReturnField::ColorRgb`/`TransitionObject`, `AudioObjectReturnField::PrevGain`/`NextGain`) are retained. Response fields on `WwiseObject` are `Option<T>` because WAAPI only populates the return fields requested via `ObjectReturnOptions` (and so older/newer Wwise versions remain compatible); `WwiseObject` provides `.id()`, `.name()`, `.object_type()`, `.path()`, `.notes()`, `.property(...)`, `.property_f64(...)`, `.property_i64(...)`, `.property_bool(...)`, and `.property_str(...)` convenience accessors. Intermediate chained client handles (`let wwise = client.wwise();`) take `&self` (`Copy`) and can be reused across multiple calls.

Representative endpoints include (every remaining namespace — `profiler.*`, `remote.*`, `sourceControl.*`, `waapi.*`, `log.*`, `audioSourcePeaks.*`, `debug.*`, `ui.layout.*`, `ui.window.*`, `ui.model.*`, `ui.cli.*` — is also generated under `wwise_waapi::api::generated::ak`):

| WAAPI URI | Chained Method on `WaapiClient` | `Args` Type | `Options` Type | `Response` Type |
|---|---|---|---|---|
| `ak.wwise.core.getInfo` | `client.wwise().core().get_info()` | — | — | `CoreGetInfoResponse` |
| `ak.wwise.core.getProjectInfo` | `client.wwise().core().get_project_info()` | — | — | `CoreGetProjectInfoResponse` |
| `ak.wwise.core.project.save` | `client.wwise().core().project().save(args)` | `CoreProjectSaveArgs` | — | `()` |
| `ak.wwise.core.object.get` | `client.wwise().core().object().get(args, options)` | `CoreObjectGetArgs` | `CoreObjectGetOptions` | `CoreObjectGetResponse` |
| `ak.wwise.core.object.set` | `client.wwise().core().object().set(args, options)` | `CoreObjectSetArgs` + `CoreObjectSetItem` | `CoreObjectSetOptions` | `CoreObjectSetResponse` + `CoreObjectSetObject` |
| `ak.wwise.core.object.create` | `client.wwise().core().object().create(args, options)` | `CoreObjectCreateArgs` | `CoreObjectCreateOptions` | `CoreObjectCreateResponse` |
| `ak.wwise.core.object.delete` | `client.wwise().core().object().delete(args)` | `CoreObjectDeleteArgs` | — | `()` |
| `ak.wwise.core.object.move` | `client.wwise().core().object().move_(args)` | `CoreObjectMoveArgs` | — | `CoreObjectMoveResponse` (`WwiseObject`) |
| `ak.wwise.core.object.copy` | `client.wwise().core().object().copy(args, options)` | `CoreObjectCopyArgs` | `CoreObjectCopyOptions` | `CoreObjectCopyResponse` (`WwiseObject`) |
| `ak.wwise.core.object.setName` | `client.wwise().core().object().set_name(args)` | `CoreObjectSetNameArgs` | — | `()` |
| `ak.wwise.core.object.setNotes` | `client.wwise().core().object().set_notes(args)` | `CoreObjectSetNotesArgs` | — | `()` |
| `ak.wwise.core.object.setProperty` | `client.wwise().core().object().set_property(args)` | `CoreObjectSetPropertyArgs` | — | `()` |
| `ak.wwise.core.object.setReference` | `client.wwise().core().object().set_reference(args)` | `CoreObjectSetReferenceArgs` | — | `()` |
| `ak.wwise.core.object.setRandomizer` | `client.wwise().core().object().set_randomizer(args)` | `CoreObjectSetRandomizerArgs` | — | `()` |
| `ak.wwise.core.object.getAttenuationCurve` | `client.wwise().core().object().get_attenuation_curve(args)` | `CoreObjectGetAttenuationCurveArgs` | — | `CoreObjectGetAttenuationCurveResponse` |
| `ak.wwise.core.object.setAttenuationCurve` | `client.wwise().core().object().set_attenuation_curve(args)` | `CoreObjectSetAttenuationCurveArgs` | — | `()` |
| `ak.wwise.core.object.diff` | `client.wwise().core().object().diff(args)` | `CoreObjectDiffArgs` | — | `CoreObjectDiffResponse` |
| `ak.wwise.core.object.pasteProperties` | `client.wwise().core().object().paste_properties(args)` | `CoreObjectPastePropertiesArgs` | — | `()` |
| `ak.wwise.core.switchContainer.addAssignment` | `client.wwise().core().switch_container().add_assignment(args)` | `CoreSwitchContainerAddAssignmentArgs` | — | `()` |
| `ak.wwise.core.switchContainer.getAssignments` | `client.wwise().core().switch_container().get_assignments(args)` | `CoreSwitchContainerGetAssignmentsArgs` | — | `CoreSwitchContainerGetAssignmentsResponse` |
| `ak.wwise.core.switchContainer.removeAssignment` | `client.wwise().core().switch_container().remove_assignment(args)` | `CoreSwitchContainerRemoveAssignmentArgs` | — | `()` |
| `ak.wwise.core.audio.import` | `client.wwise().core().audio().import(args, options)` | `CoreAudioImportArgs` (`ImportItem`, `ImportDefaults`) | `CoreAudioImportOptions` | `CoreAudioImportResponse` (`ImportLogEntry`) |
| `ak.wwise.core.transport.create` | `client.wwise().core().transport().create(args)` | `CoreTransportCreateArgs` | — | `CoreTransportCreateResponse` |
| `ak.wwise.core.transport.destroy` | `client.wwise().core().transport().destroy(args)` | `CoreTransportDestroyArgs` | — | `()` |
| `ak.wwise.core.transport.executeAction` | `client.wwise().core().transport().execute_action(args)` | `CoreTransportExecuteActionArgs` (`TransportAction`) | — | `()` |
| `ak.wwise.core.transport.getState` | `client.wwise().core().transport().get_state(args)` | `CoreTransportGetStateArgs` | — | `CoreTransportGetStateResponse` (`TransportState`) |
| `ak.wwise.core.transport.getList` | `client.wwise().core().transport().get_list()` | — | — | `CoreTransportGetListResponse` (`TransportObject`) |
| `ak.wwise.core.transport.prepare` | `client.wwise().core().transport().prepare(args)` | `CoreTransportPrepareArgs` | — | `()` |
| `ak.wwise.core.transport.useOriginals` | `client.wwise().core().transport().use_originals(args)` | `CoreTransportUseOriginalsArgs` | — | `()` |
| `ak.wwise.core.undo.beginGroup` | `client.wwise().core().undo().begin_group()` | — | — | `()` |
| `ak.wwise.core.undo.endGroup` | `client.wwise().core().undo().end_group(args)` | `CoreUndoEndGroupArgs` | — | `()` |
| `ak.wwise.core.undo.cancelGroup` | `client.wwise().core().undo().cancel_group(args)` | `CoreUndoCancelGroupArgs` | — | `()` |
| `ak.wwise.core.undo.undo` | `client.wwise().core().undo().undo()` | — | — | `()` |
| `ak.wwise.core.undo.redo` | `client.wwise().core().undo().redo()` | — | — | `()` |
| `ak.wwise.ui.getSelectedObjects` | `client.wwise().ui().get_selected_objects(options)` | — | `UiGetSelectedObjectsOptions` | `UiGetSelectedObjectsResponse` |
| `ak.wwise.ui.bringToForeground` | `client.wwise().ui().bring_to_foreground()` | — | — | `()` |
| `ak.wwise.ui.commands.execute` | `client.wwise().ui().commands().execute(args)` | `UiCommandsExecuteArgs` | — | `()` |
| `ak.wwise.ui.commands.getCommands` | `client.wwise().ui().commands().get_commands()` | — | — | `UiCommandsGetCommandsResponse` |
| `ak.wwise.ui.project.open` | `client.wwise().ui().project().open(args)` | `UiProjectOpenArgs` (`ProjectMigrationAction`) | — | `()` |
| `ak.wwise.ui.project.close` | `client.wwise().ui().project().close(args)` | `UiProjectCloseArgs` | — | `UiProjectCloseResponse` |
| `ak.soundengine.postEvent` | `client.soundengine().post_event(args)` | `SoundenginePostEventArgs` | — | `SoundenginePostEventResponse` |
| `ak.soundengine.setRTPCValue` | `client.soundengine().set_rtpc_value(args)` | `SoundengineSetRtpcValueArgs` | — | `()` |
| `ak.soundengine.registerGameObj` | `client.soundengine().register_game_obj(args)` | `SoundengineRegisterGameObjArgs` | — | `()` |
| `ak.soundengine.unregisterGameObj` | `client.soundengine().unregister_game_obj(args)` | `SoundengineUnregisterGameObjArgs` | — | `()` |
| `ak.soundengine.setPosition` | `client.soundengine().set_position(args)` | `SoundengineSetPositionArgs` (`Position`, `Vector3`) | — | `()` |
| `ak.soundengine.setDefaultListeners` | `client.soundengine().set_default_listeners(args)` | `SoundengineSetDefaultListenersArgs` | — | `()` |
| `ak.soundengine.setSwitch` | `client.soundengine().set_switch(args)` | `SoundengineSetSwitchArgs` | — | `()` |
| `ak.soundengine.setState` | `client.soundengine().set_state(args)` | `SoundengineSetStateArgs` | — | `()` |
| `ak.soundengine.getSwitch` | `client.soundengine().get_switch(args, options)` | `SoundengineGetSwitchArgs` | `SoundengineGetSwitchOptions` | `SoundengineGetSwitchResponse` (`WwiseObject`) |
| `ak.soundengine.getState` | `client.soundengine().get_state(args, options)` | `SoundengineGetStateArgs` | `SoundengineGetStateOptions` | `SoundengineGetStateResponse` (`WwiseObject`) |
| `ak.soundengine.loadBank` | `client.soundengine().load_bank(args)` | `SoundengineLoadBankArgs` | — | `()` |
| `ak.soundengine.unloadBank` | `client.soundengine().unload_bank(args)` | `SoundengineUnloadBankArgs` | — | `()` |
| `ak.soundengine.stopAll` | `client.soundengine().stop_all(args)` | `SoundengineStopAllArgs` | — | `()` |
| `ak.soundengine.stopPlayingID` | `client.soundengine().stop_playing_id(args)` | `SoundengineStopPlayingIdArgs` (`AkCurveInterpolation`) | — | `()` |
| `ak.soundengine.seekOnEvent` | `client.soundengine().seek_on_event(args)` | `SoundengineSeekOnEventArgs` | — | `()` |
| `ak.soundengine.setListeners` | `client.soundengine().set_listeners(args)` | `SoundengineSetListenersArgs` | — | `()` |
| `ak.soundengine.setListenerSpatialization` | `client.soundengine().set_listener_spatialization(args)` | `SoundengineSetListenerSpatializationArgs` | — | `()` |
| `ak.soundengine.setMultiplePositions` | `client.soundengine().set_multiple_positions(args)` | `SoundengineSetMultiplePositionsArgs` (`MultiPosition`, `MultiPositionType`) | — | `()` |
| `ak.soundengine.setGameObjectAuxSendValues` | `client.soundengine().set_game_object_aux_send_values(args)` | `SoundengineSetGameObjectAuxSendValuesArgs` (`AuxSendValue`) | — | `()` |
| `ak.soundengine.setGameObjectOutputBusVolume` | `client.soundengine().set_game_object_output_bus_volume(args)` | `SoundengineSetGameObjectOutputBusVolumeArgs` | — | `()` |
| `ak.soundengine.setObjectObstructionAndOcclusion` | `client.soundengine().set_object_obstruction_and_occlusion(args)` | `SoundengineSetObjectObstructionAndOcclusionArgs` | — | `()` |
| `ak.soundengine.setScalingFactor` | `client.soundengine().set_scaling_factor(args)` | `SoundengineSetScalingFactorArgs` | — | `()` |
| `ak.soundengine.executeActionOnEvent` | `client.soundengine().execute_action_on_event(args)` | `SoundengineExecuteActionOnEventArgs` (`ActionOnEventType`, `AkCurveInterpolation`) | — | `()` |
| `ak.soundengine.postTrigger` | `client.soundengine().post_trigger(args)` | `SoundenginePostTriggerArgs` | — | `()` |
| `ak.soundengine.postMsgMonitor` | `client.soundengine().post_msg_monitor(args)` | `SoundenginePostMsgMonitorArgs` | — | `()` |
| `ak.soundengine.resetRTPCValue` | `client.soundengine().reset_rtpc_value(args)` | `SoundengineResetRtpcValueArgs` | — | `()` |
| `ak.wwise.core.object.getPropertyAndReferenceNames` | `client.wwise().core().object().get_property_and_reference_names(args)` | `CoreObjectGetPropertyAndReferenceNamesArgs` | — | `CoreObjectGetPropertyAndReferenceNamesResponse` |
| `ak.wwise.core.object.getPropertyInfo` | `client.wwise().core().object().get_property_info(args)` | `CoreObjectGetPropertyInfoArgs` | — | `CoreObjectGetPropertyInfoResponse` (`PropertyInfoSupports`) |
| `ak.wwise.core.object.getTypes` | `client.wwise().core().object().get_types()` | — | — | `CoreObjectGetTypesResponse` (`WwiseObjectTypeInfo`) |
| `ak.wwise.core.object.isLinked` | `client.wwise().core().object().is_linked(args)` | `CoreObjectIsLinkedArgs` | — | `CoreObjectIsLinkedResponse` |
| `ak.wwise.core.object.isPropertyEnabled` | `client.wwise().core().object().is_property_enabled(args)` | `CoreObjectIsPropertyEnabledArgs` | — | `CoreObjectIsPropertyEnabledResponse` |
| `ak.wwise.core.object.setLinked` | `client.wwise().core().object().set_linked(args)` | `CoreObjectSetLinkedArgs` | — | `()` |
| `ak.wwise.core.object.setStateGroups` | `client.wwise().core().object().set_state_groups(args)` | `CoreObjectSetStateGroupsArgs` | — | `()` |
| `ak.wwise.core.object.setStateProperties` | `client.wwise().core().object().set_state_properties(args)` | `CoreObjectSetStatePropertiesArgs` | — | `()` |
| `ak.wwise.core.sound.setActiveSource` | `client.wwise().core().sound().set_active_source(args)` | `CoreSoundSetActiveSourceArgs` | — | `()` |
| `ak.wwise.core.gameParameter.setRange` | `client.wwise().core().game_parameter().set_range(args)` | `CoreGameParameterSetRangeArgs` (`OnCurveUpdate`) | — | `()` |
| `ak.wwise.core.ping` | `client.wwise().core().ping()` | — | — | `CorePingResponse` |
| `ak.wwise.core.blendContainer.addAssignment` | `client.wwise().core().blend_container().add_assignment(args)` | `CoreBlendContainerAddAssignmentArgs` (`BlendEdge`) | — | `CoreBlendContainerAddAssignmentResponse` |
| `ak.wwise.core.blendContainer.addTrack` | `client.wwise().core().blend_container().add_track(args)` | `CoreBlendContainerAddTrackArgs` | — | `CoreBlendContainerAddTrackResponse` |
| `ak.wwise.core.blendContainer.getAssignments` | `client.wwise().core().blend_container().get_assignments(args)` | `CoreBlendContainerGetAssignmentsArgs` | — | `CoreBlendContainerGetAssignmentsResponse` (`BlendTrackAssignment`) |
| `ak.wwise.core.blendContainer.removeAssignment` | `client.wwise().core().blend_container().remove_assignment(args)` | `CoreBlendContainerRemoveAssignmentArgs` | — | `()` |
| `ak.wwise.core.soundbank.convertExternalSources` | `client.wwise().core().soundbank().convert_external_sources(args)` | `CoreSoundbankConvertExternalSourcesArgs` (`ExternalSourceInfo`) | — | `()` |
| `ak.wwise.core.soundbank.generate` | `client.wwise().core().soundbank().generate(args)` | `CoreSoundbankGenerateArgs` (`SoundBankInfo`) | — | `CoreSoundbankGenerateResponse` (`LogCentralItem`) |
| `ak.wwise.core.soundbank.getInclusions` | `client.wwise().core().soundbank().get_inclusions(args)` | `CoreSoundbankGetInclusionsArgs` | — | `CoreSoundbankGetInclusionsResponse` (`SoundBankInclusion`) |
| `ak.wwise.core.soundbank.processDefinitionFiles` | `client.wwise().core().soundbank().process_definition_files(args)` | `CoreSoundbankProcessDefinitionFilesArgs` | — | `()` |
| `ak.wwise.core.soundbank.setInclusions` | `client.wwise().core().soundbank().set_inclusions(args)` | `CoreSoundbankSetInclusionsArgs` (`SoundBankInclusion`, `SoundBankInclusionOperation`) | — | `()` |
| `ak.wwise.core.audio.convert` | `client.wwise().core().audio().convert(args)` | `CoreAudioConvertArgs` | — | `CoreAudioConvertResponse` (`AudioConvertError`) |
| `ak.wwise.core.audio.importTabDelimited` | `client.wwise().core().audio().import_tab_delimited(args, options)` | `CoreAudioImportTabDelimitedArgs` | `CoreAudioImportTabDelimitedOptions` | `CoreAudioImportTabDelimitedResponse` |
| `ak.wwise.core.audio.mute` | `client.wwise().core().audio().mute(args)` | `CoreAudioMuteArgs` | — | `()` |
| `ak.wwise.core.audio.resetMute` | `client.wwise().core().audio().reset_mute()` | — | — | `()` |
| `ak.wwise.core.audio.resetSolo` | `client.wwise().core().audio().reset_solo()` | — | — | `()` |
| `ak.wwise.core.audio.setConversionPlugin` | `client.wwise().core().audio().set_conversion_plugin(args)` | `CoreAudioSetConversionPluginArgs` | — | `()` |
| `ak.wwise.core.audio.solo` | `client.wwise().core().audio().solo(args)` | `CoreAudioSoloArgs` | — | `()` |
| `ak.wwise.core.mediaPool.get` | `client.wwise().core().media_pool().get(args, options)` | `CoreMediaPoolGetArgs` | `CoreMediaPoolGetOptions` | `CoreMediaPoolGetResponse` |
| `ak.wwise.core.mediaPool.getFields` | `client.wwise().core().media_pool().get_fields()` | — | — | `CoreMediaPoolGetFieldsResponse` |
| `ak.wwise.core.profiler.captureLog.save` | `client.wwise().core().profiler().capture_log().save(args)` | `CoreProfilerCaptureLogSaveArgs` | — | `()` |
| `ak.wwise.core.profiler.moveCursor` | `client.wwise().core().profiler().move_cursor(args)` | `CoreProfilerMoveCursorArgs` | — | `CoreProfilerMoveCursorResponse` |
| `ak.wwise.core.profiler.setCursorTime` | `client.wwise().core().profiler().set_cursor_time(args)` | `CoreProfilerSetCursorTimeArgs` | — | `CoreProfilerSetCursorTimeResponse` |
| `ak.wwise.core.workUnit.load` | `client.wwise().core().work_unit().load(args)` | `CoreWorkUnitLoadArgs` | — | `()` |
| `ak.wwise.core.workUnit.unload` | `client.wwise().core().work_unit().unload(args)` | `CoreWorkUnitUnloadArgs` | — | `()` |
| `ak.wwise.ui.getSelectedFiles` | `client.wwise().ui().get_selected_files()` | — | — | `UiGetSelectedFilesResponse` |
| `ak.wwise.ui.layout.closeView` | `client.wwise().ui().layout().close_view(args)` | `UiLayoutCloseViewArgs` | — | `()` |
| `ak.wwise.ui.layout.resetLayouts` | `client.wwise().ui().layout().reset_layouts(args)` | `UiLayoutResetLayoutsArgs` | — | `()` |

All generated modules live under `wwise_waapi::api::generated::ak` (e.g. `ak::wwise::core::object`, `ak::wwise::core::transport`, `ak::wwise::core::undo`, `ak::wwise::core::switchContainer`, `ak::wwise::core::soundbank`, `ak::wwise::ui::commands`).

#### Typed Topics

Every topic has a generated payload struct (`<Base>Payload`) and a `subscribe_*` chained method returning `WaapiResult<(u64, UnboundedReceiver<Payload>)>`:

| Topic URI | Subscribe Method | Payload |
|---|---|---|
| `ak.wwise.core.object.created` | `client.wwise().core().object().subscribe_created(opts)` | `CoreObjectCreatedPayload` (`object`) |
| `ak.wwise.core.object.preDeleted` | `client.wwise().core().object().subscribe_pre_deleted(opts)` | `CoreObjectPreDeletedPayload` (`object`) |
| `ak.wwise.core.object.postDeleted` | `client.wwise().core().object().subscribe_post_deleted(opts)` | `CoreObjectPostDeletedPayload` (`object`) |
| `ak.wwise.core.object.childAdded` | `client.wwise().core().object().subscribe_child_added(opts)` | `CoreObjectChildAddedPayload` (`parent`, `child`) |
| `ak.wwise.core.object.childRemoved` | `client.wwise().core().object().subscribe_child_removed(opts)` | `CoreObjectChildRemovedPayload` (`parent`, `child`) |
| `ak.wwise.core.object.nameChanged` | `client.wwise().core().object().subscribe_name_changed(opts)` | `CoreObjectNameChangedPayload` (`object`, `newName`, `oldName`) |
| `ak.wwise.core.object.notesChanged` | `client.wwise().core().object().subscribe_notes_changed(opts)` | `CoreObjectNotesChangedPayload` (`object`, `newNotes`, `oldNotes`) |
| `ak.wwise.core.object.propertyChanged` | `client.wwise().core().object().subscribe_property_changed(opts)` | `CoreObjectPropertyChangedPayload` (`object`, `property`, `old`, `new`) |
| `ak.wwise.core.object.referenceChanged` | `client.wwise().core().object().subscribe_reference_changed(opts)` | `CoreObjectReferenceChangedPayload` |
| `ak.wwise.core.object.curveChanged` | `client.wwise().core().object().subscribe_curve_changed(opts)` | `CoreObjectCurveChangedPayload` |
| `ak.wwise.core.object.attenuationCurveChanged` | `client.wwise().core().object().subscribe_attenuation_curve_changed(opts)` | `CoreObjectAttenuationCurveChangedPayload` (`curveType`) |
| `ak.wwise.core.object.attenuationCurveLinkChanged` | `client.wwise().core().object().subscribe_attenuation_curve_link_changed(opts)` | `CoreObjectAttenuationCurveLinkChangedPayload` (`curveType`) |
| `ak.wwise.core.object.structureChanged` | `client.wwise().core().object().subscribe_structure_changed(opts)` | `CoreObjectStructureChangedPayload` (`objects`, `summary`) |
| `ak.wwise.core.switchContainer.assignmentAdded` | `client.wwise().core().switch_container().subscribe_assignment_added(opts)` | `CoreSwitchContainerAssignmentAddedPayload` |
| `ak.wwise.core.switchContainer.assignmentRemoved` | `client.wwise().core().switch_container().subscribe_assignment_removed(opts)` | `CoreSwitchContainerAssignmentRemovedPayload` |
| `ak.wwise.core.transport.stateChanged` | `client.wwise().core().transport().subscribe_state_changed(opts)` | `CoreTransportStateChangedPayload` (`state`) |
| `ak.wwise.core.soundbank.generated` | `client.wwise().core().soundbank().subscribe_generated(opts)` | `CoreSoundbankGeneratedPayload` |
| `ak.wwise.core.soundbank.generationDone` | `client.wwise().core().soundbank().subscribe_generation_done(opts)` | `CoreSoundbankGenerationDonePayload` (`logs`, `error`) |
| `ak.wwise.core.project.preClosed` | `client.wwise().core().project().subscribe_pre_closed(opts)` | `CoreProjectPreClosedPayload` |
| `ak.wwise.core.project.loaded` | `client.wwise().core().project().subscribe_loaded(opts)` | `CoreProjectLoadedPayload` |
| `ak.wwise.core.project.postClosed` | `client.wwise().core().project().subscribe_post_closed(opts)` | `CoreProjectPostClosedPayload` |
| `ak.wwise.core.project.saved` | `client.wwise().core().project().subscribe_saved(opts)` | `CoreProjectSavedPayload` (`modifiedPaths`) |
| `ak.wwise.core.audio.imported` | `client.wwise().core().audio().subscribe_imported(opts)` | `CoreAudioImportedPayload` (`files`, `objects`) |
| `ak.wwise.core.log.itemAdded` | `client.wwise().core().log().subscribe_item_added(opts)` | `CoreLogItemAddedPayload` (`channel`, `item`) |
| `ak.wwise.core.profiler.gameObjectRegistered` | `client.wwise().core().profiler().subscribe_game_object_registered(opts)` | `CoreProfilerGameObjectRegisteredPayload` |
| `ak.wwise.core.profiler.gameObjectUnregistered` | `client.wwise().core().profiler().subscribe_game_object_unregistered(opts)` | `CoreProfilerGameObjectUnregisteredPayload` |
| `ak.wwise.core.profiler.gameObjectReset` | `client.wwise().core().profiler().subscribe_game_object_reset(opts)` | `CoreProfilerGameObjectResetPayload` |
| `ak.wwise.core.profiler.stateChanged` | `client.wwise().core().profiler().subscribe_state_changed(opts)` | `CoreProfilerStateChangedPayload` |
| `ak.wwise.core.profiler.switchChanged` | `client.wwise().core().profiler().subscribe_switch_changed(opts)` | `CoreProfilerSwitchChangedPayload` |
| `ak.wwise.core.profiler.captureLog.itemAdded` | `client.wwise().core().profiler().capture_log().subscribe_item_added(opts)` | `CoreProfilerCaptureLogItemAddedPayload` |
| `ak.wwise.debug.assertFailed` | `client.wwise().debug().subscribe_assert_failed(opts)` | `DebugAssertFailedPayload` |
| `ak.wwise.ui.selectionChanged` | `client.wwise().ui().subscribe_selection_changed(opts)` | `UiSelectionChangedPayload` (`objects`) |
| `ak.wwise.ui.commands.executed` | `client.wwise().ui().commands().subscribe_executed(opts)` | `UiCommandsExecutedPayload` |
| `ak.wwise.ui.signal.click` | `client.wwise().ui().signal().subscribe_click(opts)` | `UiSignalClickPayload` |
| `ak.wwise.ui.signal.toggle` | `client.wwise().ui().signal().subscribe_toggle(opts)` | `UiSignalTogglePayload` |

Topic `opts` is `Option<XxxOptions>`; options structs offer `.return_fields([ObjectReturnField::...])` and `.platform(...)`, and some are `ObjectReturnOptions` aliases.

If you ever need to send a custom or untyped payload, use `client.call(WaapiFunction::..., ...)` / `client.call_typed(WaapiFunction::..., ...)` or `client.subscribe(WaapiTopic::..., ...)` / `client.subscribe_typed(WaapiTopic::..., ...)`.

### Specifying Return Fields

Typed endpoint and topic `Options` structs provide a `.return_fields(...)` builder that accepts any iterable (such as an array `[ObjectReturnField::Id, ObjectReturnField::Name]` or `Vec`). `ObjectReturnOptions::basic()` returns the common `[Id, Name, Type, Path]` preset, and RPC methods taking `impl Into<Options>` also accept `()` (default options) or an array of return fields directly:

```rust
// Pass ObjectReturnOptions::basic(), a custom GetOptions, an array of fields, or () for default options:
let res = client
    .object()
    .get(r"\Containers\Default Work Unit", ObjectReturnOptions::basic())
    .await?;
```

For raw JSON escape-hatch calls, return fields are a plain string array in the options object:

```rust
let options = json!({
    "return": ["id", "name", "type", "path"]
});
```

## API Endpoints

All WAAPI endpoints are available in the `waapi_function_api::ak` module as constants:

```rust
use wwise_waapi::waapi_function_api::ak;

// Core API
ak::wwise::core::getInfo
ak::wwise::core::object::get

// Sound Engine
ak::soundengine::postEvent
ak::soundengine::setRTPCValue

// UI API
ak::wwise::ui::getSelectedObjects
ak::wwise::ui::bringToForeground
```

All 180 WAAPI functions and 35 topics are available. See [`src/waapi_function_api.rs`](src/waapi_function_api.rs) and [`src/waapi_topics_api.rs`](src/waapi_topics_api.rs) for the complete list organized by module.

## Examples

Run examples with:

```bash
cargo run --example get_info
cargo run --example chained_api
cargo run --example object_get_set
cargo run --example object_set
cargo run --example raw_call
cargo run --example register_and_post_event
cargo run --example subscribe
```

The object examples target Wwise 2025 (`\Containers` hierarchy). For Wwise 2024 (`\Actor-Mixer Hierarchy`), use the `_2024` variants:

```bash
cargo run --example object_get_set_2024
cargo run --example object_set_2024
```

See the [`examples/`](examples/) directory for more usage examples. Most examples require Wwise running with WAAPI enabled on `127.0.0.1:8080`.

## Error Handling

The library uses a custom `WaapiError` enum with the following variants:

- `Connection(String)` - Failed to establish the WebSocket/WAMP connection to WAAPI
- `ClientShutdown` - Operation attempted on a `WaapiClient` that has already been shut down
- `Serialization(serde_json::Error)` - JSON serialization/deserialization failed
- `Call(WaapiCallError)` - WAAPI call returned a structured WAMP error (`err.uri()`, `err.message()`, `err.details()`, `err.args`, `err.kwargs`; also accessible via `waapi_err.error_uri()`, `waapi_err.waapi_message()`, and `waapi_err.as_call_error()`)
- `Wamp(WampError)` - Underlying WAMP/WebSocket protocol error

Always call `shutdown().await` before dropping the client to ensure clean disconnection.

## Testing

```bash
# Run all tests (unit tests, offline roundtrip tests, and live Wwise sequence tests)
cargo test

# Run only offline serialization/deserialization roundtrip tests (no Wwise required)
cargo test --test typed_api_roundtrip

# Run live end-to-end WAAPI sequence tests
cargo test --test wwise_sequences
```

> **Note:** `tests/wwise_sequences.rs` (`tests/sequences/`) runs end-to-end command sequences against a live Wwise Authoring instance (`127.0.0.1:8080`) with an open project. Each sequence creates temporary objects under the default work units, exercises the API, verifies state, and deletes all temporary objects before shutting down.

## License

Copyright (c) 2026 Christian Tronhjem.

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are more than welcome!
