from json.encoder import encode_basestring_ascii
from waapi import WaapiClient, CannotConnectToWaapiException
import json

try:
    with WaapiClient() as client:
        result = client.call("ak.wwise.waapi.getFunctions")

        json_string = json.dumps(result, ensure_ascii=False, indent=2)
        with open("waapi_functions.txt", "w", encoding="utf-8") as f:
            f.write(json_string)

        result = client.call("ak.wwise.waapi.getTopics")

        json_string = json.dumps(result, ensure_ascii=False, indent=2)
        with open("waapi_topics.txt", "w", encoding="utf-8") as f:
            f.write(json_string)



except CannotConnectToWaapiException: print("Could not connect to Waapi: Is Wwise running and Wwise Authoring API enabled?")
