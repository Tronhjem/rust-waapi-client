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
- `src/api/` — Hand-written helper types (`primitives.rs` — canonical home for `ChildObject`, `ListMode`, `ObjectRef`, `OnNameConflict`, `PlatformRef`, `Position`, `PropertyValue`, `Vector3`) and generated typed bindings (`generated/`, including `ObjectType` and `Property` from `WObjects.xml`, shared `primitives.rs`, endpoint modules under `ak/`, and chained namespace clients in `clients.rs`).
- `tools/generator/` & `tools/overrides.json` — Modular schema-driven typed API generator (`config.py`, `schema_loader.py`, `type_mapper.py`, `rust_emitter.py`, `client_emitter.py`) invoked by `tools/generate_typed_api.py`, plus externalized schema overrides in `tools/overrides.json`.
- `tests/wwise_sequences.rs` & `tests/sequences/` — End-to-end WAAPI command sequence tests against a live Wwise instance. Every sequence test must start with `let _guard = sequence_lock().await;` (from `tests/sequences/mod.rs`): Wwise has a single global undo stack and transport list, so live tests are serialized through a shared `tokio::sync::Mutex`.

## Important patterns

- `WaapiClient::connect()` (default `ws://127.0.0.1:8080/waapi`), `WaapiClient::connect_to(url)`, and `WaapiClient::new(Option<&str>)` are async.
- Always call `client.shutdown().await` before dropping; `Drop` prints a warning if you do not.
- `wwise_waapi::prelude::*` re-exports common client, query, object, property, and SoundEngine types for single-line imports.
- `client.call(WaapiFunction, Option<serde_json::Value>, Option<serde_json::Value>) -> WaapiResult<serde_json::Value>` and `client.subscribe(WaapiTopic, Option<serde_json::Value>)` take typed URI enums (whose `.as_str()` returns the `ak::...` `&str` constants). Use `call_raw` / `subscribe_raw` if a raw `&str` URI is needed.
- The typed API exposes a chained namespace hierarchy on `WaapiClient` (`client.ak().wwise()...` plus `client.wwise()`, `client.core()`, `client.object()`, `client.ui()`, and `client.soundengine()` shortcuts) that mirrors WAAPI URI dot-notation (e.g. `ak.wwise.core.object.get` → `client.object().get(...)` or `client.wwise().core().object().get(...)` with types in `api::ak::wwise::core::object`). Each module exports both canonical (`CoreObjectGetArgs`) and short module-scoped (`GetArgs`, `GetOptions`, `GetResponse`) type names, `Args::new(...)` accepts `impl Into<T>` for string/ref types, and client methods accept `impl Into<Args>` (including direct paths/GUIDs for 1-field `Args`, a single `SetItem` for `set`, and tuples for 2/3-field `Args`). Each method delegates to `WaapiClient::call_typed` / `call_typed_void` / `subscribe_typed` using `WaapiFunction` / `WaapiTopic`, keeping `WaapiClient` as the single entry point for calls.
- Topics have generated payload structs (`<Base>Payload`) and `subscribe_*` chained methods returning `WaapiResult<(u64, UnboundedReceiver<Payload>)>`; these delegate to `WaapiClient::subscribe_payload<Options, Payload>`, which deserializes each `event.kwargs` into the payload (skipping malformed events with a warning).
- `pub use serde_json;` is re-exported from the crate root, so `wwise_waapi::serde_json::json!` works without an explicit `serde_json` dependency.
- `wwise_waapi::json!` is also exported directly for the same purpose (it delegates to `serde_json::json!`).

## Regenerating the WAAPI bindings

Requires Wwise running with WAAPI enabled and the Python `waapi-client` package installed.

```bash
python tools/get_waapi_definitions.py  # writes tools/waapi_schemas/<version>/
python tools/generate_waapi_uri.py    # overwrites src/waapi_function_api.rs and src/waapi_topics_api.rs
python tools/generate_typed_api.py    # overwrites src/api/generated/
cargo build && cargo test
```

Both `generate_waapi_uri.py` and `generate_typed_api.py` read the versioned schemas in `tools/waapi_schemas/<version>/` offline (no running Wwise instance needed once schemas are fetched). They generate a **union** across every version directory present (e.g. `2024.1.7/` + `2025.1.12/`), producing a single API that serves all versions: renamed object types are kept as aliases (`ObjectType::ActorMixer` and `ObjectType::PropertyContainer` share `class_id()` `524304`), 2024-only return fields are retained, and the latest version's schema is canonical for field requiredness. To add a new version, fetch its schemas into a new `tools/waapi_schemas/<version>/` directory and re-run both generators.
