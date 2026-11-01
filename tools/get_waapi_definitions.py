"""Fetch WAAPI function/topic schemas and shared definitions from a running Wwise instance.

Requirements:
    - Wwise must be running with Wwise Authoring API enabled.
    - Python package `waapi-client` must be installed.

Usage:
    python tools/get_waapi_definitions.py

Output:
    tools/waapi_schemas/<version>/<uri>.json      # per-endpoint schemas
    tools/waapi_schemas/<version>/Definitions/    # shared JSON definitions
"""

import json
import os
import re
import shutil
import sys

from waapi import WaapiClient, CannotConnectToWaapiException


SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(SCRIPT_DIR)
OUTPUT_DIR = os.path.join(ROOT_DIR, "tools", "waapi_schemas")

DEFINITION_FILES = [
    "common_definitions.json",
    "waapi_definitions.json",
    "waapi_ui_definitions.json",
]


def normalize_version(info):
    """Return a filesystem-safe version string like '2024.1.7'."""
    version = info.get("version", {})
    name = version.get("displayName") or version.get("name") or "unknown"
    match = re.search(r"(\d+(?:\.\d+){1,3})", name)
    if match:
        parts = match.group(1).split(".")
        return ".".join(parts[:3])
    safe = re.sub(r"[^\w.\-]", "_", name)
    return safe or "unknown"


def save_json(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)


def fetch_schemas(client, uris, version_dir):
    count = 0
    for uri in uris:
        try:
            schema = client.call(
                "ak.wwise.waapi.getSchema", {"uri": uri, "includeExamples": True}
            )
        except Exception as e:
            print(f"  ERROR fetching schema for {uri}: {e}")
            continue

        filename = os.path.join(version_dir, f"{uri}.json")
        save_json(filename, schema)
        count += 1
    return count


def copy_definitions(authoring_dir, version_dir):
    source_dir = os.path.join(authoring_dir, "Data", "Schemas", "Definitions")
    target_dir = os.path.join(version_dir, "Definitions")
    os.makedirs(target_dir, exist_ok=True)

    copied = 0
    for filename in DEFINITION_FILES:
        source = os.path.join(source_dir, filename)
        target = os.path.join(target_dir, filename)
        if os.path.isfile(source):
            shutil.copy2(source, target)
            copied += 1
        else:
            print(f"  WARNING: definition file not found: {source}")
    return copied


def main():
    try:
        with WaapiClient() as client:
            info = client.call("ak.wwise.core.getInfo")
            version = normalize_version(info)
            version_dir = os.path.join(OUTPUT_DIR, version)

            functions = client.call("ak.wwise.waapi.getFunctions").get("functions", [])
            topics = client.call("ak.wwise.waapi.getTopics").get("topics", [])
            all_uris = sorted(set(functions) | set(topics))

            print(f"Detected Wwise version: {version}")
            print(f"Fetching {len(functions)} function schemas and {len(topics)} topic schemas...")
            schema_count = fetch_schemas(client, all_uris, version_dir)

            print("Copying shared definitions...")
            authoring_dir = info.get("directories", {}).get("authoring")
            if not authoring_dir:
                print("  WARNING: directories.authoring not returned by getInfo; skipping definitions copy")
                definition_count = 0
            else:
                definition_count = copy_definitions(authoring_dir, version_dir)

            print(f"\nDone. {schema_count} schemas and {definition_count} definition files saved to '{version_dir}/'")
            return 0

    except CannotConnectToWaapiException:
        print("Could not connect to WAAPI: Is Wwise running and Wwise Authoring API enabled?")
        return 1
    except Exception as e:
        print(f"Fatal error: {e}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
