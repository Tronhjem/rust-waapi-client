# Typed API Design & Endpoint Reference

All generated bindings in `src/waapi_function_api.rs`, `src/waapi_topics_api.rs`, and `src/api/generated/` are derived offline from the Wwise JSON schemas stored in `tools/waapi_schemas/<version>/` (fetched from a running Wwise instance via `tools/get_waapi_definitions.py`).

For interactive API documentation of every generated module, struct, enum, and method, see the **[Rustdoc API Documentation](https://spektakelaudio.github.io/rust-waapi-client/wwise_waapi/)**.

---

## 1. URI-to-Chain and Module Mapping

Every WAAPI URI is split on `.`:
- The **prefix segments** map 1-to-1 to both the Rust module path (`wwise_waapi::api::generated::ak::...`, re-exported at `wwise_waapi::api::ak::...`) and zero-cost chained borrower structs (`AkClient<'a>`, `WwiseClient<'a>`, `WwiseCoreClient<'a>`, `WwiseCoreObjectClient<'a>`, etc.) in `src/api/generated/clients.rs`.
- The **final segment** becomes the `snake_case` async method on that namespace client (or `subscribe_<name>` for subscription topics).
- `client.wwise()`, `client.core()`, `client.object()`, `client.ui()`, and `client.soundengine()` are provided on `WaapiClient` as convenient top-level shortcuts for `client.ak().wwise()`, `client.wwise().core()`, `client.wwise().core().object()`, `client.wwise().ui()`, and `client.ak().soundengine()`.

| WAAPI URI | Chained Call | Module Path |
|---|---|---|
| `ak.wwise.core.getInfo` | `client.core().get_info()` / `client.wwise().core().get_info()` | `api::ak::wwise::core` |
| `ak.wwise.core.object.get` | `client.object().get(args, options)` / `client.wwise().core().object().get(args, options)` | `api::ak::wwise::core::object` |
| `ak.soundengine.postEvent` | `client.soundengine().post_event(args)` | `api::ak::soundengine` |

---

## 2. Type Naming Convention (`<Base>{Args,Options,Response,Payload}`)

For each endpoint URI, the generator strips the leading `ak.wwise.` prefix (or `ak.` for non-`wwise` namespaces like `ak.soundengine`), splits camelCase words, joins all remaining segments in `PascalCase` as `<Base>`, and emits canonical types:
- `<Base>Args` — request keyword arguments (`ArgumentsKw`)
- `<Base>Options` — request WAMP call/subscribe options (`Options`)
- `<Base>Response` — deserialized response payload
- `<Base>Payload` — deserialized subscription topic payload

For example:
- `ak.wwise.core.getInfo` -> `CoreGetInfoResponse`
- `ak.wwise.core.object.get` -> `CoreObjectGetArgs`, `CoreObjectGetOptions`, `CoreObjectGetResponse`
- `ak.wwise.core.object.set` -> `CoreObjectSetArgs`, `CoreObjectSetItem`, `CoreObjectSetOptions`, `CoreObjectSetResponse`
- `ak.wwise.ui.getSelectedObjects` -> `UiGetSelectedObjectsOptions`, `UiGetSelectedObjectsResponse`
- `ak.soundengine.setRTPCValue` -> `SoundengineSetRtpcValueArgs`

Constructors (`Args::new(...)`) take concrete domain types (`ObjectRef`, `GameObjectRef`, `SoundEngineObjectRef`, `PlatformRef`, `LanguageRef`, `ObjectQuery`) and `impl Into<String>` / `impl Into<PropertyValue>` for string and property-value fields. Domain reference and query types provide explicit named constructors (`ObjectRef::path/guid/name`, `ObjectQuery::path/id/of_type/search`, `CoreObjectGetArgs::from_waql`, `SoundEngineObjectRef::name/guid/short_id`, `GameObjectRef::id`, `PlatformRef::name/guid`, `LanguageRef::name/guid`, `CoreObjectSetArgs::from_item`).

---

## 3. How Wwise JSON Schema Sections Map to Rust

Each `tools/waapi_schemas/<version>/ak.*.json` file contains either RPC function schemas (`argsSchema`, `optionsSchema`, `resultSchema`) or topic schemas (`publishSchema`, `optionsSchema`):

| JSON Schema Section | Rust Output | Rules |
|---|---|---|
| `argsSchema` | `<Base>Args` (`#[derive(Serialize)]`) | **Required properties** (`required` array) become positional parameters on `<Base>Args::new(...)`. **Optional properties** become `Option<T>` fields with `#[serde(skip_serializing_if = "Option::is_none")]` and chainable builder methods `.field(val)`. If `argsSchema` has no properties, no `<Base>Args` type is emitted and the client method takes no `args` parameter. |
| `optionsSchema` | `<Base>Options` (`#[derive(Serialize)]`) | Generated with `::new()` (`Default`) and chainable builder methods. When `optionsSchema` is `{"$ref": "waapi_definitions.json#/definitions/objectReturnOptions"}`, `<Base>Options` is emitted as `pub type <Base>Options = ObjectReturnOptions;`. If `optionsSchema` is empty, no `<Base>Options` type is emitted and the client method takes no `options` parameter. |
| `resultSchema` | `<Base>Response` (`#[derive(Serialize, Deserialize)]`) | Response fields are generated from `resultSchema.properties`. When present, the chained method calls `WaapiClient::call_typed` and returns `WaapiResult<<Base>Response>`. When `resultSchema` has no properties, no `<Base>Response` is emitted and the method calls `WaapiClient::call_typed_void`, returning `WaapiResult<()>`. |
| `publishSchema` | `WaapiTopic` + `<Base>Payload` | Identifies the schema file as a **subscription topic** (35 topics) rather than an RPC function (180 functions). `tools/generate_waapi_uri.py` emits it into `WaapiTopic` (`src/waapi_topics_api.rs`), and `tools/generate_typed_api.py` emits `<Base>Payload` and `subscribe_*` methods on the namespace client. |

---

## 4. Shared `$ref` Definitions, Return Fields, and `@Property` Extensions

- **`Definitions/*.json` `$ref` resolution**: Shared references across `common_definitions.json`, `waapi_definitions.json`, `waapi_ui_definitions.json`, and `wafm_definitions.json` map to shared primitives in `src/api/primitives.rs` and `src/api/generated/primitives.rs` (`ObjectRef`, `PlatformRef`, `SoundEngineObjectRef`, `GameObjectRef`, `LanguageRef`, `WwiseObject`, `WwiseObjectRef`, `OnNameConflict`, `ListMode`, `ImportOperation`, `LogSeverity`, `PropertyValue`, `ChildObject`).
- **Endpoint-domain enums and structs**: Schemas with inline string enums or nested objects map to generated types in `src/api/generated/primitives.rs` (re-exported from `wwise_waapi::api`), including:
  - **Transport & Authoring:** `TransportAction` / `TransportState` / `TransportObject` (`transport.*`), `AttenuationCurveType` / `AttenuationCurveUse` / `CurvePoint2D` / `CurveShape` (`object.{get,set}AttenuationCurve`), `PasteMode` (`object.pasteProperties`), `SwitchContainerAssignment` (`switchContainer.getAssignments`), `BlendEdge` / `BlendFadeMode` / `BlendTrackAssignment` (`blendContainer.*`), `OnCurveUpdate` (`gameParameter.setRange`), `WwiseObjectTypeInfo` (`object.getTypes`), `PropertyInfoSupports` / `RtpcSupportMode` / `PropertyDisplayInfo` / `PropertyUiInfo` / `PropertyUiValueInfo` / `PropertyUiDisplayAs` / `PropertyDataMeaning` / `PropertyDependency` / `PropertyDependencyType` / `PropertyDependencyCondition` / `PropertyRestriction` (`object.getPropertyInfo`), `StructureChangedObject` / `StructureChange` / `StructureChangeType` / `StructureParentChange` / `StructureOwnerChange` / `StructureNameChange` / `StructureNotesChange` (`object.structureChanged`), `ProjectMigrationAction` (`ui.project.open`), `PlatformTemplateEntry` / `PlatformsToCreate` (`ui.project.create`), and `ProjectPlatformInfo` / `ProjectLanguageInfo` / `ProjectDirectoriesInfo` / `DefaultWorkUnitInfo` (`core.getProjectInfo`).
  - **SoundEngine:** `ActionOnEventType`, `AkCurveInterpolation`, `MultiPositionType`, `MultiPosition`, `AuxSendValue`.
  - **Profiler:** `CursorTime`, `ProfilerCursor`, `CursorMovePosition`, `ProfilerDataType`, `ProfilerDataTypeEntry`, `ProfilerVoice`, `ProfilerBus`, `ProfilerAudioObject`, `AudioObjectMetadata`, `ProfilerGameObject`, `ProfilerRtpc`, `ProfilerCpuUsageItem`, `ProfilerLoadedMedia`, `ProfilerStreamedMedia`, `ProfilerPerformanceCounter`, `ProfilerMeter`, `MeterSource`, `MeterConfigurationType`, `MeterChannel`, `VoiceContributions`, `VoiceContributionObject`, `VoiceContributionParameter`, `CaptureLogType`, `CaptureLogSeverity`.
  - **SoundBank, Audio & Logs:** `SoundBankInfo`, `SoundBankInclusion`, `SoundBankInclusionType`, `SoundBankInclusionFilter`, `SoundBankInclusionOperation`, `SoundBankGeneratedBankData`, `SoundBankPluginInfo`, `SoundBankPluginLib`, `ExternalSourceInfo`, `AudioConvertError`, `AudioImportLogItem`, `AudioImportedOperation`, `LogCentralItem`, `LogChannel`, `LogSeverity`.
  - **Remote, Source Control & Media Pool:** `RemoteConsoleInfo`, `ConnectedConsoleInfo`, `SourceControlFileFilter`, `SourceControlReturnField`, `SourceControlSourceFile`, `SourceControlStatus`, `MediaPoolFilterEntry`, `MediaPoolFilterType`, `MediaPoolFilterOperator`.
  - **UI & Debug:** `ToneChannelConfig`, `ToneBitDepth`, `ToneWaveform`, `WavMarker`, `ScreenRect`, `WindowSize`, `ViewInstance`, `ViewTypeInfo`, `WidgetAttributeEntry`, `WidgetStateQuery`, `WidgetStateResult`, `WidgetStateUpdate`, `AddOnCommand`, `CommandStartMode`, `CommandContextMenu`, `CommandMainMenu`, `StringOrVec`.
- **Domain-specific return field enums**: Rather than untyped strings, `waapi_definitions.json` return expressions are generated as strongly-typed enums:
  - `ObjectReturnField` (wrapped by `ObjectReturnOptions`) — all built-in Wwise object accessors (`Id`, `Name`, `Notes`, `Type`, `Path`, `Parent`, `ShortId`, etc.), `ObjectReturnField::Property(Property)` / `ObjectReturnField::property(Property)` (which serializes as `"@Property"`), plus `ObjectReturnField::Custom(String)` / `ObjectReturnField::custom("...")` for arbitrary dot-accessor queries.
  - `BusReturnField`, `AudioObjectReturnField`, and `VoiceReturnField` for profiler queries.
- **Generated `ObjectType` enum (`WObjects.xml`)**: `src/api/generated/object_types.rs` (re-exported as `wwise_waapi::ObjectType` and `wwise_waapi::api::ObjectType`) is generated from the `<WwiseObject>` definitions in `Definitions/WObjects.xml` and contains all Wwise object types (e.g. `ObjectType::Sound`, `ObjectType::ActorMixer`/`ObjectType::PropertyContainer`, `ObjectType::Event`, `ObjectType::Folder`, `ObjectType::RandomSequenceContainer`). It is used in `CoreObjectCreateArgs`, `ChildObject`, `WwiseObject`, `ObjectQuery::OfType`, `ImportItem`, and `ImportDefaults`, and provides `as_str()`, `class_id()`, `from_class_id()`, `FromStr`, `AsRef<str>`, `Display`, `Serialize`, and `Deserialize`. The union of Wwise 2024 + 2025 `WObjects.xml` keeps `ActorMixer` (2024) and `PropertyContainer` (2025) as distinct variants sharing `class_id()` `524304`.
- **Generated `Property` enum (`WObjects.xml`)**: `src/api/generated/properties.rs` (re-exported as `wwise_waapi::Property` and `wwise_waapi::api::Property`) is generated from `Definitions/WObjects.xml` and contains all Wwise object properties, references, and object lists (e.g. `Property::Volume`, `Property::Pitch`, `Property::Lowpass`, `Property::OutputBus`, `Property::RTPC`). `Property::as_str()` returns the bare `&'static str` (`"Volume"`), `Property::as_at_str()` returns the `@`-prefixed `&'static str` (`"@Volume"`), and `Property` implements `AsRef<str>`, `Display`, `Serialize`, and `Deserialize`.
- **Dynamic `@Property` fields (`patternProperties`)**: Schemas that accept or return open-ended `@`-prefixed properties (such as `CoreObjectSetItem`, `CoreObjectCreateArgs`, `ImportItem`, `ImportDefaults`, `WwiseObject`, and `CoreObjectSetObject`) include `#[serde(flatten)] pub extra: HashMap<String, serde_json::Value>` along with `.property(Property::Volume, value)` / `.extra(key, value)` builder methods on request structs and `.property(Property::Volume)` / `.property_f64(...)` / `.property_i64(...)` / `.property_bool(...)` / `.property_str(...)` lookup helpers on response structs.
- **Field naming**: JSON `camelCase` wire names are converted to Rust `snake_case` with `#[serde(rename = "...")]` emitted automatically whenever the wire name differs (e.g. `type` -> `object_type`, `return` -> `objects` / `playing_id` / `return_fields`).

### Specifying Return Fields

Typed endpoint and topic `Options` structs provide a `.return_fields(...)` builder that accepts any iterable (such as an array `[ObjectReturnField::Id, ObjectReturnField::Name]` or `Vec`). `ObjectReturnOptions::basic()` returns the common `[Id, Name, Type, Path]` preset:

```rust
let res = client
    .object()
    .get(
        CoreObjectGetArgs::new(ObjectQuery::path(r"\Containers\Default Work Unit")),
        ObjectReturnOptions::basic(),
    )
    .await?;
```

For raw JSON escape-hatch calls, return fields are a plain string array in the options object:

```rust
let options = json!({
    "return": ["id", "name", "type", "path"]
});
```

---

## 5. Generated Typed Endpoints (180 Functions & 35 Topics)

All **180 WAAPI functions** and **35 subscription topics** across the union of `tools/waapi_schemas/<version>/` (2024.1.7 and 2025.1.12) are automatically discovered and generated into typed structs, chained call methods, and typed `subscribe_*` methods under `wwise_waapi::api::generated` (configured via `tools/overrides.json` and `tools/generator/`).

> **Multi-version union & optionality.** The bindings are generated from the *union* of all Wwise schema versions under `tools/waapi_schemas/`, so a single API serves both Wwise 2024 and Wwise 2025+. Renamed object types are kept as aliases (`ObjectType::ActorMixer` and `ObjectType::PropertyContainer` share `class_id()` `524304`, with `PropertyContainer` as the canonical `from_class_id` result), and 2024-only return fields (`ObjectReturnField::ColorRgb`/`TransitionObject`, `AudioObjectReturnField::PrevGain`/`NextGain`) are retained. Response fields on `WwiseObject` are `Option<T>` because WAAPI only populates the return fields requested via `ObjectReturnOptions` (and so older/newer Wwise versions remain compatible); `WwiseObject` provides `.id()`, `.name()`, `.object_type()`, `.path()`, `.notes()`, `.property(...)`, `.property_f64(...)`, `.property_i64(...)`, `.property_bool(...)`, and `.property_str(...)` convenience accessors. Intermediate chained client handles (`let wwise = client.wwise();`) take `&self` (`Copy`) and can be reused across multiple calls.

### Core, Authoring, UI & SoundEngine Functions

Every remaining namespace (`profiler.*`, `remote.*`, `sourceControl.*`, `waapi.*`, `log.*`, `audioSourcePeaks.*`, `debug.*`, `ui.layout.*`, `ui.window.*`, `ui.model.*`, `ui.cli.*`) is also generated under `wwise_waapi::api::ak`:

| WAAPI URI | Chained Method on `WaapiClient` | `Args` Type | `Options` Type | `Response` Type |
|---|---|---|---|---|
| `ak.wwise.core.getInfo` | `client.wwise().core().get_info()` | — | — | `CoreGetInfoResponse` |
| `ak.wwise.core.getProjectInfo` | `client.wwise().core().get_project_info()` | — | — | `CoreGetProjectInfoResponse` |
| `ak.wwise.core.project.save` | `client.wwise().core().project().save(args)` | `CoreProjectSaveArgs` | — | `()` |
| `ak.wwise.core.object.get` | `client.wwise().core().object().get(args, options)` | `CoreObjectGetArgs` | `CoreObjectGetOptions` | `CoreObjectGetResponse` |
| `ak.wwise.core.object.set` | `client.wwise().core().object().set(args, options)` | `CoreObjectSetArgs` + `CoreObjectSetItem` | `CoreObjectSetOptions` | `CoreObjectSetResponse` + `CoreObjectSetObject` |
| `ak.wwise.core.object.create` | `client.wwise().core().object().create(args)` | `CoreObjectCreateArgs` | — | `CoreObjectCreateResponse` |
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
| `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion` | `client.wwise().core().audio_source_peaks().get_min_max_peaks_in_region(args)` | `CoreAudioSourcePeaksGetMinMaxPeaksInRegionArgs` | — | `CoreAudioSourcePeaksGetMinMaxPeaksInRegionResponse` |
| `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion` | `client.wwise().core().audio_source_peaks().get_min_max_peaks_in_trimmed_region(args)` | `CoreAudioSourcePeaksGetMinMaxPeaksInTrimmedRegionArgs` | — | `CoreAudioSourcePeaksGetMinMaxPeaksInTrimmedRegionResponse` |
| `ak.wwise.core.executeLuaScript` | `client.wwise().core().execute_lua_script(args)` | `CoreExecuteLuaScriptArgs` | — | `CoreExecuteLuaScriptResponse` |
| `ak.wwise.core.log.addItem` | `client.wwise().core().log().add_item(args)` | `CoreLogAddItemArgs` | — | `()` |
| `ak.wwise.core.log.clear` | `client.wwise().core().log().clear(args)` | `CoreLogClearArgs` (`LogChannel`) | — | `()` |
| `ak.wwise.core.log.get` | `client.wwise().core().log().get(args)` | `CoreLogGetArgs` (`LogChannel`) | — | `CoreLogGetResponse` (`LogCentralItem`) |
| `ak.wwise.core.profiler.enableProfilerData` | `client.wwise().core().profiler().enable_profiler_data(args)` | `CoreProfilerEnableProfilerDataArgs` (`ProfilerDataTypeEntry`) | — | `()` |
| `ak.wwise.core.profiler.getAudioObjects` | `client.wwise().core().profiler().get_audio_objects(args, options)` | `CoreProfilerGetAudioObjectsArgs` (`CursorTime`) | `CoreProfilerGetAudioObjectsOptions` (`AudioObjectReturnField`) | `CoreProfilerGetAudioObjectsResponse` (`ProfilerAudioObject`) |
| `ak.wwise.core.profiler.getBusses` | `client.wwise().core().profiler().get_busses(args, options)` | `CoreProfilerGetBussesArgs` (`CursorTime`) | `CoreProfilerGetBussesOptions` (`BusReturnField`) | `CoreProfilerGetBussesResponse` (`ProfilerBus`) |
| `ak.wwise.core.profiler.getCpuUsage` | `client.wwise().core().profiler().get_cpu_usage(args)` | `CoreProfilerGetCpuUsageArgs` (`CursorTime`) | — | `CoreProfilerGetCpuUsageResponse` (`ProfilerCpuUsageItem`) |
| `ak.wwise.core.profiler.getCursorTime` | `client.wwise().core().profiler().get_cursor_time(args)` | `CoreProfilerGetCursorTimeArgs` (`ProfilerCursor`) | — | `CoreProfilerGetCursorTimeResponse` |
| `ak.wwise.core.profiler.getGameObjects` | `client.wwise().core().profiler().get_game_objects(args)` | `CoreProfilerGetGameObjectsArgs` (`CursorTime`) | — | `CoreProfilerGetGameObjectsResponse` (`ProfilerGameObject`) |
| `ak.wwise.core.profiler.getLoadedMedia` | `client.wwise().core().profiler().get_loaded_media(args)` | `CoreProfilerGetLoadedMediaArgs` (`CursorTime`) | — | `CoreProfilerGetLoadedMediaResponse` (`ProfilerLoadedMedia`) |
| `ak.wwise.core.profiler.getMeters` | `client.wwise().core().profiler().get_meters(args, options)` | `CoreProfilerGetMetersArgs` | `CoreProfilerGetMetersOptions` | `CoreProfilerGetMetersResponse` (`ProfilerMeter`) |
| `ak.wwise.core.profiler.getPerformanceMonitor` | `client.wwise().core().profiler().get_performance_monitor(args)` | `CoreProfilerGetPerformanceMonitorArgs` (`CursorTime`) | — | `CoreProfilerGetPerformanceMonitorResponse` (`ProfilerPerformanceCounter`) |
| `ak.wwise.core.profiler.getRTPCs` | `client.wwise().core().profiler().get_rtpcs(args)` | `CoreProfilerGetRtpCsArgs` (`CursorTime`) | — | `CoreProfilerGetRtpCsResponse` (`ProfilerRtpc`) |
| `ak.wwise.core.profiler.getStreamedMedia` | `client.wwise().core().profiler().get_streamed_media(args)` | `CoreProfilerGetStreamedMediaArgs` (`CursorTime`) | — | `CoreProfilerGetStreamedMediaResponse` (`ProfilerStreamedMedia`) |
| `ak.wwise.core.profiler.getVoiceContributions` | `client.wwise().core().profiler().get_voice_contributions(args)` | `CoreProfilerGetVoiceContributionsArgs` (`CursorTime`) | — | `CoreProfilerGetVoiceContributionsResponse` (`VoiceContributions`) |
| `ak.wwise.core.profiler.getVoices` | `client.wwise().core().profiler().get_voices(args, options)` | `CoreProfilerGetVoicesArgs` (`CursorTime`) | `CoreProfilerGetVoicesOptions` (`VoiceReturnField`) | `CoreProfilerGetVoicesResponse` (`ProfilerVoice`) |
| `ak.wwise.core.profiler.registerMeter` | `client.wwise().core().profiler().register_meter(args)` | `CoreProfilerRegisterMeterArgs` | — | `()` |
| `ak.wwise.core.profiler.saveCapture` | `client.wwise().core().profiler().save_capture(args)` | `CoreProfilerSaveCaptureArgs` | — | `()` |
| `ak.wwise.core.profiler.startCapture` | `client.wwise().core().profiler().start_capture()` | — | — | `CoreProfilerStartCaptureResponse` |
| `ak.wwise.core.profiler.stopCapture` | `client.wwise().core().profiler().stop_capture()` | — | — | `CoreProfilerStopCaptureResponse` |
| `ak.wwise.core.profiler.unregisterMeter` | `client.wwise().core().profiler().unregister_meter(args)` | `CoreProfilerUnregisterMeterArgs` | — | `()` |
| `ak.wwise.core.remote.connect` | `client.wwise().core().remote().connect(args)` | `CoreRemoteConnectArgs` | — | `()` |
| `ak.wwise.core.remote.disconnect` | `client.wwise().core().remote().disconnect()` | — | — | `()` |
| `ak.wwise.core.remote.getAvailableConsoles` | `client.wwise().core().remote().get_available_consoles()` | — | — | `CoreRemoteGetAvailableConsolesResponse` (`RemoteConsoleInfo`) |
| `ak.wwise.core.remote.getConnectionStatus` | `client.wwise().core().remote().get_connection_status()` | — | — | `CoreRemoteGetConnectionStatusResponse` (`ConnectedConsoleInfo`) |
| `ak.wwise.core.sourceControl.add` | `client.wwise().core().source_control().add(args)` | `CoreSourceControlAddArgs` (`StringOrVec`) | — | `CoreSourceControlAddResponse` (`LogCentralItem`) |
| `ak.wwise.core.sourceControl.checkOut` | `client.wwise().core().source_control().check_out(args)` | `CoreSourceControlCheckOutArgs` (`StringOrVec`) | — | `CoreSourceControlCheckOutResponse` (`LogCentralItem`) |
| `ak.wwise.core.sourceControl.commit` | `client.wwise().core().source_control().commit(args)` | `CoreSourceControlCommitArgs` (`StringOrVec`) | — | `CoreSourceControlCommitResponse` (`LogCentralItem`) |
| `ak.wwise.core.sourceControl.delete` | `client.wwise().core().source_control().delete(args)` | `CoreSourceControlDeleteArgs` (`StringOrVec`) | — | `CoreSourceControlDeleteResponse` (`LogCentralItem`) |
| `ak.wwise.core.sourceControl.getSourceFiles` | `client.wwise().core().source_control().get_source_files(args, options)` | `CoreSourceControlGetSourceFilesArgs` (`SourceControlFileFilter`) | `CoreSourceControlGetSourceFilesOptions` (`SourceControlReturnField`) | `CoreSourceControlGetSourceFilesResponse` (`SourceControlSourceFile`) |
| `ak.wwise.core.sourceControl.getStatus` | `client.wwise().core().source_control().get_status(args)` | `CoreSourceControlGetStatusArgs` (`StringOrVec`) | — | `CoreSourceControlGetStatusResponse` (`SourceControlStatus`, `LogCentralItem`) |
| `ak.wwise.core.sourceControl.move` | `client.wwise().core().source_control().move_(args)` | `CoreSourceControlMoveArgs` | — | `CoreSourceControlMoveResponse` (`LogCentralItem`) |
| `ak.wwise.core.sourceControl.revert` | `client.wwise().core().source_control().revert(args)` | `CoreSourceControlRevertArgs` (`StringOrVec`) | — | `CoreSourceControlRevertResponse` (`LogCentralItem`) |
| `ak.wwise.core.sourceControl.setProvider` | `client.wwise().core().source_control().set_provider(args)` | `CoreSourceControlSetProviderArgs` | — | `CoreSourceControlSetProviderResponse` (`LogCentralItem`) |
| `ak.wwise.debug.enableAsserts` | `client.wwise().debug().enable_asserts(args)` | `DebugEnableAssertsArgs` | — | `()` |
| `ak.wwise.debug.enableAutomationMode` | `client.wwise().debug().enable_automation_mode(args)` | `DebugEnableAutomationModeArgs` | — | `()` |
| `ak.wwise.debug.generateToneWAV` | `client.wwise().debug().generate_tone_wav(args)` | `DebugGenerateToneWavArgs` (`ToneWaveform`, `ToneBitDepth`, `ToneChannelConfig`, `WavMarker`) | — | `()` |
| `ak.wwise.debug.getWalTree` | `client.wwise().debug().get_wal_tree()` | — | — | `DebugGetWalTreeResponse` |
| `ak.wwise.debug.restartWaapiServers` | `client.wwise().debug().restart_waapi_servers()` | — | — | `()` |
| `ak.wwise.debug.testAssert` | `client.wwise().debug().test_assert()` | — | — | `()` |
| `ak.wwise.debug.testCrash` | `client.wwise().debug().test_crash()` | — | — | `()` |
| `ak.wwise.debug.validateCall` | `client.wwise().debug().validate_call(args)` | `DebugValidateCallArgs` | — | `()` |
| `ak.wwise.ui.captureScreen` | `client.wwise().ui().capture_screen(args)` | `UiCaptureScreenArgs` (`ScreenRect`) | — | `UiCaptureScreenResponse` |
| `ak.wwise.ui.cli.executeLuaScript` | `client.wwise().ui().cli().execute_lua_script(args)` | `UiCliExecuteLuaScriptArgs` | — | `UiCliExecuteLuaScriptResponse` |
| `ak.wwise.ui.cli.launch` | `client.wwise().ui().cli().launch(args)` | `UiCliLaunchArgs` | — | `()` |
| `ak.wwise.ui.commands.register` | `client.wwise().ui().commands().register(args)` | `UiCommandsRegisterArgs` (`AddOnCommand`) | — | `()` |
| `ak.wwise.ui.commands.unregister` | `client.wwise().ui().commands().unregister(args)` | `UiCommandsUnregisterArgs` | — | `()` |
| `ak.wwise.ui.layout.dockView` | `client.wwise().ui().layout().dock_view(args)` | `UiLayoutDockViewArgs` | — | `()` |
| `ak.wwise.ui.layout.getCurrentLayoutName` | `client.wwise().ui().layout().get_current_layout_name()` | — | — | `UiLayoutGetCurrentLayoutNameResponse` |
| `ak.wwise.ui.layout.getElementRectangle` | `client.wwise().ui().layout().get_element_rectangle(args)` | `UiLayoutGetElementRectangleArgs` | — | `UiLayoutGetElementRectangleResponse` |
| `ak.wwise.ui.layout.getLayout` | `client.wwise().ui().layout().get_layout(args)` | `UiLayoutGetLayoutArgs` | — | `UiLayoutGetLayoutResponse` |
| `ak.wwise.ui.layout.getLayoutNames` | `client.wwise().ui().layout().get_layout_names()` | — | — | `UiLayoutGetLayoutNamesResponse` |
| `ak.wwise.ui.layout.getOrCreateView` | `client.wwise().ui().layout().get_or_create_view(args)` | `UiLayoutGetOrCreateViewArgs` | — | `UiLayoutGetOrCreateViewResponse` |
| `ak.wwise.ui.layout.getViewHandle` | `client.wwise().ui().layout().get_view_handle(args)` | `UiLayoutGetViewHandleArgs` | — | `()` |
| `ak.wwise.ui.layout.getViewInstances` | `client.wwise().ui().layout().get_view_instances(args)` | `UiLayoutGetViewInstancesArgs` | — | `UiLayoutGetViewInstancesResponse` (`ViewInstance`) |
| `ak.wwise.ui.layout.getViewTypes` | `client.wwise().ui().layout().get_view_types()` | — | — | `UiLayoutGetViewTypesResponse` (`ViewTypeInfo`) |
| `ak.wwise.ui.layout.moveSplitter` | `client.wwise().ui().layout().move_splitter(args)` | `UiLayoutMoveSplitterArgs` | — | `()` |
| `ak.wwise.ui.layout.removeLayout` | `client.wwise().ui().layout().remove_layout(args)` | `UiLayoutRemoveLayoutArgs` | — | `()` |
| `ak.wwise.ui.layout.setLayout` | `client.wwise().ui().layout().set_layout(args)` | `UiLayoutSetLayoutArgs` | — | `()` |
| `ak.wwise.ui.layout.switchLayout` | `client.wwise().ui().layout().switch_layout(args)` | `UiLayoutSwitchLayoutArgs` | — | `()` |
| `ak.wwise.ui.layout.undockView` | `client.wwise().ui().layout().undock_view(args)` | `UiLayoutUndockViewArgs` | — | `()` |
| `ak.wwise.ui.model.createHandle` | `client.wwise().ui().model().create_handle(args)` | `UiModelCreateHandleArgs` | — | `UiModelCreateHandleResponse` |
| `ak.wwise.ui.model.destroyHandle` | `client.wwise().ui().model().destroy_handle(args)` | `UiModelDestroyHandleArgs` | — | `()` |
| `ak.wwise.ui.model.getState` | `client.wwise().ui().model().get_state(args)` | `UiModelGetStateArgs` (`WidgetStateQuery`) | — | `()` |
| `ak.wwise.ui.model.registerWafm` | `client.wwise().ui().model().register_wafm(args)` | `UiModelRegisterWafmArgs` | — | `UiModelRegisterWafmResponse` |
| `ak.wwise.ui.model.setState` | `client.wwise().ui().model().set_state(args)` | `UiModelSetStateArgs` (`WidgetStateUpdate`) | — | `()` |
| `ak.wwise.ui.project.create` | `client.wwise().ui().project().create(args)` | `UiProjectCreateArgs` (`PlatformsToCreate`) | — | `()` |
| `ak.wwise.ui.signal.emit` | `client.wwise().ui().signal().emit(args)` | `UiSignalEmitArgs` | — | `()` |
| `ak.wwise.ui.window.close` | `client.wwise().ui().window().close(args)` | `UiWindowCloseArgs` | — | `()` |
| `ak.wwise.ui.window.create` | `client.wwise().ui().window().create(args)` | `UiWindowCreateArgs` (`WindowSize`) | — | `UiWindowCreateResponse` |
| `ak.wwise.ui.window.present` | `client.wwise().ui().window().present(args)` | `UiWindowPresentArgs` | — | `()` |
| `ak.wwise.waapi.getFunctions` | `client.wwise().waapi().get_functions()` | — | — | `WaapiGetFunctionsResponse` |
| `ak.wwise.waapi.getSchema` | `client.wwise().waapi().get_schema(args)` | `WaapiGetSchemaArgs` | — | `WaapiGetSchemaResponse` |
| `ak.wwise.waapi.getTopics` | `client.wwise().waapi().get_topics()` | — | — | `WaapiGetTopicsResponse` |

### Typed Topics

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

---

## 6. Intentionally Dynamic / Untyped Fields (`serde_json::Value`)

All **180 WAAPI functions** and **35 subscription topics** are generated with typed `Args`, `Options`, `Response`, and `Payload` structs and chained client methods (`EXCLUDED_ENDPOINTS` and `EXCLUDED_TOPICS` in `tools/overrides.json` are both empty).

However, inside **10 endpoints**, specific payload fields are intentionally typed as `serde_json::Value` (or `Vec<serde_json::Value>`) rather than fixed Rust structs because the official Wwise JSON Schema defines them as open-ended, arbitrary, or self-referential JSON documents:

| WAAPI URI | Field(s) kept as `serde_json::Value` | Why it is not mapped to a fixed Rust struct |
|---|---|---|
| `ak.wwise.core.executeLuaScript` | `CoreExecuteLuaScriptResponse::result` | Returns whatever arbitrary value (`boolean`, `number`, `string`, `array`, or `object` table) the user's Lua script returns. |
| `ak.wwise.ui.cli.executeLuaScript` | `UiCliExecuteLuaScriptResponse::result` | Schema defines `result` as `{"type": "any"}` for arbitrary Lua script return values. |
| `ak.wwise.debug.validateCall` | `DebugValidateCallArgs::{args, options, result}` | Debug endpoint that validates arbitrary raw JSON `args`, `options`, and `result` payloads against any WAAPI URI's schema. |
| `ak.wwise.debug.getWalTree` | `DebugGetWalTreeResponse::tree` | Internal Wwise Authoring Layer debug dump whose schema is an open-ended `{"type": "object"}` with no fixed properties. |
| `ak.wwise.waapi.getSchema` | `WaapiGetSchemaResponse::{args_schema, options_schema, result_schema, publish_schema, examples}` | Returns raw JSON Schema draft-04 documents and example JSON payloads for a given WAAPI URI. |
| `ak.wwise.core.mediaPool.get` | `CoreMediaPoolGetResponse::results` (`Vec<serde_json::Value>`) | Each returned item is a dynamic key/value map keyed by whichever arbitrary media-pool field strings (including custom user metadata columns) were requested in `CoreMediaPoolGetOptions::return_fields`. |
| `ak.wwise.ui.layout.getLayout` / `ak.wwise.ui.layout.setLayout` | `UiLayoutGetLayoutResponse::layout`, `UiLayoutSetLayoutArgs::layout` | WAFM docking layout tree (`wafm_definitions.json`), an opaque recursive UI docking specification intended to be saved from `getLayout` and passed back to `setLayout`. |
| `ak.wwise.ui.model.registerWafm` | `UiModelRegisterWafmArgs::wafm` | Accepts either a file path string or an arbitrary inline WAFM UI model JSON document (`oneOf` string or raw WAFM object). |
| `ak.wwise.core.soundbank.generated` | `CoreSoundbankGeneratedPayload::{platform, language, bank_info}` | Wwise's `publishSchema` defines `platform`, `language`, and `bankInfo` items as unschematized `{"type": "object"}` nodes without properties (whereas `soundbank: WwiseObject`, `bank_data: Option<SoundBankGeneratedBankData>`, `plugin_info: Option<SoundBankPluginInfo>`, and `error: Option<String>` on the same payload are strongly typed). |

