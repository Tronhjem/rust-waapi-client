# WAAPI Tools

Python scripts for working with the Wwise Authoring API.

## Scripts

### get_waapi_functions.py

Connects to a running Wwise instance and retrieves the list of all available WAAPI functions and topics, saving them to `waapi_functions.txt` and `waapi_topics.txt`.

**Requirements:**
- Wwise must be running
- Wwise Authoring API must be enabled
- Python package `waapi-client` must be installed

**Usage:**
```bash
python get_waapi_functions.py
```

**Output:**
Creates `waapi_functions.txt` and `waapi_topics.txt` with a JSON list of all WAAPI function and topic endpoints.

---

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
python get_waapi_definitions.py
```

**Output:**
```
tools/waapi_schemas/<version>/
  ak.wwise.core.object.get.json
  ak.wwise.core.object.set.json
  ...
  Definitions/
    common_definitions.json
    waapi_definitions.json
    waapi_ui_definitions.json
```

The `<version>` directory is derived from `ak.wwise.core.getInfo` (e.g. `2024.1.7`).

---

### get_waapi_schemas.py

(Deprecated in favor of `get_waapi_definitions.py`.) Fetches per-endpoint schemas into a flat `waapi_schemas/` directory without versioning or shared definitions.

---

### generate_api_rs.py

Generates the Rust `src/waapi_function_api.rs` and `src/waapi_topics_api.rs` files from `waapi_functions.txt` and `waapi_topics.txt`, creating a properly structured module hierarchy with all WAAPI endpoints as constants.

**Requirements:**
- Python 3.6+
- `waapi_functions.txt` and `waapi_topics.txt` must exist (run `get_waapi_functions.py` first)

**Usage:**
```bash
python generate_api_rs.py
```

**Output:**
Overwrites `../src/waapi_function_api.rs` and `../src/waapi_topics_api.rs` with:
- Proper nested module structure matching WAAPI hierarchy
- All function/topic endpoints as public constants
- Rust keyword escaping (e.g., `move` becomes `r#move`)
- Alphabetically sorted functions and modules

**Example:**

Input from `waapi_functions.txt`:
```json
{
  "functions": [
    "ak.wwise.core.object.get",
    "ak.wwise.core.object.set",
    "ak.wwise.core.getInfo"
  ]
}
```

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

## Workflow

To update the API endpoints and schemas when a new version of Wwise is released:

1. Start Wwise and enable WAAPI
2. Run `python get_waapi_functions.py` to update `waapi_functions.txt` and `waapi_topics.txt`
3. Run `python get_waapi_definitions.py` to download versioned schemas and definitions
4. Run `python generate_api_rs.py` to regenerate `src/waapi_function_api.rs` and `src/waapi_topics_api.rs`
5. Build and test: `cargo build` and `cargo test`
