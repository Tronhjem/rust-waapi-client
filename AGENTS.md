# AGENTS.md

Rust client library for the Wwise Authoring API (WAAPI) over WAMP/WebSocket.

## Quick commands

```bash
cargo build
cargo test
cargo test <test_name>
cargo run --example get_info     # requires Wwise running with WAAPI on 127.0.0.1:8080
RUST_LOG=info cargo run --example get_info
```

## Layout

- `src/wamp/` — Private, minimal WAMP v2 client (JSON/WebSocket only). Not a submodule.
- `src/waapi_client.rs` — `WaapiClient` wraps `WampClient`; default URL is `ws://127.0.0.1:8080/waapi`.
- `src/types.rs` — Re-exports of `serde_json::Value`/`Map` for convenience.
- `src/waapi_function_api.rs` / `src/waapi_topics_api.rs` — Auto-generated const strings (`ak::...`) and typed URI enums (`WaapiFunction` / `WaapiTopic`) for all WAAPI RPC endpoints and subscription topics.
- `src/api/` — Hand-written helper types (`primitives.rs`) and generated typed bindings (`generated/`).

## Important patterns

- `WaapiClient::new` is async.
- Always call `client.shutdown().await` before dropping; `Drop` prints a warning if you do not.
- `client.call(WaapiFunction, Option<serde_json::Value>, Option<serde_json::Value>) -> WaapiResult<serde_json::Value>` and `client.subscribe(WaapiTopic, Option<serde_json::Value>)` take typed URI enums (whose `.as_str()` returns the `ak::...` `&str` constants). Use `call_raw` / `subscribe_raw` if a raw `&str` URI is needed.
- The typed API delegates to `WaapiClient::call_typed` / `call_typed_void` / `subscribe_typed` using `WaapiFunction` / `WaapiTopic`, which serialize typed args/options and deserialize the response. This keeps `WaapiClient` as the single entry point for calls and avoids duplicating serialization logic in generated code.
- `pub use serde_json;` is re-exported from the crate root, so `waapi_client::serde_json::json!` works without an explicit `serde_json` dependency.
- `waapi_client::json!` is also exported directly for the same purpose (it delegates to `serde_json::json!`).
- The generated module hierarchy mirrors WAAPI URI dot-notation, e.g. `ak.wwise.core.object.get` → `ak::wwise::core::object::get`.

## Regenerating the WAAPI bindings

Requires Wwise running with WAAPI enabled and the Python `waapi-client` package installed.

```bash
python tools/get_waapi_definitions.py  # writes tools/waapi_schemas/<version>/
python tools/generate_waapi_uri.py    # overwrites src/waapi_function_api.rs and src/waapi_topics_api.rs
python tools/generate_typed_api.py    # overwrites src/api/generated/
cargo build && cargo test
```

Both `generate_waapi_uri.py` and `generate_typed_api.py` read the versioned schemas in `tools/waapi_schemas/<version>/` offline (no running Wwise instance needed once schemas are fetched).
