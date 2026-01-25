# waapi-client

A Rust client library for the Wwise Authoring API (WAAPI), providing a clean, type-safe interface for interacting with Audiokinetic Wwise.
More information about WAAPI can be found here: https://www.audiokinetic.com/en/public-library/2025.1.4_9062/?source=SDK&id=waapi.html

For now it's using a forked version of wampire for working with wamp. 
https://github.com/Tronhjem/wamp-for-rust

## Basic Usage

### Connecting to WAAPI

```rust
use waapi_client::{WaapiClient, WaapiResult};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    // Connect to WAAPI (default: 127.0.0.1:8080)
    let mut client = WaapiClient::new(None)?;
    
    // Make calls...
    
    // Always shutdown when done
    client.shutdown().await?;
    Ok(())
}
```

### Making Simple Calls

```rust
use waapi_client::api::ak;

// Call without arguments
let response = client.call(ak::wwise::core::GET_INFO, None, None).await?;
response.print();
```

### Building Arguments

Use `WaapiArgs` to construct call arguments:

```rust
use waapi_client::{WaapiArgs, WaapiValue};
use std::collections::HashMap;

// Build a "from" query by ID
let mut from_dict = HashMap::new();
from_dict.insert(
    "id".to_string(),
    WaapiValue::List(vec![
        WaapiValue::String("{GUID-HERE}".to_string())
    ])
);

let args = WaapiArgs::new()
    .insert("from", WaapiValue::Dict(from_dict));
```

### Specifying Return Fields

Use `WaapiOptions` with type-safe `ReturnType` enum:

```rust
use waapi_client::{WaapiOptions, ReturnType};

let options = WaapiOptions::with_return(&[
    ReturnType::Id,
    ReturnType::Name,
    ReturnType::Type,
    ReturnType::Path,
]);
```

### Complete Example

```rust
use std::collections::HashMap;
use waapi_client::{
    WaapiClient, WaapiArgs, WaapiOptions, WaapiValue, ReturnType,
    api::ak,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = WaapiClient::new(Some("127.0.0.1:8080"))?;

    // Build arguments
    let mut from_dict = HashMap::new();
    from_dict.insert(
        "id".to_string(),
        WaapiValue::List(vec![
            WaapiValue::String("{GUID}".to_string())
        ])
    );
    let args = WaapiArgs::new()
        .insert("from", WaapiValue::Dict(from_dict));

    // Specify return fields
    let options = WaapiOptions::with_return(&[
        ReturnType::Id,
        ReturnType::Name,
        ReturnType::Type,
    ]);

    // Make the call
    let response = client
        .call(ak::wwise::core::object::GET, Some(args), Some(options))
        .await?;

    // Extract values (type-safe)
    if let Some(id) = response.get_return_value(ReturnType::Id) {
        println!("Object ID: {:?}", id);
    }

    client.shutdown().await?;
    Ok(())
}
```

## Extracting Response Values

The `WaapiResponse` provides several helper methods:

```rust
// Get raw kwargs value by key
let value = response.get("return");

// Get the first object from the return/objects list
if let Some(obj) = response.get_return_object() {
    // obj is a HashMap<String, WaapiValue>
    for (key, value) in obj {
        println!("{}: {:?}", key, value);
    }
}

// Get all return objects as a Vec
let objects = response.get_return_objects();
for obj in objects {
    println!("Object: {:?}", obj);
}

// Get a specific field as string (for display)
if let Some(name) = response.get_return_string("name") {
    println!("Name: {}", name);
}

// Get a field value (type-safe, cloned for reuse in other calls)
if let Some(id_value) = response.get_return_value(ReturnType::Id) {
    // Reuse id_value in another call
    let mut from_dict = HashMap::new();
    from_dict.insert("id".to_string(), WaapiValue::List(vec![id_value]));
    let args = WaapiArgs::new().insert("from", WaapiValue::Dict(from_dict));
}
```

## API Endpoints

Predefined API endpoints are available in the `api::ak` module:

```rust
use waapi_client::api::ak;

// Core API
ak::wwise::core::GET_INFO
ak::wwise::core::object::GET

// UI API
ak::wwise::ui::GET_SELECTED_OBJECTS
```

More endpoints will be added over time, or you can use string literals for any WAAPI endpoint.

## Return Types

The `ReturnType` enum provides type-safe field names for common WAAPI return fields:

- `Id`, `Name`, `Type`, `Notes`, `Path`
- `Parent`, `Owner`, `Category`
- `ChildrenCount`, `IsPlayable`
- Audio source properties: `PlaybackDuration`, `MaxDurationSource`, etc.
- Workunit properties: `WorkunitType`, `WorkunitIsDirty`, etc.
- And many more...

See [src/types.rs](src/types.rs) for the complete list.

## Examples

Run examples with:

```bash
cargo run --example get_info
```

See the [examples/](examples/) directory for more usage examples.

## Error Handling

The library uses a custom `WaapiError` type with variants:

- `ConnectionFailed(String)` - Failed to connect to WAAPI
- `CallFailed(String)` - WAAPI call returned an error
- `ShutdownNotCalled` - Warning when client is dropped without shutdown

Always call `shutdown()` before dropping the client to ensure clean disconnection.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are more than welcome! 
