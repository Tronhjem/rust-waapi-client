use serde_json::{json, Value};
use waapi_client::api::generated::ak::soundengine::{
    SoundenginePosteventArgs, SoundengineSetrtpcvalueArgs,
};
use waapi_client::api::generated::ak::wwise::core::WwiseCoreGetinfoResponse;
use waapi_client::api::generated::ak::wwise::core::audio::{
    WwiseCoreAudioImportArgs, WwiseCoreAudioImportOptions,
};
use waapi_client::api::generated::ak::wwise::core::object::{
    WwiseCoreObjectCreateArgs, WwiseCoreObjectDeleteArgs, WwiseCoreObjectGetArgs,
    WwiseCoreObjectGetOptions, WwiseCoreObjectGetResponse,
};
use waapi_client::api::generated::ak::wwise::core::project::WwiseCoreProjectSaveArgs;
use waapi_client::api::generated::ak::wwise::ui::{
    WwiseUiGetselectedobjectsOptions, WwiseUiGetselectedobjectsResponse,
};
use waapi_client::api::generated::primitives::{
    GameObjectRef, ImportDefaults, ImportItem, ImportOperation, ObjectQuery, OnNameConflict,
    PlatformRef, RtpcRef, Transform,
};
use waapi_client::api::primitives::ObjectRef;

#[test]
fn object_get_args_serializes_to_expected_shape() {
    let mut transform = Transform::default();
    transform.extra.insert(
        "select".to_string(),
        Value::Array(vec![Value::String("children".to_string())]),
    );
    let args = WwiseCoreObjectGetArgs::new(ObjectQuery::from(
        r"\Actor-Mixer Hierarchy\Default Work Unit",
    ))
    .transform(vec![transform]);

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(
        value["from"]["path"],
        json!([r"\Actor-Mixer Hierarchy\Default Work Unit"])
    );
    assert!(value["transform"].is_array());
}

#[test]
fn object_get_options_serializes_return_keyword() {
    let options = WwiseCoreObjectGetOptions::new()
        .return_fields(vec!["id".to_string(), "name".to_string()])
        .platform(PlatformRef::Name("Windows".to_string()));

    let value = serde_json::to_value(&options).unwrap();
    assert_eq!(value["return"], json![["id", "name"]]);
    assert_eq!(value["platform"], "Windows");
}

#[test]
fn object_create_args_serializes_required_and_optional_fields() {
    let args = WwiseCoreObjectCreateArgs::new(
        ObjectRef::path(r"\Actor-Mixer Hierarchy"),
        "Folder".to_string(),
        "MyFolder".to_string(),
    )
    .on_name_conflict(OnNameConflict::Rename);

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["parent"], r"\Actor-Mixer Hierarchy");
    assert_eq!(value["type"], "Folder");
    assert_eq!(value["name"], "MyFolder");
    assert_eq!(value["onNameConflict"], "rename");
}

#[test]
fn object_delete_args_serializes_object_ref() {
    let args =
        WwiseCoreObjectDeleteArgs::new(ObjectRef::guid("{00000000-0000-0000-0000-000000000001}"));

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["object"], "{00000000-0000-0000-0000-000000000001}");
}

#[test]
fn soundengine_post_event_args_serializes() {
    let args = SoundenginePosteventArgs::new("Play_Test".into(), GameObjectRef::from(42u64));

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["event"], "Play_Test");
    assert_eq!(value["gameObject"], 42);
}

#[test]
fn soundengine_set_rtpc_value_args_serializes() {
    let args = SoundengineSetrtpcvalueArgs::new("Volume".into(), 0.75, GameObjectRef::from(100u32));

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["rtpc"], "Volume");
    assert_eq!(value["value"], 0.75);
    assert_eq!(value["gameObject"], 100);
}

#[test]
fn audio_import_args_serializes_imports_and_defaults() {
    let mut item = ImportItem::new(r"\Actor-Mixer Hierarchy\Default Work Unit\MySound");
    item.audio_file = Some(r"C:\Audio\MySound.wav".to_string());

    let defaults = ImportDefaults {
        object_type: Some("Sound".to_string()),
        ..Default::default()
    };

    let args = WwiseCoreAudioImportArgs::new(vec![item])
        .import_operation(ImportOperation::CreateNew)
        .default(defaults);

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["importOperation"], "createNew");
    assert_eq!(
        value["imports"][0]["objectPath"],
        r"\Actor-Mixer Hierarchy\Default Work Unit\MySound"
    );
    assert_eq!(value["imports"][0]["audioFile"], r"C:\Audio\MySound.wav");
    assert_eq!(value["default"]["objectType"], "Sound");
}

#[test]
fn audio_import_options_serializes_return_and_platform() {
    let options = WwiseCoreAudioImportOptions::new()
        .return_fields(vec!["id".to_string(), "name".to_string()])
        .platform(PlatformRef::Guid("{GUID}".to_string()));

    let value = serde_json::to_value(&options).unwrap();
    assert_eq!(value["return"], json![["id", "name"]]);
    assert_eq!(value["platform"], "{GUID}");
}

#[test]
fn project_save_args_serializes_empty_args() {
    let args = WwiseCoreProjectSaveArgs::new();
    let value = serde_json::to_value(&args).unwrap();
    assert!(value.as_object().unwrap().is_empty());
}

#[test]
fn ui_get_selected_objects_options_serializes() {
    let options = WwiseUiGetselectedobjectsOptions::new().return_fields(vec!["id".to_string()]);
    let value = serde_json::to_value(&options).unwrap();
    assert_eq!(value["return"], json![["id"]]);
}

#[test]
fn get_info_response_deserializes() {
    let json = json!({
        "displayName": "Wwise",
        "platform": "x64",
        "version": {
            "displayName": "2024.1.7",
            "year": 2024,
            "major": 1,
            "minor": 7,
            "build": 8863,
            "nickname": "",
            "schema": 108
        },
        "sessionId": "{FF59687C-48CF-4385-B1C5-CE84B0A63880}",
        "apiVersion": 1,
        "branch": "v2024.1/wwise_main",
        "copyright": "Copyright 2024",
        "configuration": "release",
        "isCommandLine": false,
        "processId": 12345,
        "processPath": "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863/Authoring/x64/Release/bin/Wwise.exe",
        "directories": {
            "user": "%APPDATA%/Audiokinetic/Wwise/",
            "install": "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863",
            "authoring": "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863/Authoring",
            "bin": "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863/Authoring/x64/Release/bin",
            "help": "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863/Authoring/Help"
        }
    });

    let resp: WwiseCoreGetinfoResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.display_name, "Wwise");
    assert_eq!(resp.platform, "x64");
    assert_eq!(resp.version.display_name, Some("2024.1.7".to_string()));
    assert_eq!(resp.version.schema, Some(108));
    assert_eq!(
        resp.session_id,
        "{FF59687C-48CF-4385-B1C5-CE84B0A63880}".to_string()
    );
    assert_eq!(resp.api_version, 1.0);
    assert_eq!(resp.configuration, "release");
    assert_eq!(resp.process_id, 12345);
    assert_eq!(
        resp.directories.install,
        Some("C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863".to_string())
    );
}

#[test]
fn object_get_response_deserializes() {
    let json = json!({
        "objects": [
            {
                "id": "{GUID-1}",
                "name": "MySound",
                "type": "Sound",
                "shortId": 12345
            }
        ],
        "return": ["id", "name", "type", "shortId"]
    });

    let resp: WwiseCoreObjectGetResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.objects.len(), 1);
    let obj = &resp.objects[0];
    assert_eq!(obj.id, Some("{GUID-1}".to_string()));
    assert_eq!(obj.name, Some("MySound".to_string()));
    assert_eq!(obj.object_type, Some("Sound".to_string()));
    assert_eq!(obj.short_id, Some(12345));
}

#[test]
fn ui_get_selected_objects_response_deserializes() {
    let json = json!({
        "objects": [
            {
                "id": "{GUID-2}",
                "name": "MyEvent",
                "type": "Event"
            }
        ],
        "return": ["id", "name", "type"]
    });

    let resp: WwiseUiGetselectedobjectsResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.objects.len(), 1);
    assert_eq!(resp.objects[0].name, Some("MyEvent".to_string()));
}

#[test]
fn rtpc_ref_serializes_name_guid_and_short_id() {
    assert_eq!(
        serde_json::to_value(RtpcRef::from("Volume")).unwrap(),
        "Volume"
    );
    assert_eq!(
        serde_json::to_value(RtpcRef::Guid("{GUID}".to_string())).unwrap(),
        "{GUID}"
    );
    assert_eq!(serde_json::to_value(RtpcRef::from(123u32)).unwrap(), 123);
}

#[test]
fn object_ref_deserialization_roundtrip() {
    let name: waapi_client::api::primitives::ObjectRef =
        serde_json::from_value(json!("Play_Test")).unwrap();
    assert_eq!(serde_json::to_value(name).unwrap(), "Play_Test");

    let guid: waapi_client::api::primitives::ObjectRef =
        serde_json::from_value(json!("{00000000-0000-0000-0000-000000000001}")).unwrap();
    assert_eq!(
        serde_json::to_value(guid).unwrap(),
        "{00000000-0000-0000-0000-000000000001}"
    );

    let path: waapi_client::api::primitives::ObjectRef =
        serde_json::from_value(json!(r"\Events\Default Work Unit\Play_Test")).unwrap();
    assert_eq!(
        serde_json::to_value(path).unwrap(),
        r"\Events\Default Work Unit\Play_Test"
    );
}
