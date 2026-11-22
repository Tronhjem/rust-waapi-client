# WAAPI Tools

Python scripts for working with the Wwise Authoring API.

## Scripts

### get_waapi_definitions.py

Connects to a running Wwise instance, detects the Wwise version, and downloads:

- A JSON schema for every WAAPI function and topic endpoint.
- The shared JSON definition files used by those schemas.

**Requirements:**
- Wwise must be running
- Wwise Authoring API must be enabled
- Python package `waapi-client` must be installed

**Usage:**
```bash
python tools/get_waapi_definitions.py
```

**Output:**
```
tools/waapi_schemas/<version>/
  ak.wwise.core.object.get.json
  ak.wwise.core.object.set.json
  ...
  Definitions/
    WObjects.xml
    common_definitions.json
    waapi_definitions.json
    waapi_ui_definitions.json
    wafm_definitions.json
```

The `<version>` directory is derived from `ak.wwise.core.getInfo` (e.g. `2024.1.7`).

---

### generate_waapi_uri.py

Generates the Rust `src/waapi_function_api.rs` and `src/waapi_topics_api.rs` files from the union of all versioned schemas in `tools/waapi_schemas/<version>/`, creating both a structured `ak::...` module hierarchy of `&str` constants and the `WaapiFunction` / `WaapiTopic` enums backed by those constants.

**Requirements:**
- Python 3.6+
- `tools/waapi_schemas/<version>/` must exist (run `get_waapi_definitions.py` first)

**Usage:**
```bash
python tools/generate_waapi_uri.py
```

**Output:**
Overwrites `src/waapi_function_api.rs` and `src/waapi_topics_api.rs` with:
- Proper nested module structure matching WAAPI hierarchy (`ak::...` `&str` constants)
- `WaapiFunction` and `WaapiTopic` enums whose `as_str()` returns the `ak::...` `&str` constants
- Rust keyword suffixing (e.g., `move` becomes `move_`)
- Alphabetically sorted functions and modules

**Example:**

Generated in `waapi_function_api.rs`:
```rust
pub mod ak {
    pub mod wwise {
        pub mod core {
            pub const getInfo: &str = "ak.wwise.core.getInfo";

            pub mod object {
                pub const get: &str = "ak.wwise.core.object.get";
                pub const set: &str = "ak.wwise.core.object.set";
            }
        }
    }
}
```

---

### generate_typed_api.py (and `tools/generator/`, `tools/overrides.json`)

Generates the typed Rust bindings under `src/api/generated/` by discovering all function and topic schemas across the union of `tools/waapi_schemas/<version>/`.

- `tools/overrides.json` holds externalized schema overrides (`ctor_order`, `result_field_names`, `result_wrapped_object_return`, `response_required_overrides`, `field_name_overrides`, `prop_type_overrides`, `ref_types`, `excluded_endpoints`, `excluded_topics`).
- `tools/generator/` contains the modular generator pipeline (`config.py`, `schema_loader.py`, `type_mapper.py`, `rust_emitter.py`, `client_emitter.py`).

**Requirements:**
- Python 3.6+
- At least one `tools/waapi_schemas/<version>/` directory with schemas and `Definitions/` must exist (run `get_waapi_definitions.py` first)

**Usage:**
```bash
python tools/generate_typed_api.py
```

**Output:**
Overwrites `src/api/generated/` with typed argument, option, response, and topic payload structs for all discovered endpoints (currently 180 functions and 35 topics), the `ObjectType` (`object_types.rs`) and `Property` (`properties.rs`) enums (generated from the union of all `Definitions/WObjects.xml` files), shared primitives (`primitives.rs`), and chained namespace clients (`clients.rs`), and prints a summary of any schema fields that fell back to `serde_json::Value`.

## Workflow

To update the API endpoints and schemas when a new version of Wwise is released:

1. Start Wwise and enable WAAPI
2. Run `python tools/get_waapi_definitions.py` to download the new version's schemas and definitions into `tools/waapi_schemas/<version>/`
3. Run `python tools/generate_waapi_uri.py` to regenerate `src/waapi_function_api.rs` and `src/waapi_topics_api.rs` across the union of all version directories
4. Run `python tools/generate_typed_api.py` to regenerate `src/api/generated/` across the union of all version directories
5. Build and test: `cargo build && cargo test`
