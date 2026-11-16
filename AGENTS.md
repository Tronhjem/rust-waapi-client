# AGENTS.md

Rust client library for the Wwise Authoring API (WAAPI) over WAMP/WebSocket.

## Quick commands

```bash
cargo build
cargo test                              # runs unit, roundtrip, and live Wwise sequence tests
cargo test --test typed_api_roundtrip   # offline serialization/deserialization tests only
cargo test --test wwise_sequences       # live sequence tests (requires Wwise open with a project on 127.0.0.1:8080)
cargo test <test_name>
cargo run --example get_info            # requires Wwise running with WAAPI on 127.0.0.1:8080
cargo run --example register_and_post_event  # requires Wwise running with WAAPI and event "Play_EventTestSound"
RUST_LOG=info cargo run --example get_info
```

## Layout

- `src/wamp/` — Private, minimal WAMP v2 client (JSON/WebSocket only). Not a submodule.
- `src/waapi_client.rs` — `WaapiClient` wraps `WampClient`; default URL is `ws://127.0.0.1:8080/waapi`.
- `src/types.rs` — Re-exports of `serde_json::Value`/`Map` for convenience.
- `src/waapi_function_api.rs` / `src/waapi_topics_api.rs` — Auto-generated const strings (`ak::...`) and typed URI enums (`WaapiFunction` / `WaapiTopic`) for all WAAPI RPC endpoints and subscription topics.
- `src/api/` — Hand-written helper types (`primitives.rs`) and generated typed bindings (`generated/`, including `ObjectType` and `Property` from `WObjects.xml`, shared `primitives.rs`, endpoint modules under `ak/`, and chained namespace clients in `clients.rs`).
- `tests/wwise_sequences.rs` & `tests/sequences/` — End-to-end WAAPI command sequence tests against a live Wwise instance.

## Important patterns

- `WaapiClient::new` is async.
- Always call `client.shutdown().await` before dropping; `Drop` prints a warning if you do not.
- `client.call(WaapiFunction, Option<serde_json::Value>, Option<serde_json::Value>) -> WaapiResult<serde_json::Value>` and `client.subscribe(WaapiTopic, Option<serde_json::Value>)` take typed URI enums (whose `.as_str()` returns the `ak::...` `&str` constants). Use `call_raw` / `subscribe_raw` if a raw `&str` URI is needed.
- The typed API exposes a chained namespace hierarchy on `WaapiClient` (`client.ak().wwise()...` plus `client.wwise()` and `client.soundengine()` shortcuts) that mirrors WAAPI URI dot-notation (e.g. `ak.wwise.core.object.get` → `client.wwise().core().object().get(...)` with types in `ak::wwise::core::object`). Each method delegates to `WaapiClient::call_typed` / `call_typed_void` / `subscribe_typed` using `WaapiFunction` / `WaapiTopic`, keeping `WaapiClient` as the single entry point for calls.
- `pub use serde_json;` is re-exported from the crate root, so `waapi_client::serde_json::json!` works without an explicit `serde_json` dependency.
- `waapi_client::json!` is also exported directly for the same purpose (it delegates to `serde_json::json!`).

## Regenerating the WAAPI bindings

Requires Wwise running with WAAPI enabled and the Python `waapi-client` package installed.

```bash
python tools/get_waapi_definitions.py  # writes tools/waapi_schemas/<version>/
python tools/generate_waapi_uri.py    # overwrites src/waapi_function_api.rs and src/waapi_topics_api.rs
python tools/generate_typed_api.py    # overwrites src/api/generated/
cargo build && cargo test
```

Both `generate_waapi_uri.py` and `generate_typed_api.py` read the versioned schemas in `tools/waapi_schemas/<version>/` offline (no running Wwise instance needed once schemas are fetched).
