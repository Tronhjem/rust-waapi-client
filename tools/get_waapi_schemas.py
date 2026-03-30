import os
import json
from waapi import WaapiClient, CannotConnectToWaapiException

OUTPUT_DIR = "waapi_schemas"

try:
    with WaapiClient() as client:
        result = client.call("ak.wwise.waapi.getFunctions")
        functions = result.get("functions", [])

        os.makedirs(OUTPUT_DIR, exist_ok=True)

        for uri in functions:
            print(f"Fetching schema for: {uri}")
            try:
                schema = client.call("ak.wwise.waapi.getSchema", {"uri": uri, "includeExamples": True})
                filename = os.path.join(OUTPUT_DIR, f"{uri}.json")
                with open(filename, "w", encoding="utf-8") as f:
                    json.dump(schema, f, ensure_ascii=False, indent=2)
            except Exception as e:
                print(f"  ERROR fetching schema for {uri}: {e}")

        print(f"\nDone. {len(functions)} schemas saved to '{OUTPUT_DIR}/'")

except CannotConnectToWaapiException:
    print("Could not connect to Waapi: Is Wwise running and Wwise Authoring API enabled?")
