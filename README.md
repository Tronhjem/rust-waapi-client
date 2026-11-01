# waapi-client

A Rust client library for the Wwise Authoring API (WAAPI), providing a clean, type-safe interface for interacting with Audiokinetic Wwise.
More information about WAAPI can be found here: https://www.audiokinetic.com/en/public-library/2025.1.4_9062/?source=SDK&id=waapi.html

This crate ships with a small, private WAMP v2 client that speaks JSON over WebSocket. It is no longer dependent on an external `wampire` submodule.

## Basic Usage

### Connecting to WAAPI

```rust
use waapi_client::{WaapiClient, WaapiResult};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    // Connect to WAAPI (default: 127.0.0.1:8080)
    let mut client = WaapiClient::new(None).await?;

    // Make calls...

    // Always shutdown when done
    client.shutdown().await?;
    Ok(())
}
```

`WaapiClient::new` accepts `None` for the default URL, a `host:port` string, or a full WebSocket URI such as `ws://127.0.0.1:8080/waapi`.

### Making Simple Calls

The escape-hatch API works directly with `serde_json::Value`. `serde_json` is re-exported from the crate root, so the `json!` macro is available without an extra dependency:

```rust
use waapi_client::waapi_function_api::ak;
use waapi_client::serde_json::json;

let response = client
    .call(ak::wwise::core::getInfo, None, None)
    .await?;
println!("{}", serde_json::to_string_pretty(&response)?);
```

### Building Arguments

Use `serde_json::json!` to build WAAPI kwargs:

```rust
use waapi_client::serde_json::json;

let args = json!({
    "from": {
        "id": ["{GUID-HERE}"]
    }
});

let options = json!({
    "return": ["id", "name", "type"]
});

let response = client
    .call(ak::wwise::core::object::get, Some(args), Some(options))
    .await?;
```

### Typed API

For supported endpoints, use the generated typed methods under `client`:

```rust
use waapi_client::api::generated::ak::wwise::core::WwiseCoreGetinfoResponse;

let info: WwiseCoreGetinfoResponse = client.get_info(()).await?;
println!("Wwise version: {:?}", info.version);

use waapi_client::api::generated::ak::wwise::core::object::{
    WwiseCoreObjectGetArgs, WwiseCoreObjectGetOptions,
};

let args = WwiseCoreObjectGetArgs::new(r"\Actor-Mixer Hierarchy\Default Work Unit\MySound".into())
    .transform(vec![]);
let options = WwiseCoreObjectGetOptions::new()
    .return_fields(vec!["id".into(), "name".into(), "type".into()]);
let response = client.object_get(args, options).await?;
```

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
use waapi_client::{WaapiClient, WaapiResult};
use waapi_client::waapi_function_api::ak;
use waapi_client::serde_json::json;

#[tokio::main]
async fn main() -> WaapiResult<()> {
    let mut client = WaapiClient::new(Some("127.0.0.1:8080")).await?;

    let args = json!({
        "from": { "id": ["{GUID}"] }
    });
    let options = json!({
        "return": ["id", "name", "type"]
    });

    let response = client
        .call(ak::wwise::core::object::get, Some(args), Some(options))
        .await?;

    println!("{}", serde_json::to_string_pretty(&response)?);

    client.shutdown().await?;
    Ok(())
}
```

### Subscriptions

Subscribe to WAAPI topics through the low-level WAMP client:

```rust
use waapi_client::waapi_topics_api::ak;
use waapi_client::serde_json::json;

let options = json!({ "return": ["id", "name"] });
let (id, mut events) = client
    .subscribe(ak::wwise::core::object::nameChanged, Some(options))
    .await?;
while let Some(event) = events.recv().await {
    println!("{}", serde_json::to_string_pretty(&event.kwargs)?);
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
```

See the [`examples/`](examples/) directory for more usage examples.

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
