# waapi-client

A Rust client library for the Wwise Authoring API (WAAPI), providing a clean interface for interacting with Audiokinetic Wwise.
More information about WAAPI can be found here: https://www.audiokinetic.com/en/public-library/2025.1.4_9062/?source=SDK&id=waapi.html

This crate ships with a small, private WAMP v2 client that speaks JSON over WebSocket. It is no longer dependent on an external `wampire` submodule.

## Basic Usage

`waapi-client` is async and built on `tokio`.

### Connecting to WAAPI

```rust
use waapi_client::{WaapiClient, WaapiResult};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    // Connect to WAAPI (default: ws://127.0.0.1:8080/waapi)
    let mut client = WaapiClient::new(None).await?;

    // Make calls...

    // Always shutdown when done
    client.shutdown().await?;
    Ok(())
}
```

`WaapiClient::new` accepts `None` for the default URL, a `host:port` string, or a full WebSocket URI such as `ws://127.0.0.1:8080/waapi`.

### Making Simple Calls

The JSON value API works with `WaapiFunction` and `serde_json::Value` (use `call_raw` if you need to pass an arbitrary `&str`). The `json!` macro is re-exported from the crate root, so applications can build JSON kwargs without adding a direct `serde_json` dependency:

```rust
use waapi_client::{WaapiFunction, json};

let response = client
    .call(WaapiFunction::ak_wwise_core_getInfo, None, None)
    .await?;
println!("{}", waapi_client::serde_json::to_string_pretty(&response)?);
```

### Building Arguments

Use `waapi_client::json!` to build WAAPI kwargs:

```rust
use waapi_client::{WaapiFunction, json};

let args = json!({
    "from": {
        "path": [r"\Actor-Mixer Hierarchy\Default Work Unit"]
    }
});

let options = json!({
    "return": ["id", "name", "type"]
});

let response = client
    .call(WaapiFunction::ak_wwise_core_object_get, Some(args), Some(options))
    .await?;
```

### Typed API

Typed endpoints are exposed via a **chained namespace API** on `WaapiClient` that mirrors WAAPI URI dot-notation (`client.ak().wwise()...`, plus top-level shortcuts `client.wwise()` and `client.soundengine()`). Each chained method delegates to `WaapiClient::call_typed` or `WaapiClient::call_typed_void`, which serializes typed `Args`/`Options` structs and deserializes the typed `Response`:

```rust
use waapi_client::api::generated::ak::wwise::core::CoreGetInfoResponse;

let info: CoreGetInfoResponse = client.wwise().core().get_info().await?;
println!("Wwise version: {:?}", info.version.display_name);
```

```rust
use waapi_client::api::generated::ak::wwise::core::object::{
    CoreObjectGetArgs, CoreObjectGetOptions,
};
use waapi_client::api::generated::primitives::{ObjectQuery, ObjectReturnField};

let args = CoreObjectGetArgs::new(ObjectQuery::from(
    r"\Actor-Mixer Hierarchy\Default Work Unit\MySound",
));

let options = CoreObjectGetOptions::new()
    .return_fields([ObjectReturnField::Id, ObjectReturnField::Name, ObjectReturnField::Type]);

let response = client.wwise().core().object().get(args, options).await?;
```

---

## Typed API Design & JSON Schema Mapping

All generated bindings in `src/waapi_function_api.rs`, `src/waapi_topics_api.rs`, and `src/api/generated/` are derived offline from the Wwise JSON schemas stored in `tools/waapi_schemas/<version>/` (fetched from a running Wwise instance via `tools/get_waapi_definitions.py`).

### 1. URI-to-Chain and Module Mapping

Every WAAPI URI is split on `.`:
- The **prefix segments** map 1-to-1 to both the Rust module path (`waapi_client::api::generated::ak::...`) and zero-cost chained borrower structs (`AkClient<'a>`, `WwiseClient<'a>`, `WwiseCoreClient<'a>`, `WwiseCoreObjectClient<'a>`, etc.) in `src/api/generated/clients.rs`.
- The **final segment** becomes the `snake_case` async method on that namespace client.
- `client.wwise()` and `client.soundengine()` are provided on `WaapiClient` as convenient shortcuts for `client.ak().wwise()` and `client.ak().soundengine()`.

| WAAPI URI | Chained Call | Module Path |
|---|---|---|
| `ak.wwise.core.getInfo` | `client.wwise().core().get_info()` | `api::generated::ak::wwise::core` |
| `ak.wwise.core.object.get` | `client.wwise().core().object().get(args, options)` | `api::generated::ak::wwise::core::object` |
| `ak.soundengine.postEvent` | `client.soundengine().post_event(args)` | `api::generated::ak::soundengine` |

### 2. Type Naming Convention (`<Base>{Args,Options,Response}`)

For each endpoint URI, the generator strips the leading `ak.wwise.` prefix (or `ak.` for non-`wwise` namespaces like `ak.soundengine`), splits camelCase words, joins all remaining segments in `PascalCase` as `<Base>`, and emits up to three types:
- `<Base>Args` — request keyword arguments (`ArgumentsKw`)
- `<Base>Options` — request WAMP call/subscribe options (`Options`)
- `<Base>Response` — deserialized response payload

For example:
- `ak.wwise.core.getInfo` → `CoreGetInfoResponse`
- `ak.wwise.core.object.get` → `CoreObjectGetArgs`, `CoreObjectGetOptions`, `CoreObjectGetResponse`
- `ak.wwise.ui.getSelectedObjects` → `UiGetSelectedObjectsOptions`, `UiGetSelectedObjectsResponse`
- `ak.soundengine.setRTPCValue` → `SoundengineSetRtpcValueArgs`

### 3. How Wwise JSON Schema Sections Map to Rust

Each `tools/waapi_schemas/<version>/ak.*.json` file contains either RPC function schemas (`argsSchema`, `optionsSchema`, `resultSchema`) or topic schemas (`publishSchema`, `optionsSchema`):

| JSON Schema Section | Rust Output | Rules |
|---|---|---|
| `argsSchema` | `<Base>Args` (`#[derive(Serialize)]`) | **Required properties** (`required` array) become positional parameters on `<Base>Args::new(...)`. **Optional properties** become `Option<T>` fields with `#[serde(skip_serializing_if = "Option::is_none")]` and chainable builder methods `.field(val)`. If `argsSchema` has no properties, no `<Base>Args` type is emitted and the client method takes no `args` parameter. |
| `optionsSchema` | `<Base>Options` (`#[derive(Serialize)]`) | Generated with `::new()` (`Default`) and chainable builder methods. When `optionsSchema` is `{"$ref": "waapi_definitions.json#/definitions/objectReturnOptions"}`, `<Base>Options` is emitted as `pub type <Base>Options = ObjectReturnOptions;`. If `optionsSchema` is empty, no `<Base>Options` type is emitted and the client method takes no `options` parameter. |
| `resultSchema` | `<Base>Response` (`#[derive(Serialize, Deserialize)]`) | Response fields are generated from `resultSchema.properties`. When present, the chained method calls `WaapiClient::call_typed` and returns `WaapiResult<<Base>Response>`. When `resultSchema` has no properties, no `<Base>Response` is emitted and the method calls `WaapiClient::call_typed_void`, returning `WaapiResult<()>`. |
| `publishSchema` | `WaapiTopic` + topic event payload | Identifies the schema file as a **subscription topic** (34 topics) rather than an RPC function (170 functions). `tools/generate_waapi_uri.py` emits it into `WaapiTopic` (`src/waapi_topics_api.rs`). Subscriptions currently use `client.subscribe(WaapiTopic::..., ...)` or `client.subscribe_typed(WaapiTopic::..., Some(options))`, and `publishSchema` + `optionsSchema` define the schema for typed topic subscriptions. |

### 4. Shared `$ref` Definitions, Return Fields, and `@Property` Extensions

- **`Definitions/*.json` `$ref` resolution**: Shared references across `common_definitions.json`, `waapi_definitions.json`, `waapi_ui_definitions.json`, and `wafm_definitions.json` map to shared primitives in `src/api/primitives.rs` and `src/api/generated/primitives.rs` (`ObjectRef`, `PlatformRef`, `SoundEngineObjectRef`, `GameObjectRef`, `LanguageRef`, `WwiseObject`, `WwiseObjectRef`, `OnNameConflict`, `ListMode`, `ImportOperation`, `LogSeverity`, `PropertyValue`, `ChildObject`).
- **Domain-specific return field enums**: Rather than untyped strings, `waapi_definitions.json` return expressions are generated as strongly-typed enums:
  - `ObjectReturnField` (wrapped by `ObjectReturnOptions`) — all built-in Wwise object accessors (`Id`, `Name`, `Notes`, `Type`, `Path`, `Parent`, `ShortId`, etc.), `ObjectReturnField::Property(Property)` (which serializes as `"@Property"`), plus `ObjectReturnField::Custom(String)` (and `From<&str>` / `From<String>`) for arbitrary dot-accessor queries.
  - `BusReturnField`, `AudioObjectReturnField`, and `VoiceReturnField` for profiler queries.
- **Generated `Property` enum (`WObjects.xml`)**: `src/api/generated/properties.rs` (re-exported as `waapi_client::Property` and `waapi_client::api::Property`) is generated from `Definitions/WObjects.xml` and contains all 464 Wwise object properties, references, and object lists (e.g. `Property::Volume`, `Property::Pitch`, `Property::Lowpass`, `Property::OutputBus`, `Property::RTPC`). `Property::as_str()` returns the bare `&'static str` (`"Volume"`), `Property::as_at_str()` returns the `@`-prefixed `&'static str` (`"@Volume"`), and `Property` implements `AsRef<str>`, `Display`, `Serialize`, and `Deserialize`.
- **Dynamic `@Property` fields (`patternProperties`)**: Schemas that accept or return open-ended `@`-prefixed properties (such as `CoreObjectSetItem`, `CoreObjectCreateArgs`, `ImportItem`, `ImportDefaults`, `WwiseObject`, and `CoreObjectSetObject`) include `#[serde(flatten)] pub extra: HashMap<String, serde_json::Value>` along with `.property(Property::Volume, value)` / `.extra(key, value)` builder methods on request structs and `.property(Property::Volume) -> Option<&serde_json::Value>` lookup helpers on response structs.
- **Field naming**: JSON `camelCase` wire names are converted to Rust `snake_case` with `#[serde(rename = "...")]` emitted automatically whenever the wire name differs (e.g. `type` → `object_type`, `return` → `objects` / `playing_id` / `return_fields`).

### 5. Currently Generated Typed Endpoints

The following 10 endpoints currently have generated typed structs and chained methods under `waapi_client::api::generated`:

| WAAPI URI | Chained Method on `WaapiClient` | Module (`waapi_client::api::generated::*`) | `Args` Type | `Options` Type | `Response` Type |
|---|---|---|---|---|---|
| `ak.wwise.core.getInfo` | `client.wwise().core().get_info()` | `ak::wwise::core` | — | — | `CoreGetInfoResponse` |
| `ak.wwise.core.object.get` | `client.wwise().core().object().get(args, options)` | `ak::wwise::core::object` | `CoreObjectGetArgs` | `CoreObjectGetOptions` (`ObjectReturnOptions`) | `CoreObjectGetResponse` |
| `ak.wwise.core.object.set` | `client.wwise().core().object().set(args, options)` | `ak::wwise::core::object` | `CoreObjectSetArgs` + `CoreObjectSetItem` | `CoreObjectSetOptions` (`ObjectReturnOptions`) | `CoreObjectSetResponse` + `CoreObjectSetObject` |
| `ak.wwise.core.object.create` | `client.wwise().core().object().create(args, options)` | `ak::wwise::core::object` | `CoreObjectCreateArgs` | `CoreObjectCreateOptions` (`ObjectReturnOptions`) | `CoreObjectCreateResponse` |
| `ak.wwise.core.object.delete` | `client.wwise().core().object().delete(args)` | `ak::wwise::core::object` | `CoreObjectDeleteArgs` | — | `()` |
| `ak.wwise.core.audio.import` | `client.wwise().core().audio().import(args, options)` | `ak::wwise::core::audio` | `CoreAudioImportArgs` (`ImportItem`, `ImportDefaults`) | `CoreAudioImportOptions` (`ObjectReturnOptions`) | `CoreAudioImportResponse` (`ImportLogEntry`) |
| `ak.wwise.core.project.save` | `client.wwise().core().project().save(args)` | `ak::wwise::core::project` | `CoreProjectSaveArgs` | — | `()` |
| `ak.wwise.ui.getSelectedObjects` | `client.wwise().ui().get_selected_objects(options)` | `ak::wwise::ui` | — | `UiGetSelectedObjectsOptions` (`ObjectReturnOptions`) | `UiGetSelectedObjectsResponse` |
| `ak.soundengine.postEvent` | `client.soundengine().post_event(args)` | `ak::soundengine` | `SoundenginePostEventArgs` | — | `SoundenginePostEventResponse` |
| `ak.soundengine.setRTPCValue` | `client.soundengine().set_rtpc_value(args)` | `ak::soundengine` | `SoundengineSetRtpcValueArgs` | — | `()` |

For any of the 170 WAAPI functions or 34 topics not yet in the table above, use `client.call(WaapiFunction::..., ...)` / `client.call_typed(WaapiFunction::..., ...)` or `client.subscribe(WaapiTopic::..., ...)` / `client.subscribe_typed(WaapiTopic::..., ...)`.

### Specifying Return Fields

For escape-hatch calls, return fields are a plain `Vec<String>` in the options object:

```rust
let options = json!({
    "return": ["id", "name", "type", "path"]
});
```

Typed endpoint `Options` structs provide a `.return_fields(...)` builder for the same purpose.

### Complete Example

```rust
use waapi_client::{WaapiClient, WaapiFunction, WaapiResult, json};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    let mut client = WaapiClient::new(Some("127.0.0.1:8080")).await?;

    let args = json!({
        "from": { "path": [r"\Actor-Mixer Hierarchy\Default Work Unit"] }
    });
    let options = json!({
        "return": ["id", "name", "type"]
    });

    let response = client
        .call(WaapiFunction::ak_wwise_core_object_get, Some(args), Some(options))
        .await?;

    println!("{}", waapi_client::serde_json::to_string_pretty(&response)?);

    client.shutdown().await?;
    Ok(())
}
```

### Setting Object Properties

See `examples/object_set.rs` for setting properties on Wwise objects using the generated builders:

```rust
use waapi_client::WaapiClient;
use waapi_client::api::generated::ak::wwise::core::object::{
    CoreObjectSetArgs, CoreObjectSetItem, CoreObjectSetOptions,
};
use waapi_client::api::generated::primitives::ObjectReturnField;
use waapi_client::api::{ObjectRef, Property};

let mut client = WaapiClient::new(None).await?;

let args = CoreObjectSetArgs::new(vec![
    CoreObjectSetItem::new(ObjectRef::path(
        r"\Actor-Mixer Hierarchy\Default Work Unit\MySound",
    ))
    .name("MyRenamedSound")
    .property(Property::Volume, -6.0),
]);

let options = CoreObjectSetOptions::new().return_fields([
    ObjectReturnField::Id,
    ObjectReturnField::Name,
    ObjectReturnField::Property(Property::Volume),
]);

let response = client.wwise().core().object().set(args, options).await?;
client.shutdown().await?;
```

### Subscriptions

Subscribe to WAAPI topics and receive events on an async channel:

```rust
use waapi_client::{WaapiTopic, json};

let options = json!({ "return": ["id", "name"] });
let (id, mut events) = client
    .subscribe(WaapiTopic::ak_wwise_core_object_created, Some(options))
    .await?;

while let Some(event) = events.recv().await {
    println!("{}", waapi_client::serde_json::to_string_pretty(&event.kwargs)?);
}

client.unsubscribe(id).await?;
```

## API Endpoints

All WAAPI endpoints are available in the `waapi_function_api::ak` module as constants:

```rust
use waapi_client::waapi_function_api::ak;

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

All 170+ WAAPI endpoints are available. See [`src/waapi_function_api.rs`](src/waapi_function_api.rs) for the complete list organized by module.

## Examples

Run examples with:

```bash
cargo run --example get_info
cargo run --example chained_api
cargo run --example object_get_set
cargo run --example object_set
cargo run --example raw_call
cargo run --example subscribe
```

See the [`examples/`](examples/) directory for more usage examples. Most examples require Wwise running with WAAPI enabled on `127.0.0.1:8080`.

## Error Handling

The library uses a custom `WaapiError` enum with the following variants:

- `Connection(String)` - Failed to connect to WAAPI or client already shut down
- `Serialization(serde_json::Error)` - JSON serialization/deserialization failed
- `Call(String)` - WAAPI call returned an error (includes the error URI, args, and kwargs)
- `Wamp(WampError)` - Underlying WAMP/WebSocket protocol error

Always call `shutdown().await` before dropping the client to ensure clean disconnection.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are more than welcome!
