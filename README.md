# wwise-waapi

[![Docs (GitHub Pages)](https://img.shields.io/badge/docs-github%20pages-blue)](https://tronhjem.github.io/rust-waapi-client/wwise_waapi/)
[![docs.rs](https://docs.rs/wwise-waapi/badge.svg)](https://docs.rs/wwise-waapi)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Async, compile-time checked, and strongly typed Rust client for the Wwise Authoring API (WAAPI).
- **API Documentation (Rustdoc):** https://tronhjem.github.io/rust-waapi-client/wwise_waapi/
- **Wwise WAAPI Reference:** https://www.audiokinetic.com/en/public-library/2025.1.4_9062/?source=SDK&id=waapi.html

## Why `wwise-waapi`?

Previously I mostly wrote Wwise automation and tooling in Python using raw JSON dictionaries. This creates problems where a misspelled URI, wrong argument shape, invalid property name, or Wwise version rename only fails at runtime. `wwise-waapi` brings **compile-time verification** and **native performance** to Wwise tooling and companion apps:

- **Compile-Time Checked & Type-Safe:** All 180 WAAPI functions, 35 subscription topics, 77 Wwise `ObjectType` variants, and 493 `Property` accessors are generated directly from Wwise's schemas and `WObjects.xml`. Invalid arguments, missing required fields, or mistyped property and object names are caught by the Rust compiler before your tool ever runs.
- **Multi-Version Wwise 2024 & 2025+ Support:** Generated from the union of Wwise 2024 and 2025+ schemas so a single typed API works seamlessly across Wwise versions (including 2025 hierarchy and `ActorMixer` -> `PropertyContainer` renames).
- **Significantly Faster than Python (`waapi-client`):** Built on a lightweight, low-latency WAMP v2 transport with single-pass `serde` serialization and async request pipelining - delivering **~3x faster per-call roundtrips** (`1.99 ms` vs `5.88 ms` on a 186-object `object.get` query) and **~8.5x faster end-to-end CLI execution** (`9.2 ms` vs `77.4 ms` from process start to exit).

## Basic Usage

`wwise-waapi` is async and built on `tokio`. Import `wwise_waapi::prelude::*` for the most common client, query, object, property, and SoundEngine types.

### Connecting & Calling the Typed API

Typed endpoints are exposed via a **chained namespace API** on `WaapiClient` that mirrors WAAPI URI dot-notation (`client.ak().wwise()...`, plus top-level shortcuts `client.wwise()`, `client.core()`, `client.object()`, `client.ui()`, and `client.soundengine()`). All client and namespace handles take `&self` (`Copy`), so they can be stored and reused freely:

```rust
use wwise_waapi::api::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectGetArgs, CoreObjectGetOptions,
};
use wwise_waapi::prelude::*;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    // Connect to WAAPI (default: ws://127.0.0.1:8080/waapi)
    let client = WaapiClient::connect().await?;

    let info = client.core().get_info().await?;
    println!("Connected to {} ({})", info.display_name, info.version.display_name);

    // Query objects by path, GUID, ObjectType, or WAQL via ObjectQuery / CoreObjectGetArgs::from_waql
    let response = client
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::path(r"\Containers\Default Work Unit")),
            CoreObjectGetOptions::new().return_fields([
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
            CoreObjectCreateArgs::new(
                ObjectRef::path(r"\Containers\Default Work Unit"),
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

See `examples/object_set.rs` for renaming or setting properties on Wwise objects (`ak.wwise.core.object.set`):

```rust
use wwise_waapi::api::ak::wwise::core::object::{
    CoreObjectSetArgs, CoreObjectSetItem, CoreObjectSetOptions,
};
use wwise_waapi::prelude::*;

let client = WaapiClient::connect().await?;

let item = CoreObjectSetItem::new(ObjectRef::path(r"\Containers\Default Work Unit\MySound"))
    .name("MyRenamedSound")
    .property(Property::Volume, -6.0);

let options = CoreObjectSetOptions::new().return_fields([
    ObjectReturnField::Id,
    ObjectReturnField::Name,
    ObjectReturnField::Property(Property::Volume),
]);

let response = client
    .object()
    .set(CoreObjectSetArgs::from_item(item), options)
    .await?;
client.shutdown().await?;
```

### Querying Objects (`ObjectQuery` vs WAQL) vs Addressing a Single Object (`ObjectRef`)

WAAPI distinguishes between **multi-object search queries** (`ak.wwise.core.object.get`) and **single-object target references** (`create`, `set`, `delete`, `move_`, `copy`, etc.):

- **`CoreObjectGetArgs` (`ak.wwise.core.object.get`)** queries one or more objects via `ObjectQuery` (`from`) or a WAQL string (`waql`):
  ```rust
  use wwise_waapi::api::ak::wwise::core::object::{CoreObjectGetArgs, CoreObjectGetOptions};
  use wwise_waapi::prelude::*;

  // 1. Query by hierarchy path, GUID, type, or search text using ObjectQuery:
  let by_path = CoreObjectGetArgs::new(ObjectQuery::path(r"\Containers\Default Work Unit"))
      .transform(vec![Transform::select(TransformSelect::Children)]);
  let by_type = CoreObjectGetArgs::new(ObjectQuery::of_type(ObjectType::Sound));

  // 2. Or query with a WAQL expression:
  let by_waql = CoreObjectGetArgs::from_waql("$ from type Sound where Volume < 0");

  let response = client
      .object()
      .get(by_waql, CoreObjectGetOptions::basic())
      .await?;
  ```
- **`ObjectRef`** identifies a specific single Wwise object by GUID, path, or qualified name:
  ```rust
  let by_path = ObjectRef::path(r"\Containers\Default Work Unit");
  let by_guid = ObjectRef::guid("{F546017D-201A-49BD-8D4E-0A28F5DBB28D}");
  let by_name = ObjectRef::name("Sound:MySound");
  ```

### SoundEngine: Game Objects, 3D Positions & Posting Events

See `examples/post_event.rs`, `examples/set_position.rs`, and `examples/register_and_post_event.rs` for driving the Wwise SoundEngine over WAAPI:

```rust
use wwise_waapi::api::ak::soundengine::{
    SoundenginePostEventArgs, SoundengineRegisterGameObjArgs, SoundengineSetPositionArgs,
};
use wwise_waapi::prelude::*;

let se = client.soundengine();
let emitter = GameObjectRef::id(100);

se.register_game_obj(SoundengineRegisterGameObjArgs::new(emitter, "PlayerEmitter"))
    .await?;

se.set_position(SoundengineSetPositionArgs::new(
    emitter,
    Position::new(Vector3::new(0.0, 1.5, 10.0)),
))
.await?;

// Address events/switches/states/RTPCs by name, GUID, or short ID:
let response = se
    .post_event(
        SoundenginePostEventArgs::new(SoundEngineObjectRef::name("Play_TestSound"))
            .game_object(emitter),
    )
    .await?;
println!("Playing ID: {}", response.playing_id);
```

### Typed Subscriptions

Subscribe to WAAPI topics using the typed subscription methods (`subscribe_*`):

```rust
use wwise_waapi::api::ak::wwise::core::object::CoreObjectCreatedOptions;
use wwise_waapi::prelude::*;

let options = CoreObjectCreatedOptions::new()
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

## API Overview

All **180 WAAPI functions** and **35 subscription topics** across the union of Wwise 2024 and Wwise 2025+ schemas are generated into typed structs, chained call methods, and typed `subscribe_*` methods under `wwise_waapi::api::ak`:

- **Chained Namespace API:** Mirrors WAAPI URI dot-notation (`ak.wwise.core.object.get` -> `client.wwise().core().object().get(...)`, with shortcuts `client.wwise()`, `client.core()`, `client.object()`, `client.ui()`, and `client.soundengine()`).
- **Ergonomic Types & Builders:** Each endpoint module exports canonical `<Base>{Args,Options,Response,Payload}` types (`CoreObjectGetArgs`, `CoreObjectGetOptions`, `CoreObjectGetResponse`, `CoreObjectCreatedPayload`). Domain reference and query types (`ObjectRef::path/guid/name`, `ObjectQuery::path/id/of_type/search`, `CoreObjectGetArgs::from_waql`, `SoundEngineObjectRef::name/guid/short_id`, `GameObjectRef::id`, `PlatformRef::name/guid`, `LanguageRef::name/guid`, `CoreObjectSetArgs::from_item`) use explicit named constructors matching the WAAPI specification.
- **Strongly-Typed Wwise Schema Enums:**
  - `ObjectType` — All Wwise object types from `WObjects.xml` (`ObjectType::Sound`, `ObjectType::PropertyContainer` / `ObjectType::ActorMixer`, `ObjectType::Event`, etc.).
  - `Property` — All 460+ Wwise object properties, references, and object lists (`Property::Volume`, `Property::Pitch`, `Property::OutputBus`, etc.).
  - `ObjectReturnField` — Typed return-field selectors (`Id`, `Name`, `Type`, `Path`, `ObjectReturnField::Property(Property::Volume)`, etc.).
- **Multi-Version Union (Wwise 2024 + 2025+):** A single unified API supports both Wwise 2024 and Wwise 2025+ (keeping renamed variants like `ObjectType::ActorMixer` and `ObjectType::PropertyContainer` as aliases sharing `class_id()` `524304`).
- **Raw URI Constants & Escape Hatch:** Every URI is also available as a `WaapiFunction` / `WaapiTopic` enum variant and as a `&str` constant under `wwise_waapi::waapi_function_api::ak` and `wwise_waapi::waapi_topics_api::ak`.

### Intentionally Dynamic Payloads (`serde_json::Value`)

All **180 WAAPI functions** and **35 subscription topics** have generated typed `Args`, `Options`, `Response`, and `Payload` wrappers and chained client methods (`0` endpoints are excluded). Within **10 endpoints**, specific payload fields are intentionally kept as `serde_json::Value` because Wwise's schema defines them as arbitrary, user-defined, or opaque JSON documents:

- **Lua script execution** (`ak.wwise.core.executeLuaScript`, `ak.wwise.ui.cli.executeLuaScript`) — `result` returns whatever arbitrary value or table a user Lua script produces.
- **Schema & call introspection** (`ak.wwise.waapi.getSchema`, `ak.wwise.debug.validateCall`, `ak.wwise.debug.getWalTree`) — raw JSON Schema documents, arbitrary call validation payloads, and internal Authoring Layer debug trees with no fixed schema properties.
- **Dynamic media-pool query rows** (`ak.wwise.core.mediaPool.get`) — `CoreMediaPoolGetArgs` (`MediaPoolFilterEntry`) and `CoreMediaPoolGetOptions` are strongly typed, while `results` is `Vec<serde_json::Value>` because each entry is keyed dynamically by whichever media-pool/user metadata field names were requested.
- **Opaque WAFM UI layout & model documents** (`ak.wwise.ui.layout.getLayout`, `ak.wwise.ui.layout.setLayout`, `ak.wwise.ui.model.registerWafm`) — recursive WAFM docking layout trees and inline UI model definitions designed to be round-tripped as JSON documents.
- **Unschematized sub-objects in `ak.wwise.core.soundbank.generated`** — `soundbank`, `bank_data` (`SoundBankGeneratedBankData`), and `plugin_info` (`SoundBankPluginInfo`) are strongly typed, while `platform`, `language`, and `bank_info` are defined in Wwise's schema as bare `{"type": "object"}` nodes without properties.

For the complete 180-function and 35-topic tables, JSON Schema mapping rules, and the full field-by-field breakdown of dynamic payloads, see:
- **[Typed API Design & Endpoint Reference (`docs/API_REFERENCE.md`)](docs/API_REFERENCE.md)**
- **[Interactive Rustdoc Documentation](https://tronhjem.github.io/rust-waapi-client/wwise_waapi/)**

## Examples

Run any example in [`examples/`](examples/) with `cargo run --example <name>` (requires Wwise running with WAAPI enabled on `127.0.0.1:8080`):

| Command | Description |
|---|---|
| `cargo run --example get_info` | Connect to WAAPI and print Wwise version, platform, and install directory (`ak.wwise.core.getInfo`). |
| `cargo run --example chained_api` | Query objects under `\Containers` and inspect the currently selected UI objects (`client.ak().wwise()...`, `client.object()`, `client.ui()`). |
| `cargo run --example query_project` | Query the project root with WAQL (`CoreObjectGetArgs::from_waql`) and print its name, file path, and dirty state. |
| `cargo run --example object_get_properties` | Query all `Sound` objects with WAQL and read their `@Volume` property and `@OutputBus` reference. |
| `cargo run --example create_object` | Create a `RandomSequenceContainer` with two `ChildObject` `Sound` children under `\Containers\Default Work Unit` (`ak.wwise.core.object.create`). |
| `cargo run --example object_set` | Rename an object and set `@Volume` under `\Containers\Default Work Unit` (`ak.wwise.core.object.set`, Wwise 2025+). |
| `cargo run --example object_set_2024` | Wwise 2024 variant of `object_set` using `\Actor-Mixer Hierarchy\Default Work Unit`. |
| `cargo run --example object_get_set` | Query `\Containers\Default Work Unit`, inspect the selected object in the Wwise UI, and update its notes and `@Volume` (Wwise 2025+). |
| `cargo run --example object_get_set_2024` | Wwise 2024 variant of `object_get_set` using `\Actor-Mixer Hierarchy\Default Work Unit`. |
| `cargo run --example post_event` | Post an Event to the SoundEngine by name, Short ID, or GUID (`SoundEngineObjectRef::name/short_id/guid`). |
| `cargo run --example set_position` | Set a game object's 3D position and orientation vectors (`SoundengineSetPositionArgs`, `Position::with_orientations`, `Vector3`). |
| `cargo run --example register_and_post_event` | Register listener & emitter game objects, configure default listeners, set 3D positions with `Vector3` math, post an Event, and unregister. |
| `cargo run --example subscribe` | Subscribe to `ak.wwise.core.object.created` using `client.object().subscribe_created(...)` and print the typed `CoreObjectCreatedPayload`. |
| `cargo run --example raw_call` | Call WAAPI endpoints using the raw JSON escape hatch (`client.call` with `WaapiFunction` and `wwise_waapi::json!`). |

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
