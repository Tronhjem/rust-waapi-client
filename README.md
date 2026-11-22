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

## API Overview

All **180 WAAPI functions** and **35 subscription topics** across the union of Wwise 2024 and Wwise 2025+ schemas are generated into typed structs, chained call methods, and typed `subscribe_*` methods under `wwise_waapi::api::ak`:

- **Chained Namespace API:** Mirrors WAAPI URI dot-notation (`ak.wwise.core.object.get` -> `client.wwise().core().object().get(...)`, with shortcuts `client.wwise()`, `client.core()`, `client.object()`, `client.ui()`, and `client.soundengine()`).
- **Ergonomic Types & Builders:** Each endpoint module exports both canonical (`CoreObjectGetArgs`) and short module-scoped (`GetArgs`, `GetOptions`, `GetResponse`, `CreatedPayload`) type names. Constructors and client methods accept `impl Into<...>` for strings, paths, GUIDs, references, and options (`ObjectReturnOptions::basic()`, arrays `[ObjectReturnField::...]`, or `()`).
- **Strongly-Typed Wwise Schema Enums:**
  - `ObjectType` — All Wwise object types from `WObjects.xml` (`ObjectType::Sound`, `ObjectType::PropertyContainer` / `ObjectType::ActorMixer`, `ObjectType::Event`, etc.).
  - `Property` — All 460+ Wwise object properties, references, and object lists (`Property::Volume`, `Property::Pitch`, `Property::OutputBus`, etc.).
  - `ObjectReturnField` — Typed return-field selectors (`Id`, `Name`, `Type`, `Path`, `ObjectReturnField::Property(Property::Volume)`, etc.).
- **Multi-Version Union (Wwise 2024 + 2025+):** A single unified API supports both Wwise 2024 and Wwise 2025+ (keeping renamed variants like `ObjectType::ActorMixer` and `ObjectType::PropertyContainer` as aliases sharing `class_id()` `524304`).
- **Raw URI Constants & Escape Hatch:** Every URI is also available as a `WaapiFunction` / `WaapiTopic` enum variant and as a `&str` constant under `wwise_waapi::waapi_function_api::ak` and `wwise_waapi::waapi_topics_api::ak`.

For the complete endpoint & topic tables and JSON Schema mapping rules, see:
- **[Typed API Design & Endpoint Reference (`docs/API_REFERENCE.md`)](docs/API_REFERENCE.md)**
- **[Interactive Rustdoc Documentation](https://tronhjem.github.io/rust-waapi-client/wwise_waapi/)**

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
