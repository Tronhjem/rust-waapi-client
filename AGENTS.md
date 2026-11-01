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
- `src/waapi_function_api.rs` / `src/waapi_topics_api.rs` — Auto-generated const strings for all WAAPI RPC endpoints and subscription topics.
- `src/api/` — Hand-written helper types (`object_set.rs`, `primitives.rs`) and generated typed bindings (`generated/`).

## Important patterns

- `WaapiClient::new` is async.
- Always call `client.shutdown().await` before dropping; `Drop` prints a warning if you do not.
- The escape-hatch call is `client.call(uri, Option<serde_json::Value>, Option<serde_json::Value>) -> WaapiResult<serde_json::Value>`.
- The typed API serializes structs directly to `serde_json::Value` and deserializes the response; there is no intermediate `WaapiValue` layer.
- `pub use serde_json;` is re-exported from the crate root, so `waapi_client::serde_json::json!` works without an explicit `serde_json` dependency.
- The generated module hierarchy mirrors WAAPI URI dot-notation, e.g. `ak.wwise.core.object.get` → `ak::wwise::core::object::get`.

## Regenerating the WAAPI bindings

Requires Wwise running with WAAPI enabled and the Python `waapi-client` package installed.

```bash
python tools/get_waapi_functions.py   # writes tools/waapi_functions.txt, tools/waapi_topics.txt
python tools/get_waapi_schemas.py     # writes tools/waapi_schemas/<uri>.json (reference only)
python tools/generate_api_rs.py       # overwrites src/waapi_function_api.rs and src/waapi_topics_api.rs
python tools/generate_typed_api.py    # overwrites src/api/generated/
cargo build && cargo test
```

`generate_api_rs.py` only reads `waapi_functions.txt` and `waapi_topics.txt`; the JSON schemas are for human reference.
