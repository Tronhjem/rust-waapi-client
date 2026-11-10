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

A subset of common endpoints has generated type-safe wrappers on `WaapiClient`. These methods delegate to internal `call_typed` / `call_typed_void` helpers that serialize typed argument/option structs and deserialize the response for you:

```rust
use waapi_client::api::generated::ak::wwise::core::WwiseCoreGetInfoResponse;

let info: WwiseCoreGetInfoResponse = client.get_info().await?;
println!("Wwise version: {:?}", info.version.display_name);
```

```rust
use waapi_client::api::generated::ak::wwise::core::object::{
    WwiseCoreObjectGetArgs, WwiseCoreObjectGetOptions,
};
use waapi_client::api::generated::primitives::ObjectQuery;

let args = WwiseCoreObjectGetArgs::new(ObjectQuery::Path(vec![
    r"\Actor-Mixer Hierarchy\Default Work Unit\MySound".to_string(),
]));

let options = WwiseCoreObjectGetOptions::new()
    .return_fields(vec!["id".into(), "name".into(), "type".into()]);

let response = client.object_get(args, options).await?;
```

Typed methods are generated for a small set of endpoints (see `src/api/generated/client_impl.rs`). For all other WAAPI calls, use the escape-hatch `client.call` API.

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
    WwiseCoreObjectSetArgs, WwiseCoreObjectSetItem, WwiseCoreObjectSetOptions,
};
use waapi_client::api::generated::primitives::ObjectReturnField;
use waapi_client::api::{ObjectRef, PropertyValue};

let mut client = WaapiClient::new(None).await?;

let args = WwiseCoreObjectSetArgs::new(vec![
    WwiseCoreObjectSetItem::new(ObjectRef::path(
        r"\Actor-Mixer Hierarchy\Default Work Unit\MySound",
    ))
    .name("MyRenamedSound")
    .property("Volume", PropertyValue::Number(-6.0)),
]);

let options = WwiseCoreObjectSetOptions::new()
    .return_fields(vec![ObjectReturnField::Id, ObjectReturnField::Name]);

let response = client.object_set(args, options).await?;
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
cargo run --example get_info_typed
cargo run --example typed_api
cargo run --example subscribe
cargo run --example object_set
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
