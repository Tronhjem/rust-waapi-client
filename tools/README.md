# WAAPI Tools

Python scripts for working with the Wwise Authoring API.

## Scripts

### GetWaapiFunctions.py

Connects to a running Wwise instance and retrieves the list of all available WAAPI functions, saving them to `functions.txt`.

**Requirements:**
- Wwise must be running
- Wwise Authoring API must be enabled
- Python package `waapi-client` must be installed

**Usage:**
```bash
python get_waapi_finctions.py
```

**Output:**
Creates `waapi_functions.txt` and `waapi_topics.tdxt` with a JSON list of all WAAPI function endpoints.

---

### generate_api_rs.py

Generates the Rust `src/api.rs` file from `waapi_functions.txt`, creating a properly structured module hierarchy with all WAAPI endpoints as constants.

**Requirements:**
- Python 3.6+
- `waapi_functions.txt` must exist (run `get_waapi_functions.py` first)

**Usage:**
```bash
python generate_api_rs.py
```

**Output:**
Overwrites `../src/api.rs` with:
- Properly nested module structure matching WAAPI hierarchy
- All function endpoints as public constants
- Rust keyword escaping (e.g., `move` becomes `r#move`)
- Proper line length formatting
- Alphabetically sorted functions and modules

**Example:**

Input from `functions.txt`:
```json
{
  "functions": [
    "ak.wwise.core.object.get",
    "ak.wwise.core.object.set",
    "ak.wwise.core.getInfo"
  ]
}
```

Generated in `api.rs`:
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

To update the API endpoints when a new version of Wwise is released:

1. Start Wwise and enable WAAPI
2. Run `python GetWaapiFunctions.py` to update `functions.txt`
3. Run `python generate_api_rs.py` to regenerate `src/api.rs`
4. Build and test: `cargo build` and `cargo test`
