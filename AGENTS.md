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

- `src/wamp/` — Git submodule, a forked `wampire` WAMP v2 client. The root crate depends on it via `path = "src/wamp"`. Not a workspace member.
- `src/waapi_client.rs` — `WaapiClient` wraps `wampire::Client`; default URL is `ws://127.0.0.1:8080/waapi`.
- `src/types.rs` — `WaapiValue`, `WaapiArgs`, `WaapiOptions`, `ReturnType`.
- `src/waapi_function_api.rs` / `src/waapi_topics_api.rs` — Auto-generated const strings for all WAAPI RPC endpoints and subscription topics.
- `src/api/` — Hand-written helper types for building WAAPI calls (`object_set.rs`, `primitives.rs`).

## Important patterns

- Always call `client.shutdown().await` before dropping; `Drop` prints a warning if you do not.
- `WaapiValue` ↔ `wampire::Value` conversions live in `types.rs`; keep `wampire` types out of the public API.
- The generated module hierarchy mirrors WAAPI URI dot-notation, e.g. `ak.wwise.core.object.get` → `ak::wwise::core::object::get`.

## Regenerating the WAAPI bindings

Requires Wwise running with WAAPI enabled and the Python `waapi-client` package installed.

```bash
python tools/get_waapi_functions.py   # writes tools/waapi_functions.txt, tools/waapi_topics.txt
python tools/get_waapi_schemas.py     # writes tools/waapi_schemas/<uri>.json (reference only)
python tools/generate_api_rs.py       # overwrites src/waapi_function_api.rs and src/waapi_topics_api.rs
cargo build && cargo test
```

`generate_api_rs.py` only reads `waapi_functions.txt` and `waapi_topics.txt`; the JSON schemas are for human reference.
