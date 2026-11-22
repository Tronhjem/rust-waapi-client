use serde_json::json;
use wwise_waapi::api::generated::ak::soundengine::{
    SoundenginePostEventArgs, SoundengineRegisterGameObjArgs, SoundengineSetDefaultListenersArgs,
    SoundengineSetPositionArgs, SoundengineSetRtpcValueArgs, SoundengineUnregisterGameObjArgs,
};
use wwise_waapi::api::generated::ak::wwise::core::CoreGetInfoResponse;
use wwise_waapi::api::generated::ak::wwise::core::audio::{
    CoreAudioImportArgs, CoreAudioImportOptions,
};
use wwise_waapi::api::generated::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectDeleteArgs, CoreObjectGetArgs, CoreObjectGetOptions,
    CoreObjectGetResponse, CoreObjectSetArgs, CoreObjectSetItem, CoreObjectSetResponse,
};
use wwise_waapi::api::generated::ak::wwise::core::project::CoreProjectSaveArgs;
use wwise_waapi::api::generated::ak::wwise::ui::{
    UiGetSelectedObjectsOptions, UiGetSelectedObjectsResponse,
};
use wwise_waapi::api::generated::primitives::{
    GameObjectRef, ImportDefaults, ImportItem, ImportOperation, ObjectQuery, ObjectReturnField,
    RtpcRef, Transform, TransformSelect,
};
use wwise_waapi::api::primitives::{
    ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, Position, PropertyValue, Vector3,
};
use wwise_waapi::{ObjectType, Property};

#[test]
fn object_get_args_serializes_to_expected_shape() {
    let args = CoreObjectGetArgs::new(ObjectQuery::from(
        r"\Actor-Mixer Hierarchy\Default Work Unit",
    ))
    .transform(vec![
        Transform::select(TransformSelect::Children),
        Transform::distinct(),
        Transform::range(0, 10),
        Transform::where_name_contains("Footstep"),
        Transform::where_name_matches(".*_01$"),
        Transform::where_type_is_in([ObjectType::Sound, ObjectType::ActorMixer]),
        Transform::where_category_is_in(["Actor-Mixer Hierarchy"]),
    ]);

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(
        value["from"]["path"],
        json!([r"\Actor-Mixer Hierarchy\Default Work Unit"])
    );
    assert_eq!(value["transform"][0]["select"], json!(["children"]));
    assert_eq!(value["transform"][1], json!("distinct"));
    assert_eq!(value["transform"][2]["range"], json!([0, 10]));
    assert_eq!(
        value["transform"][3]["where"],
        json!(["name:contains", "Footstep"])
    );
    assert_eq!(
        value["transform"][4]["where"],
        json!(["name:matches", ".*_01$"])
    );
    assert_eq!(
        value["transform"][5]["where"],
        json!(["type:isIn", ["Sound", "ActorMixer"]])
    );
    assert_eq!(
        value["transform"][6]["where"],
        json!(["category:isIn", ["Actor-Mixer Hierarchy"]])
    );
}

#[test]
fn object_get_options_serializes_return_keyword() {
    let options = CoreObjectGetOptions::new()
        .return_fields(vec!["id".to_string(), "name".to_string()])
        .platform(PlatformRef::Name("Windows".to_string()));

    let value = serde_json::to_value(&options).unwrap();
    assert_eq!(value["return"], json![["id", "name"]]);
    assert_eq!(value["platform"], "Windows");
}

#[test]
fn object_return_options_accept_typed_fields() {
    let options = CoreObjectGetOptions::new().return_fields([
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Property(Property::Volume),
    ]);

    let value = serde_json::to_value(&options).unwrap();
    assert_eq!(value["return"], json!(["id", "name", "@Volume"]));
}

#[test]
fn object_create_args_serializes_required_and_optional_fields() {
    let args = CoreObjectCreateArgs::new(
        ObjectRef::path(r"\Actor-Mixer Hierarchy"),
        ObjectType::Folder,
        "MyFolder".to_string(),
    )
    .on_name_conflict(OnNameConflict::Rename)
    .children(vec![
        ChildObject::new(ObjectType::Sound, "MySound")
            .property(Property::Volume, PropertyValue::Number(-3.0)),
    ]);

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["parent"], r"\Actor-Mixer Hierarchy");
    assert_eq!(value["type"], "Folder");
    assert_eq!(value["name"], "MyFolder");
    assert_eq!(value["onNameConflict"], "rename");
    assert_eq!(value["children"][0]["type"], "Sound");
    assert_eq!(value["children"][0]["name"], "MySound");
    assert_eq!(value["children"][0]["@Volume"], -3.0);
}

#[test]
fn object_delete_args_serializes_object_ref() {
    let args = CoreObjectDeleteArgs::new(ObjectRef::guid("{00000000-0000-0000-0000-000000000001}"));

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["object"], "{00000000-0000-0000-0000-000000000001}");
}

#[test]
fn soundengine_post_event_args_serializes() {
    let args = SoundenginePostEventArgs::new("Play_Test").game_object(42u64);

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["event"], "Play_Test");
    assert_eq!(value["gameObject"], 42);
}

#[test]
fn soundengine_set_rtpc_value_args_serializes() {
    let args = SoundengineSetRtpcValueArgs::new("Volume", 0.75).game_object(100u32);

    let value = serde_json::to_value(&args).unwrap();
    assert_eq!(value["rtpc"], "Volume");
    assert_eq!(value["value"], 0.75);
    assert_eq!(value["gameObject"], 100);
}

#[test]
fn soundengine_register_position_and_listeners_serialize() {
    let reg = serde_json::to_value(SoundengineRegisterGameObjArgs::new(1u64, "Listener")).unwrap();
    assert_eq!(reg, json!({ "gameObject": 1, "name": "Listener" }));

    let listeners = serde_json::to_value(SoundengineSetDefaultListenersArgs::new(vec![
        GameObjectRef::game_object(1),
    ]))
    .unwrap();
    assert_eq!(listeners, json!({ "listeners": [1] }));

    let pos_val = Vector3::forward * -2.0 + Vector3::right * 2.0;
    let set_pos = serde_json::to_value(SoundengineSetPositionArgs::new(
        GameObjectRef::game_object(2),
        Position::new(pos_val),
    ))
    .unwrap();
    assert_eq!(
        set_pos,
        json!({
            "gameObject": 2,
            "position": {
                "orientationFront": { "x": 0.0, "y": 0.0, "z": 1.0 },
                "orientationTop": { "x": 0.0, "y": 1.0, "z": 0.0 },
                "position": { "x": 2.0, "y": 0.0, "z": -2.0 }
            }
        })
    );

    let unreg = serde_json::to_value(SoundengineUnregisterGameObjArgs::new(
        GameObjectRef::game_object(2),
    ))
    .unwrap();
    assert_eq!(unreg, json!({ "gameObject": 2 }));
}

#[test]
fn audio_import_args_serializes_imports_and_defaults() {
    let mut item = ImportItem::new(r"\Actor-Mixer Hierarchy\Default Work Unit\MySound".to_string());
    item.audio_file = Some(r"C:\Audio\MySound.wav".to_string());

    let defaults = ImportDefaults {
        object_type: Some(ObjectType::Sound),
        ..Default::default()
    };

    let args = CoreAudioImportArgs::new(vec![item])
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
    let options = CoreAudioImportOptions::new()
        .return_fields(vec!["id".to_string(), "name".to_string()])
        .platform(PlatformRef::Guid("{GUID}".to_string()));

    let value = serde_json::to_value(&options).unwrap();
    assert_eq!(value["return"], json![["id", "name"]]);
    assert_eq!(value["platform"], "{GUID}");
}

#[test]
fn project_save_args_serializes_empty_args() {
    let args = CoreProjectSaveArgs::new();
    let value = serde_json::to_value(&args).unwrap();
    assert!(value.as_object().unwrap().is_empty());
}

#[test]
fn ui_get_selected_objects_options_serializes() {
    let options = UiGetSelectedObjectsOptions::new().return_fields(vec!["id".to_string()]);
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

    let resp: CoreGetInfoResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.display_name, "Wwise");
    assert_eq!(resp.platform, "x64");
    assert_eq!(resp.version.display_name, "2024.1.7");
    assert_eq!(resp.version.schema, 108);
    assert_eq!(
        resp.session_id,
        "{FF59687C-48CF-4385-B1C5-CE84B0A63880}".to_string()
    );
    assert_eq!(resp.api_version, 1.0);
    assert_eq!(resp.configuration, "release");
    assert_eq!(resp.process_id, 12345);
    assert_eq!(
        resp.directories.install,
        "C:/Program Files/Audiokinetic/Wwise 2024.1.7.8863"
    );
}

#[test]
fn object_get_response_deserializes() {
    // Real wire shape: the object array is returned under the "return" key.
    let json = json!({
        "return": [
            {
                "id": "{GUID-1}",
                "name": "MySound",
                "type": "Sound",
                "shortId": 12345
            }
        ]
    });

    let resp: CoreObjectGetResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.objects.len(), 1);
    let obj = &resp.objects[0];
    assert_eq!(obj.id, Some("{GUID-1}".to_string()));
    assert_eq!(obj.name, Some("MySound".to_string()));
    assert_eq!(obj.object_type, Some(ObjectType::Sound));
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

    let resp: UiGetSelectedObjectsResponse = serde_json::from_value(json).unwrap();
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
    let name: wwise_waapi::api::primitives::ObjectRef =
        serde_json::from_value(json!("Play_Test")).unwrap();
    assert_eq!(serde_json::to_value(name).unwrap(), "Play_Test");

    let guid: wwise_waapi::api::primitives::ObjectRef =
        serde_json::from_value(json!("{00000000-0000-0000-0000-000000000001}")).unwrap();
    assert_eq!(
        serde_json::to_value(guid).unwrap(),
        "{00000000-0000-0000-0000-000000000001}"
    );

    let path: wwise_waapi::api::primitives::ObjectRef =
        serde_json::from_value(json!(r"\Events\Default Work Unit\Play_Test")).unwrap();
    assert_eq!(
        serde_json::to_value(path).unwrap(),
        r"\Events\Default Work Unit\Play_Test"
    );
}

#[test]
fn object_set_args_serializes_to_expected_shape() {
    let args = CoreObjectSetArgs::new(vec![
        CoreObjectSetItem::new(ObjectRef::path(
            r"\Actor-Mixer Hierarchy\Default Work Unit\MySound",
        ))
        .name("NewName")
        .notes("set from the typed API")
        .property(Property::Volume, -6.0)
        .property(
            Property::OutputBus,
            r"\Master-Mixer Hierarchy\Default Work Unit\Master Audio Bus",
        ),
    ])
    .list_mode(ListMode::Append);

    let value = serde_json::to_value(&args).unwrap();
    let item = &value["objects"][0];
    assert_eq!(
        item["object"],
        r"\Actor-Mixer Hierarchy\Default Work Unit\MySound"
    );
    assert_eq!(item["name"], "NewName");
    assert_eq!(item["notes"], "set from the typed API");
    assert_eq!(item["@Volume"], -6.0);
    assert_eq!(
        item["@OutputBus"],
        r"\Master-Mixer Hierarchy\Default Work Unit\Master Audio Bus"
    );
    assert!(item.get("children").is_none());
    assert_eq!(value["listMode"], "append");
}

#[test]
fn object_set_response_deserializes_with_extra_fields() {
    let raw = json!({
        "objects": [
            {
                "id": "{BAE891BA-E025-43ED-A4D1-4DC55C68E7CA}",
                "name": "Sound",
                "@RTPC": [{"id": "{31DA7AE1-B215-4689-A5F7-67A9C5B3DFDC}", "name": ""}]
            },
            {
                "id": "{7A12D08F-B0D9-4403-9EFA-2E6338C197C1}",
                "name": "Default Work Unit",
                "children": [{"id": "{66666666-7777-8888-9999-AAAAAAAAAAAA}", "name": "Boom"}]
            }
        ]
    });

    let response: CoreObjectSetResponse = serde_json::from_value(raw).unwrap();
    assert_eq!(response.objects.len(), 2);
    assert_eq!(
        response.objects[0].id.as_deref(),
        Some("{BAE891BA-E025-43ED-A4D1-4DC55C68E7CA}")
    );
    assert!(response.objects[0].property(Property::RTPC).is_some());
    assert_eq!(response.objects[1].children.len(), 1);
    assert_eq!(
        response.objects[1].children[0].name.as_deref(),
        Some("Boom")
    );
}

#[test]
fn property_enum_maps_to_bare_and_at_prefixed_strings() {
    assert_eq!(Property::Volume.as_str(), "Volume");
    assert_eq!(Property::Volume.as_at_str(), "@Volume");
    assert_eq!(Property::Volume.as_ref(), "Volume");
    assert_eq!(Property::_3DSpatialization.as_str(), "3DSpatialization");
    assert_eq!(Property::_3DSpatialization.as_at_str(), "@3DSpatialization");
    assert_eq!(serde_json::to_value(Property::Pitch).unwrap(), "Pitch");
    assert_eq!(
        serde_json::from_value::<Property>(json!("Volume")).unwrap(),
        Property::Volume
    );
    assert_eq!(
        serde_json::from_value::<Property>(json!("@Volume")).unwrap(),
        Property::Volume
    );
}

#[test]
fn object_type_enum_maps_to_names_class_ids_and_queries() {
    assert_eq!(ObjectType::Sound.as_str(), "Sound");
    assert_eq!(ObjectType::Sound.as_ref(), "Sound");
    assert_eq!(ObjectType::Sound.class_id(), 65552);
    assert_eq!(ObjectType::from_class_id(65552), Some(ObjectType::Sound));
    assert_eq!(ObjectType::ActorMixer.class_id(), 524304);
    assert_eq!(
        "ActorMixer".parse::<ObjectType>().unwrap(),
        ObjectType::ActorMixer
    );

    let query = ObjectQuery::from(ObjectType::Sound);
    let value = serde_json::to_value(&query).unwrap();
    assert_eq!(value["ofType"], json!(["Sound"]));
}

#[test]
fn waapi_uri_enums_map_to_const_strings() {
    use wwise_waapi::{WaapiFunction, WaapiTopic, waapi_function_api, waapi_topics_api};

    assert_eq!(
        WaapiFunction::ak_wwise_core_getInfo.as_str(),
        waapi_function_api::ak::wwise::core::getInfo
    );
    assert_eq!(
        WaapiFunction::ak_wwise_core_object_move.as_str(),
        waapi_function_api::ak::wwise::core::object::move_
    );
    assert_eq!(
        WaapiTopic::ak_wwise_core_object_created.as_str(),
        waapi_topics_api::ak::wwise::core::object::created
    );
}

#[test]
fn undo_args_serialize() {
    use wwise_waapi::api::generated::ak::wwise::core::undo::{
        CoreUndoCancelGroupArgs, CoreUndoEndGroupArgs,
    };

    let end = CoreUndoEndGroupArgs::new("Batch Rename".to_string());
    assert_eq!(
        serde_json::to_value(end).unwrap(),
        json!({"displayName": "Batch Rename"})
    );

    let cancel = CoreUndoCancelGroupArgs::new().undo(true);
    assert_eq!(serde_json::to_value(cancel).unwrap(), json!({"undo": true}));
}

#[test]
fn transport_args_and_responses_roundtrip() {
    use wwise_waapi::api::generated::ak::wwise::core::transport::{
        CoreTransportCreateArgs, CoreTransportCreateResponse, CoreTransportDestroyArgs,
        CoreTransportExecuteActionArgs, CoreTransportGetListResponse, CoreTransportGetStateArgs,
        CoreTransportGetStateResponse, CoreTransportPrepareArgs, CoreTransportUseOriginalsArgs,
    };
    use wwise_waapi::api::generated::primitives::{GameObjectRef, TransportAction, TransportState};

    let create =
        CoreTransportCreateArgs::new(ObjectRef::guid("{00000000-0000-0000-0000-000000000001}"))
            .game_object(GameObjectRef::game_object(100));
    assert_eq!(
        serde_json::to_value(create).unwrap(),
        json!({
            "object": "{00000000-0000-0000-0000-000000000001}",
            "gameObject": 100
        })
    );

    let create_resp: CoreTransportCreateResponse =
        serde_json::from_value(json!({"transport": 42})).unwrap();
    assert_eq!(create_resp.transport, 42);

    let exec = CoreTransportExecuteActionArgs::new(TransportAction::Play).transport(42u32);
    assert_eq!(
        serde_json::to_value(exec).unwrap(),
        json!({"action": "play", "transport": 42})
    );

    let get_state = CoreTransportGetStateArgs::new(42);
    assert_eq!(
        serde_json::to_value(get_state).unwrap(),
        json!({"transport": 42})
    );

    let state_resp: CoreTransportGetStateResponse =
        serde_json::from_value(json!({"state": "playing"})).unwrap();
    assert_eq!(state_resp.state, TransportState::Playing);

    let list_resp: CoreTransportGetListResponse = serde_json::from_value(json!({
        "list": [
            {
                "transport": 42,
                "object": "{00000000-0000-0000-0000-000000000001}",
                "gameObject": 100
            }
        ]
    }))
    .unwrap();
    assert_eq!(list_resp.list.len(), 1);
    assert_eq!(list_resp.list[0].transport, 42);
    assert_eq!(
        list_resp.list[0].object,
        "{00000000-0000-0000-0000-000000000001}"
    );

    let prep = CoreTransportPrepareArgs::new(ObjectRef::from("Sound:MySound"));
    assert_eq!(
        serde_json::to_value(prep).unwrap(),
        json!({"object": "Sound:MySound"})
    );

    let orig = CoreTransportUseOriginalsArgs::new(true);
    assert_eq!(serde_json::to_value(orig).unwrap(), json!({"enable": true}));

    let destroy = CoreTransportDestroyArgs::new(42);
    assert_eq!(
        serde_json::to_value(destroy).unwrap(),
        json!({"transport": 42})
    );
}

#[test]
fn object_helpers_and_attenuation_curve_roundtrip() {
    use wwise_waapi::api::generated::ak::wwise::core::object::{
        CoreObjectCopyArgs, CoreObjectDiffArgs, CoreObjectDiffResponse,
        CoreObjectGetAttenuationCurveArgs, CoreObjectGetAttenuationCurveResponse,
        CoreObjectMoveArgs, CoreObjectPastePropertiesArgs, CoreObjectSetAttenuationCurveArgs,
        CoreObjectSetNameArgs, CoreObjectSetNotesArgs, CoreObjectSetPropertyArgs,
        CoreObjectSetRandomizerArgs, CoreObjectSetReferenceArgs,
    };
    use wwise_waapi::api::generated::primitives::{
        AttenuationCurveType, AttenuationCurveUse, CurvePoint2D, CurveShape, OnNameConflict,
        PasteMode, PropertyValue,
    };

    let mv = CoreObjectMoveArgs::new(ObjectRef::guid("{OBJ-1}"), ObjectRef::guid("{PARENT-2}"))
        .on_name_conflict(OnNameConflict::Rename);
    assert_eq!(
        serde_json::to_value(mv).unwrap(),
        json!({
            "object": "{OBJ-1}",
            "parent": "{PARENT-2}",
            "onNameConflict": "rename"
        })
    );

    let cp = CoreObjectCopyArgs::new(ObjectRef::guid("{OBJ-1}"), ObjectRef::guid("{PARENT-2}"))
        .on_name_conflict(OnNameConflict::Replace);
    assert_eq!(
        serde_json::to_value(cp).unwrap(),
        json!({
            "object": "{OBJ-1}",
            "parent": "{PARENT-2}",
            "onNameConflict": "replace"
        })
    );

    let set_name = CoreObjectSetNameArgs::new(ObjectRef::guid("{OBJ-1}"), "Renamed".to_string());
    assert_eq!(
        serde_json::to_value(set_name).unwrap(),
        json!({"object": "{OBJ-1}", "value": "Renamed"})
    );

    let set_notes = CoreObjectSetNotesArgs::new(ObjectRef::guid("{OBJ-1}"), "Notes".to_string());
    assert_eq!(
        serde_json::to_value(set_notes).unwrap(),
        json!({"object": "{OBJ-1}", "value": "Notes"})
    );

    let set_prop = CoreObjectSetPropertyArgs::new(
        ObjectRef::guid("{OBJ-1}"),
        Property::Volume,
        PropertyValue::from(-6.0),
    );
    assert_eq!(
        serde_json::to_value(set_prop).unwrap(),
        json!({"object": "{OBJ-1}", "property": "Volume", "value": -6.0})
    );

    let set_ref = CoreObjectSetReferenceArgs::new(
        ObjectRef::guid("{OBJ-1}"),
        Property::OutputBus,
        ObjectRef::path(r"\Master-Mixer Hierarchy\Default Work Unit\Master Audio Bus"),
    );
    assert_eq!(
        serde_json::to_value(set_ref).unwrap(),
        json!({
            "object": "{OBJ-1}",
            "reference": "OutputBus",
            "value": r"\Master-Mixer Hierarchy\Default Work Unit\Master Audio Bus"
        })
    );

    let rand_args = CoreObjectSetRandomizerArgs::new(ObjectRef::guid("{OBJ-1}"), Property::Volume)
        .enabled(true)
        .min(-3.0)
        .max(3.0);
    assert_eq!(
        serde_json::to_value(rand_args).unwrap(),
        json!({
            "object": "{OBJ-1}",
            "property": "Volume",
            "enabled": true,
            "min": -3.0,
            "max": 3.0
        })
    );

    let points = vec![
        CurvePoint2D::new(0.0, 0.0, CurveShape::Linear),
        CurvePoint2D::new(50.0, -6.0, CurveShape::Log2),
        CurvePoint2D::new(100.0, -200.0, CurveShape::Constant),
    ];
    let set_curve = CoreObjectSetAttenuationCurveArgs::new(
        ObjectRef::guid("{ATTEN-1}"),
        AttenuationCurveType::VolumeDryUsage,
        AttenuationCurveUse::Custom,
        points.clone(),
    );
    let set_curve_json = serde_json::to_value(&set_curve).unwrap();
    assert_eq!(set_curve_json["curveType"], "VolumeDryUsage");
    assert_eq!(set_curve_json["use"], "Custom");
    assert_eq!(set_curve_json["points"].as_array().unwrap().len(), 3);
    assert_eq!(set_curve_json["points"][1]["shape"], "Log2");

    let get_curve = CoreObjectGetAttenuationCurveArgs::new(
        ObjectRef::guid("{ATTEN-1}"),
        AttenuationCurveType::VolumeDryUsage,
    );
    assert_eq!(
        serde_json::to_value(get_curve).unwrap(),
        json!({"object": "{ATTEN-1}", "curveType": "VolumeDryUsage"})
    );

    let curve_resp: CoreObjectGetAttenuationCurveResponse = serde_json::from_value(json!({
        "curveType": "VolumeDryUsage",
        "use": "Custom",
        "points": [
            {"x": 0.0, "y": 0.0, "shape": "Linear"},
            {"x": 50.0, "y": -6.0, "shape": "Log2"},
            {"x": 100.0, "y": -200.0, "shape": "Constant"}
        ]
    }))
    .unwrap();
    assert_eq!(curve_resp.curve_type, AttenuationCurveType::VolumeDryUsage);
    assert_eq!(curve_resp.use_, AttenuationCurveUse::Custom);
    assert_eq!(curve_resp.points, points);

    let diff = CoreObjectDiffArgs::new(ObjectRef::guid("{SRC}"), ObjectRef::guid("{DST}"));
    assert_eq!(
        serde_json::to_value(diff).unwrap(),
        json!({"source": "{SRC}", "target": "{DST}"})
    );
    let diff_resp: CoreObjectDiffResponse = serde_json::from_value(json!({
        "properties": ["Volume", "Pitch"],
        "lists": []
    }))
    .unwrap();
    assert_eq!(diff_resp.properties, vec!["Volume", "Pitch"]);
    assert!(diff_resp.lists.is_empty());

    let paste = CoreObjectPastePropertiesArgs::new(
        ObjectRef::guid("{SRC}"),
        vec![ObjectRef::guid("{DST}")],
    )
    .paste_mode(PasteMode::ReplaceEntire)
    .inclusion(vec![Property::Volume, Property::Pitch]);
    assert_eq!(
        serde_json::to_value(paste).unwrap(),
        json!({
            "source": "{SRC}",
            "targets": ["{DST}"],
            "pasteMode": "replaceEntire",
            "inclusion": ["Volume", "Pitch"]
        })
    );
}

#[test]
fn switch_container_assignments_roundtrip() {
    use wwise_waapi::api::generated::ak::wwise::core::switchContainer::{
        CoreSwitchContainerAddAssignmentArgs, CoreSwitchContainerGetAssignmentsArgs,
        CoreSwitchContainerGetAssignmentsResponse, CoreSwitchContainerRemoveAssignmentArgs,
    };
    use wwise_waapi::api::generated::primitives::SwitchContainerAssignment;

    let add = CoreSwitchContainerAddAssignmentArgs::new(
        ObjectRef::guid("{CHILD-1}"),
        ObjectRef::guid("{SWITCH-1}"),
    );
    assert_eq!(
        serde_json::to_value(add).unwrap(),
        json!({"child": "{CHILD-1}", "stateOrSwitch": "{SWITCH-1}"})
    );

    let rem = CoreSwitchContainerRemoveAssignmentArgs::new(
        ObjectRef::guid("{CHILD-1}"),
        ObjectRef::guid("{SWITCH-1}"),
    );
    assert_eq!(
        serde_json::to_value(rem).unwrap(),
        json!({"child": "{CHILD-1}", "stateOrSwitch": "{SWITCH-1}"})
    );

    let get = CoreSwitchContainerGetAssignmentsArgs::new(ObjectRef::guid("{CONTAINER-1}"));
    assert_eq!(
        serde_json::to_value(get).unwrap(),
        json!({"id": "{CONTAINER-1}"})
    );

    let resp: CoreSwitchContainerGetAssignmentsResponse = serde_json::from_value(json!({
        "return": [
            {
                "child": "{CHILD-1}",
                "stateOrSwitch": "{SWITCH-1}"
            }
        ]
    }))
    .unwrap();
    assert_eq!(
        resp.assignments,
        vec![SwitchContainerAssignment {
            child: "{CHILD-1}".to_string(),
            state_or_switch: "{SWITCH-1}".to_string(),
        }]
    );
}

#[test]
fn project_info_and_ui_commands_roundtrip() {
    use wwise_waapi::api::generated::ak::wwise::core::CoreGetProjectInfoResponse;
    use wwise_waapi::api::generated::ak::wwise::ui::commands::{
        UiCommandsExecuteArgs, UiCommandsGetCommandsResponse,
    };
    use wwise_waapi::api::generated::ak::wwise::ui::project::{
        UiProjectCloseArgs, UiProjectCloseResponse, UiProjectOpenArgs,
    };
    use wwise_waapi::api::generated::primitives::ProjectMigrationAction;

    let proj: CoreGetProjectInfoResponse = serde_json::from_value(json!({
        "name": "WwiseProject",
        "displayTitle": "WwiseProject*",
        "path": "/Projects/WwiseProject/WwiseProject.wproj",
        "id": "{PROJ-ID}",
        "isDirty": true,
        "currentLanguageId": "{LANG-1}",
        "referenceLanguageId": "{LANG-1}",
        "currentPlatformId": "{PLAT-1}",
        "defaultConversion": {
            "id": "{CONV-1}",
            "name": "Default Conversion Settings"
        },
        "platforms": [
            {
                "id": "{PLAT-1}",
                "name": "Mac",
                "baseName": "Mac",
                "baseDisplayName": "macOS",
                "soundBankPath": "/Projects/WwiseProject/GeneratedSoundBanks/Mac",
                "copiedMediaPath": "/Projects/WwiseProject/GeneratedSoundBanks/Mac"
            }
        ],
        "languages": [
            {
                "id": "{LANG-1}",
                "name": "English(US)",
                "shortId": 684519430
            }
        ],
        "directories": {
            "root": "/Projects/WwiseProject",
            "cache": "/Projects/WwiseProject/.cache",
            "originals": "/Projects/WwiseProject/Originals",
            "soundBankOutputRoot": "/Projects/WwiseProject/GeneratedSoundBanks",
            "commands": "/Projects/WwiseProject/Commands",
            "properties": "/Projects/WwiseProject/Properties"
        }
    }))
    .unwrap();
    assert_eq!(proj.name, "WwiseProject");
    assert_eq!(proj.platforms[0].base_display_name, "macOS");
    assert_eq!(proj.languages[0].short_id, 684519430);
    assert_eq!(proj.directories.root, "/Projects/WwiseProject");

    let exec = UiCommandsExecuteArgs::new("FindInProjectExplorerSyncGroup1".to_string())
        .objects(vec![ObjectRef::guid("{OBJ-1}")]);
    assert_eq!(
        serde_json::to_value(exec).unwrap(),
        json!({
            "command": "FindInProjectExplorerSyncGroup1",
            "objects": ["{OBJ-1}"]
        })
    );

    let cmds: UiCommandsGetCommandsResponse = serde_json::from_value(json!({
        "commands": ["Inspect", "FindInProjectExplorerSyncGroup1"]
    }))
    .unwrap();
    assert_eq!(cmds.commands.len(), 2);

    let open = UiProjectOpenArgs::new("/Projects/WwiseProject/WwiseProject.wproj".to_string())
        .on_migration_required(ProjectMigrationAction::Migrate)
        .bypass_save(true);
    assert_eq!(
        serde_json::to_value(open).unwrap(),
        json!({
            "path": "/Projects/WwiseProject/WwiseProject.wproj",
            "onMigrationRequired": "migrate",
            "bypassSave": true
        })
    );

    let close = UiProjectCloseArgs::new().bypass_save(true);
    assert_eq!(
        serde_json::to_value(close).unwrap(),
        json!({"bypassSave": true})
    );
    let close_resp: UiProjectCloseResponse =
        serde_json::from_value(json!({"hadProjectOpen": true})).unwrap();
    assert!(close_resp.had_project_open);
}

#[test]
fn soundengine_int_enums_serialize_as_ints() {
    use wwise_waapi::api::generated::primitives::{
        ActionOnEventType, AkCurveInterpolation, MultiPositionType,
    };

    assert_eq!(ActionOnEventType::Stop.as_u32(), 0);
    assert_eq!(ActionOnEventType::ReleaseEnvelope.as_u32(), 4);
    assert_eq!(
        ActionOnEventType::from_u32(2),
        Some(ActionOnEventType::Resume)
    );
    assert_eq!(ActionOnEventType::from_u32(99), None);
    assert_eq!(serde_json::to_value(ActionOnEventType::Pause).unwrap(), 1);
    assert_eq!(
        serde_json::from_value::<ActionOnEventType>(json!(3)).unwrap(),
        ActionOnEventType::Break
    );

    assert_eq!(AkCurveInterpolation::Linear.as_u32(), 4);
    assert_eq!(AkCurveInterpolation::Constant.as_u32(), 9);
    assert_eq!(
        AkCurveInterpolation::from_u32(1),
        Some(AkCurveInterpolation::Sine)
    );
    assert_eq!(
        serde_json::to_value(AkCurveInterpolation::SCurve).unwrap(),
        5
    );

    assert_eq!(MultiPositionType::SingleSource.as_u32(), 0);
    assert_eq!(MultiPositionType::MultiDirections.as_u32(), 2);
    assert_eq!(
        serde_json::to_value(MultiPositionType::MultiSources).unwrap(),
        1
    );
}

#[test]
fn soundengine_switch_and_state_roundtrip() {
    use wwise_waapi::api::generated::ak::soundengine::{
        SoundengineGetStateArgs, SoundengineGetSwitchArgs, SoundengineSetStateArgs,
        SoundengineSetSwitchArgs,
    };
    use wwise_waapi::api::generated::primitives::GameObjectRef;

    let set_switch = SoundengineSetSwitchArgs::new("MySwitchGroup", "On")
        .game_object(GameObjectRef::game_object(7));
    assert_eq!(
        serde_json::to_value(set_switch).unwrap(),
        json!({"switchGroup": "MySwitchGroup", "switchState": "On", "gameObject": 7})
    );

    let set_state = SoundengineSetStateArgs::new("MyStateGroup", "Combat");
    assert_eq!(
        serde_json::to_value(set_state).unwrap(),
        json!({"stateGroup": "MyStateGroup", "state": "Combat"})
    );

    let get_switch =
        SoundengineGetSwitchArgs::new(ObjectRef::guid("{00000000-0000-0000-0000-0000000000AA}"))
            .game_object(GameObjectRef::game_object(7));
    assert_eq!(
        serde_json::to_value(get_switch).unwrap(),
        json!({"switchGroup": "{00000000-0000-0000-0000-0000000000AA}", "gameObject": 7})
    );

    let get_state =
        SoundengineGetStateArgs::new(ObjectRef::guid("{00000000-0000-0000-0000-0000000000BB}"));
    assert_eq!(
        serde_json::to_value(get_state).unwrap(),
        json!({"stateGroup": "{00000000-0000-0000-0000-0000000000BB}"})
    );
}

#[test]
fn soundengine_actions_seek_and_events_roundtrip() {
    use wwise_waapi::api::generated::ak::soundengine::{
        SoundengineExecuteActionOnEventArgs, SoundenginePostTriggerArgs,
        SoundengineSeekOnEventArgs, SoundengineStopPlayingIdArgs,
    };
    use wwise_waapi::api::generated::primitives::{
        ActionOnEventType, AkCurveInterpolation, GameObjectRef,
    };

    let exec = SoundengineExecuteActionOnEventArgs::new(
        "Play_Test",
        ActionOnEventType::Stop,
        GameObjectRef::game_object(1),
        500,
        AkCurveInterpolation::Linear,
    );
    assert_eq!(
        serde_json::to_value(exec).unwrap(),
        json!({
            "event": "Play_Test",
            "actionType": 0,
            "gameObject": 1,
            "transitionDuration": 500,
            "fadeCurve": 4
        })
    );

    let stop = SoundengineStopPlayingIdArgs::new(42, 100, AkCurveInterpolation::SCurve);
    assert_eq!(
        serde_json::to_value(stop).unwrap(),
        json!({"playingId": 42, "transitionDuration": 100, "fadeCurve": 5})
    );

    let seek =
        SoundengineSeekOnEventArgs::new("Play_Test", GameObjectRef::game_object(1), true, 42)
            .position(1000)
            .percent(50.0);
    assert_eq!(
        serde_json::to_value(seek).unwrap(),
        json!({
            "event": "Play_Test",
            "gameObject": 1,
            "seekToNearestMarker": true,
            "playingId": 42,
            "position": 1000,
            "percent": 50.0
        })
    );

    let trigger =
        SoundenginePostTriggerArgs::new("MyTrigger").game_object(GameObjectRef::game_object(3));
    assert_eq!(
        serde_json::to_value(trigger).unwrap(),
        json!({"trigger": "MyTrigger", "gameObject": 3})
    );
}

#[test]
fn soundengine_positions_volume_and_listeners_roundtrip() {
    use wwise_waapi::api::generated::ak::soundengine::{
        SoundengineLoadBankArgs, SoundenginePostMsgMonitorArgs, SoundengineResetRtpcValueArgs,
        SoundengineSetGameObjectAuxSendValuesArgs, SoundengineSetGameObjectOutputBusVolumeArgs,
        SoundengineSetListenerSpatializationArgs, SoundengineSetListenersArgs,
        SoundengineSetMultiplePositionsArgs, SoundengineSetObjectObstructionAndOcclusionArgs,
        SoundengineSetScalingFactorArgs, SoundengineStopAllArgs, SoundengineUnloadBankArgs,
    };
    use wwise_waapi::api::generated::primitives::{
        AuxSendValue, GameObjectRef, MultiPositionType, Position,
    };

    let aux = SoundengineSetGameObjectAuxSendValuesArgs::new(
        GameObjectRef::game_object(1),
        vec![AuxSendValue::new(1u64, "MyAuxBus", -3.0)],
    );
    assert_eq!(
        serde_json::to_value(aux).unwrap(),
        json!({
            "gameObject": 1,
            "auxSendValues": [{"listener": 1, "auxBus": "MyAuxBus", "controlValue": -3.0}]
        })
    );

    let positions = SoundengineSetMultiplePositionsArgs::new(
        GameObjectRef::game_object(1),
        vec![Position::origin().into()],
        MultiPositionType::MultiSources,
    );
    assert_eq!(
        serde_json::to_value(positions).unwrap(),
        json!({
            "gameObject": 1,
            "positions": [{
                "position": {
                    "orientationFront": {"x": 0.0, "y": 0.0, "z": 1.0},
                    "orientationTop": {"x": 0.0, "y": 1.0, "z": 0.0},
                    "position": {"x": 0.0, "y": 0.0, "z": 0.0}
                }
            }],
            "multiPositionType": 1
        })
    );

    let listeners = SoundengineSetListenersArgs::new(
        GameObjectRef::game_object(1),
        vec![GameObjectRef::game_object(2)],
    );
    assert_eq!(
        serde_json::to_value(listeners).unwrap(),
        json!({"emitter": 1, "listeners": [2]})
    );

    let spatial = SoundengineSetListenerSpatializationArgs::new(
        GameObjectRef::game_object(1),
        true,
        1,
        vec![0.0, -6.0],
    );
    assert_eq!(
        serde_json::to_value(spatial).unwrap(),
        json!({"listener": 1, "spatialized": true, "channelConfig": 1, "volumeOffsets": [0.0, -6.0]})
    );

    let occ = SoundengineSetObjectObstructionAndOcclusionArgs::new(
        GameObjectRef::game_object(1),
        GameObjectRef::game_object(2),
        0.5,
        0.25,
    );
    assert_eq!(
        serde_json::to_value(occ).unwrap(),
        json!({"emitter": 1, "listener": 2, "obstructionLevel": 0.5, "occlusionLevel": 0.25})
    );

    let scaling = SoundengineSetScalingFactorArgs::new(GameObjectRef::game_object(1), 0.75);
    assert_eq!(
        serde_json::to_value(scaling).unwrap(),
        json!({"gameObject": 1, "attenuationScalingFactor": 0.75})
    );

    let bus_vol = SoundengineSetGameObjectOutputBusVolumeArgs::new(
        GameObjectRef::game_object(1),
        GameObjectRef::game_object(2),
        -6.0,
    );
    assert_eq!(
        serde_json::to_value(bus_vol).unwrap(),
        json!({"emitter": 1, "listener": 2, "controlValue": -6.0})
    );

    let reset =
        SoundengineResetRtpcValueArgs::new("Volume").game_object(GameObjectRef::game_object(1));
    assert_eq!(
        serde_json::to_value(reset).unwrap(),
        json!({"rtpc": "Volume", "gameObject": 1})
    );

    let load = SoundengineLoadBankArgs::new("Init");
    assert_eq!(
        serde_json::to_value(load).unwrap(),
        json!({"soundBank": "Init"})
    );

    let unload = SoundengineUnloadBankArgs::new("Init");
    assert_eq!(
        serde_json::to_value(unload).unwrap(),
        json!({"soundBank": "Init"})
    );

    let stop_all = SoundengineStopAllArgs::new().game_object(GameObjectRef::game_object(1));
    assert_eq!(
        serde_json::to_value(stop_all).unwrap(),
        json!({"gameObject": 1})
    );

    let msg = SoundenginePostMsgMonitorArgs::new("hello".to_string());
    assert_eq!(
        serde_json::to_value(msg).unwrap(),
        json!({"message": "hello"})
    );
}

#[test]
fn object_introspection_roundtrip() {
    use wwise_waapi::Property;
    use wwise_waapi::api::generated::ak::wwise::core::object::{
        CoreObjectGetPropertyAndReferenceNamesArgs, CoreObjectGetPropertyAndReferenceNamesResponse,
        CoreObjectGetPropertyInfoArgs, CoreObjectGetPropertyInfoResponse,
        CoreObjectGetTypesResponse, CoreObjectIsLinkedArgs, CoreObjectIsLinkedResponse,
        CoreObjectIsPropertyEnabledArgs, CoreObjectIsPropertyEnabledResponse,
        CoreObjectSetLinkedArgs, CoreObjectSetStateGroupsArgs, CoreObjectSetStatePropertiesArgs,
    };
    use wwise_waapi::api::generated::primitives::RtpcSupportMode;

    let types: CoreObjectGetTypesResponse = serde_json::from_value(json!({
        "return": [
            {"type": "Sound", "classId": 65552, "name": "Sound"},
            {"type": "ActorMixer", "classId": 524304, "name": "Actor-Mixer"}
        ]
    }))
    .unwrap();
    assert_eq!(types.types.len(), 2);
    assert_eq!(types.types[0].object_type, "Sound");
    assert_eq!(types.types[0].class_id, 65552);
    assert_eq!(types.types[1].object_type, "ActorMixer");

    let names_args =
        CoreObjectGetPropertyAndReferenceNamesArgs::new().object(ObjectRef::guid("{OBJ-1}"));
    assert_eq!(
        serde_json::to_value(names_args).unwrap(),
        json!({"object": "{OBJ-1}"})
    );
    let names: CoreObjectGetPropertyAndReferenceNamesResponse = serde_json::from_value(json!({
        "return": ["Volume", "OutputBus"]
    }))
    .unwrap();
    assert_eq!(names.names, vec!["Volume", "OutputBus"]);

    let info_args =
        CoreObjectGetPropertyInfoArgs::new(Property::Volume).object(ObjectRef::guid("{OBJ-1}"));
    assert_eq!(
        serde_json::to_value(info_args).unwrap(),
        json!({"object": "{OBJ-1}", "property": "Volume"})
    );
    let info: CoreObjectGetPropertyInfoResponse = serde_json::from_value(json!({
        "type": "Real64",
        "name": "Volume",
        "default": 0.0,
        "audioEngineId": 0,
        "supports": {"rtpc": "Additive", "randomizer": true, "unlink": false}
    }))
    .unwrap();
    assert_eq!(info.object_type.as_deref(), Some("Real64"));
    assert_eq!(info.name.as_deref(), Some("Volume"));
    assert_eq!(info.default, Some(json!(0.0)));
    assert_eq!(info.audio_engine_id, Some(0));
    let supports = info.supports.unwrap();
    assert_eq!(supports.rtpc, Some(RtpcSupportMode::Additive));
    assert_eq!(supports.randomizer, Some(true));
    assert_eq!(supports.unlink, Some(false));

    let is_linked = CoreObjectIsLinkedArgs::new(
        ObjectRef::guid("{OBJ-1}"),
        Property::Volume,
        PlatformRef::Name("Windows".to_string()),
    );
    assert_eq!(
        serde_json::to_value(is_linked).unwrap(),
        json!({"object": "{OBJ-1}", "property": "Volume", "platform": "Windows"})
    );
    let linked_resp: CoreObjectIsLinkedResponse =
        serde_json::from_value(json!({"linked": true})).unwrap();
    assert_eq!(linked_resp.linked, Some(true));

    let is_enabled = CoreObjectIsPropertyEnabledArgs::new(
        ObjectRef::guid("{OBJ-1}"),
        PlatformRef::Name("Windows".to_string()),
        Property::Volume,
    );
    assert_eq!(
        serde_json::to_value(is_enabled).unwrap(),
        json!({"object": "{OBJ-1}", "property": "Volume", "platform": "Windows"})
    );
    let enabled_resp: CoreObjectIsPropertyEnabledResponse =
        serde_json::from_value(json!({"return": false})).unwrap();
    assert_eq!(enabled_resp.enabled, Some(false));

    let set_linked = CoreObjectSetLinkedArgs::new(
        ObjectRef::guid("{OBJ-1}"),
        Property::Volume,
        PlatformRef::Name("Windows".to_string()),
        true,
    );
    assert_eq!(
        serde_json::to_value(set_linked).unwrap(),
        json!({"object": "{OBJ-1}", "property": "Volume", "platform": "Windows", "linked": true})
    );

    let set_groups = CoreObjectSetStateGroupsArgs::new(
        ObjectRef::guid("{OBJ-1}"),
        vec![ObjectRef::guid("{STATE-GROUP-1}")],
    );
    assert_eq!(
        serde_json::to_value(set_groups).unwrap(),
        json!({"object": "{OBJ-1}", "stateGroups": ["{STATE-GROUP-1}"]})
    );

    let set_props = CoreObjectSetStatePropertiesArgs::new(
        ObjectRef::guid("{OBJ-1}"),
        vec!["Volume".to_string(), "Pitch".to_string()],
    );
    assert_eq!(
        serde_json::to_value(set_props).unwrap(),
        json!({"object": "{OBJ-1}", "stateProperties": ["Volume", "Pitch"]})
    );
}

#[test]
fn singles_roundtrip() {
    use wwise_waapi::api::generated::ak::wwise::core::CorePingResponse;
    use wwise_waapi::api::generated::ak::wwise::core::gameParameter::CoreGameParameterSetRangeArgs;
    use wwise_waapi::api::generated::ak::wwise::core::sound::CoreSoundSetActiveSourceArgs;
    use wwise_waapi::api::generated::primitives::OnCurveUpdate;

    let active = CoreSoundSetActiveSourceArgs::new(
        ObjectRef::guid("{SOUND-1}"),
        ObjectRef::guid("{SOURCE-1}"),
    )
    .platform(PlatformRef::Name("Windows".to_string()));
    assert_eq!(
        serde_json::to_value(active).unwrap(),
        json!({"sound": "{SOUND-1}", "source": "{SOURCE-1}", "platform": "Windows"})
    );

    let range = CoreGameParameterSetRangeArgs::new(
        ObjectRef::guid("{GAMEPARAM-1}"),
        -100.0,
        0.0,
        OnCurveUpdate::Stretch,
    );
    assert_eq!(
        serde_json::to_value(range).unwrap(),
        json!({"object": "{GAMEPARAM-1}", "min": -100.0, "max": 0.0, "onCurveUpdate": "stretch"})
    );
    let preserve = CoreGameParameterSetRangeArgs::new(
        ObjectRef::guid("{GAMEPARAM-1}"),
        0.0,
        100.0,
        OnCurveUpdate::PreserveX,
    );
    assert_eq!(
        serde_json::to_value(preserve).unwrap()["onCurveUpdate"],
        "preserveX"
    );

    let ping: CorePingResponse = serde_json::from_value(json!({"isAvailable": true})).unwrap();
    assert!(ping.is_available);
}

#[test]
fn blend_container_roundtrip() {
    use wwise_waapi::api::generated::ak::wwise::core::blendContainer::{
        CoreBlendContainerAddAssignmentArgs, CoreBlendContainerAddAssignmentResponse,
        CoreBlendContainerAddTrackArgs, CoreBlendContainerAddTrackResponse,
        CoreBlendContainerGetAssignmentsArgs, CoreBlendContainerGetAssignmentsResponse,
        CoreBlendContainerRemoveAssignmentArgs,
    };
    use wwise_waapi::api::generated::primitives::{BlendEdge, BlendFadeMode, BlendTrackAssignment};

    let add = CoreBlendContainerAddAssignmentArgs::new(
        "{TRACK-1}".to_string(),
        ObjectRef::guid("{CHILD-1}"),
    )
    .index(0)
    .edges(vec![BlendEdge {
        fade_mode: BlendFadeMode::Manual,
        fade_shape: wwise_waapi::api::generated::primitives::CurveShape::Linear,
        edge_position: 0.5,
        fade_position: Some(0.25),
    }]);
    assert_eq!(
        serde_json::to_value(add).unwrap(),
        json!({
            "object": "{TRACK-1}",
            "child": "{CHILD-1}",
            "index": 0,
            "edges": [{"fadeMode": "Manual", "fadeShape": "Linear", "edgePosition": 0.5, "fadePosition": 0.25}]
        })
    );

    let add_resp: CoreBlendContainerAddAssignmentResponse = serde_json::from_value(json!({
        "child": "{CHILD-1}",
        "index": 0,
        "edges": [{"fadeMode": "Automatic", "fadeShape": "SCurve", "edgePosition": 0.0}]
    }))
    .unwrap();
    assert_eq!(add_resp.child.as_deref(), Some("{CHILD-1}"));
    assert_eq!(add_resp.edges.len(), 1);

    let add_track =
        CoreBlendContainerAddTrackArgs::new(ObjectRef::guid("{BLEND-1}"), "Track A".to_string());
    assert_eq!(
        serde_json::to_value(add_track).unwrap(),
        json!({"object": "{BLEND-1}", "name": "Track A"})
    );
    let track_resp: CoreBlendContainerAddTrackResponse = serde_json::from_value(json!({
        "id": "{TRACK-1}",
        "type": "BlendTrack",
        "shortId": 100,
        "name": "Track A"
    }))
    .unwrap();
    assert_eq!(track_resp.id.as_deref(), Some("{TRACK-1}"));
    assert_eq!(track_resp.short_id, Some(100));

    let get = CoreBlendContainerGetAssignmentsArgs::new("{TRACK-1}".to_string());
    assert_eq!(
        serde_json::to_value(get).unwrap(),
        json!({"object": "{TRACK-1}"})
    );
    let get_resp: CoreBlendContainerGetAssignmentsResponse = serde_json::from_value(json!({
        "return": [{"child": "{CHILD-1}", "index": 0}]
    }))
    .unwrap();
    assert_eq!(
        get_resp.assignments,
        vec![BlendTrackAssignment {
            child: "{CHILD-1}".to_string(),
            index: Some(0),
            edges: None,
        }]
    );

    let remove = CoreBlendContainerRemoveAssignmentArgs::new(
        "{TRACK-1}".to_string(),
        ObjectRef::guid("{CHILD-1}"),
    );
    assert_eq!(
        serde_json::to_value(remove).unwrap(),
        json!({"object": "{TRACK-1}", "child": "{CHILD-1}"})
    );
}

#[test]
fn soundbank_roundtrip() {
    use wwise_waapi::api::generated::ak::wwise::core::soundbank::{
        CoreSoundbankConvertExternalSourcesArgs, CoreSoundbankGenerateArgs,
        CoreSoundbankGenerateResponse, CoreSoundbankGetInclusionsArgs,
        CoreSoundbankGetInclusionsResponse, CoreSoundbankProcessDefinitionFilesArgs,
        CoreSoundbankSetInclusionsArgs,
    };
    use wwise_waapi::api::generated::primitives::{
        ExternalSourceInfo, LogSeverity, SoundBankInclusion, SoundBankInclusionFilter,
        SoundBankInclusionOperation, SoundBankInclusionType, SoundBankInfo,
    };

    let generate = CoreSoundbankGenerateArgs::new()
        .platforms(vec!["Mac".to_string()])
        .languages(vec!["English(US)".into()])
        .soundbanks(vec![
            SoundBankInfo::new("Main")
                .inclusions(vec![
                    SoundBankInclusionType::Event,
                    SoundBankInclusionType::Structure,
                ])
                .events(vec![ObjectRef::guid("{EVENT-1}")])
                .rebuild(true),
        ])
        .skip_languages(false)
        .write_to_disk(true);
    let gen_json = serde_json::to_value(&generate).unwrap();
    assert_eq!(gen_json["platforms"], json!(["Mac"]));
    assert_eq!(gen_json["soundbanks"][0]["name"], "Main");
    assert_eq!(
        gen_json["soundbanks"][0]["inclusions"],
        json!(["event", "structure"])
    );
    assert_eq!(gen_json["soundbanks"][0]["rebuild"], true);
    assert_eq!(gen_json["writeToDisk"], true);

    let gen_resp: CoreSoundbankGenerateResponse = serde_json::from_value(json!({
        "logs": [{
            "severity": "Message",
            "time": 1710000000,
            "messageId": "SB-1",
            "message": "SoundBank generated"
        }]
    }))
    .unwrap();
    assert_eq!(gen_resp.logs.len(), 1);
    assert_eq!(gen_resp.logs[0].severity, LogSeverity::Message);
    assert_eq!(gen_resp.logs[0].message_id, "SB-1");

    let get_inc = CoreSoundbankGetInclusionsArgs::new(ObjectRef::guid("{BANK-1}"));
    assert_eq!(
        serde_json::to_value(get_inc).unwrap(),
        json!({"soundbank": "{BANK-1}"})
    );
    let inc_resp: CoreSoundbankGetInclusionsResponse = serde_json::from_value(json!({
        "inclusions": [{"object": "{OBJ-1}", "filter": ["events", "media"]}]
    }))
    .unwrap();
    assert_eq!(inc_resp.inclusions.len(), 1);
    assert_eq!(
        inc_resp.inclusions[0].filter,
        vec![
            SoundBankInclusionFilter::Events,
            SoundBankInclusionFilter::Media
        ]
    );

    let set_inc = CoreSoundbankSetInclusionsArgs::new(
        ObjectRef::guid("{BANK-1}"),
        vec![
            SoundBankInclusion::new(ObjectRef::guid("{OBJ-1}"))
                .filter(SoundBankInclusionFilter::Events),
        ],
        SoundBankInclusionOperation::Add,
    );
    assert_eq!(
        serde_json::to_value(set_inc).unwrap(),
        json!({
            "soundbank": "{BANK-1}",
            "inclusions": [{"object": "{OBJ-1}", "filter": ["events"]}],
            "operation": "add"
        })
    );

    let proc = CoreSoundbankProcessDefinitionFilesArgs::new(vec!["/path/def.wdef".to_string()]);
    assert_eq!(
        serde_json::to_value(proc).unwrap(),
        json!({"files": ["/path/def.wdef"]})
    );

    let convert = CoreSoundbankConvertExternalSourcesArgs::new(vec![
        ExternalSourceInfo::new(
            PlatformRef::Name("Mac".to_string()),
            "/in/file.wem".to_string(),
        )
        .output("/out/file.wem".to_string()),
    ]);
    assert_eq!(
        serde_json::to_value(convert).unwrap(),
        json!({
            "sources": [{"platform": "Mac", "input": "/in/file.wem", "output": "/out/file.wem"}]
        })
    );
}

#[test]
fn audio_mute_convert_and_import_roundtrip() {
    use wwise_waapi::api::generated::ak::wwise::core::audio::{
        CoreAudioConvertArgs, CoreAudioConvertResponse, CoreAudioImportTabDelimitedArgs,
        CoreAudioMuteArgs, CoreAudioSetConversionPluginArgs, CoreAudioSoloArgs,
    };
    use wwise_waapi::api::generated::primitives::LogSeverity;

    let mute = CoreAudioMuteArgs::new(vec![ObjectRef::guid("{OBJ-1}")], true);
    assert_eq!(
        serde_json::to_value(mute).unwrap(),
        json!({"objects": ["{OBJ-1}"], "value": true})
    );

    let solo = CoreAudioSoloArgs::new(vec![ObjectRef::guid("{OBJ-1}")], false);
    assert_eq!(
        serde_json::to_value(solo).unwrap(),
        json!({"objects": ["{OBJ-1}"], "value": false})
    );

    let convert = CoreAudioConvertArgs::new(
        vec![ObjectRef::guid("{OBJ-1}")],
        vec![PlatformRef::Name("Mac".to_string())],
        vec!["English(US)".into()],
    );
    assert_eq!(
        serde_json::to_value(convert).unwrap(),
        json!({"objects": ["{OBJ-1}"], "platforms": ["Mac"], "languages": ["English(US)"]})
    );
    let convert_resp: CoreAudioConvertResponse = serde_json::from_value(json!({
        "errors": [{"severity": "Error", "message": "Failed"}]
    }))
    .unwrap();
    assert_eq!(convert_resp.errors.len(), 1);
    assert_eq!(convert_resp.errors[0].severity, LogSeverity::Error);

    let import = CoreAudioImportTabDelimitedArgs::new(
        "English(US)".to_string(),
        "createNew".to_string(),
        "/tmp/import.tsv".to_string(),
    )
    .import_location(ObjectRef::guid("{LOC-1}"))
    .auto_add_to_source_control(false);
    assert_eq!(
        serde_json::to_value(import).unwrap(),
        json!({
            "importLanguage": "English(US)",
            "importOperation": "createNew",
            "importFile": "/tmp/import.tsv",
            "importLocation": "{LOC-1}",
            "autoAddToSourceControl": false
        })
    );

    let set_plugin = CoreAudioSetConversionPluginArgs::new(
        ObjectRef::guid("{CONV-1}"),
        PlatformRef::Name("Mac".to_string()),
        "PCM".to_string(),
    );
    assert_eq!(
        serde_json::to_value(set_plugin).unwrap(),
        json!({"conversion": "{CONV-1}", "platform": "Mac", "plugin": "PCM"})
    );
}

#[test]
fn topic_payloads_deserialize() {
    use wwise_waapi::api::generated::ak::wwise::core::object::{
        CoreObjectCreatedPayload, CoreObjectNameChangedPayload, CoreObjectPropertyChangedPayload,
    };
    use wwise_waapi::api::generated::ak::wwise::core::transport::CoreTransportStateChangedPayload;
    use wwise_waapi::api::generated::primitives::TransportState;

    let created: CoreObjectCreatedPayload = serde_json::from_value(json!({
        "object": {"id": "{OBJ-1}", "name": "MySound", "type": "Sound"}
    }))
    .unwrap();
    assert_eq!(created.object.id.as_deref(), Some("{OBJ-1}"));
    assert_eq!(created.object.name.as_deref(), Some("MySound"));

    let renamed: CoreObjectNameChangedPayload = serde_json::from_value(json!({
        "object": {"id": "{OBJ-1}", "name": "Renamed"},
        "newName": "Renamed",
        "oldName": "MySound"
    }))
    .unwrap();
    assert_eq!(renamed.new_name, "Renamed");
    assert_eq!(renamed.old_name, "MySound");

    let prop: CoreObjectPropertyChangedPayload = serde_json::from_value(json!({
        "object": {"id": "{OBJ-1}"},
        "property": "Volume",
        "platform": "Mac",
        "old": -3.0,
        "new": -6.0
    }))
    .unwrap();
    assert_eq!(prop.property, wwise_waapi::Property::Volume);
    assert_eq!(prop.old, json!(-3.0));
    assert_eq!(prop.new, json!(-6.0));

    let state: CoreTransportStateChangedPayload = serde_json::from_value(json!({
        "object": "{OBJ-1}",
        "transport": 42,
        "state": "playing"
    }))
    .unwrap();
    assert_eq!(state.transport, 42);
    assert_eq!(state.state, TransportState::Playing);
    assert!(state.game_object.is_none());
}

#[test]
fn remaining_topic_payloads_deserialize() {
    use wwise_waapi::api::generated::ak::wwise::core::audio::CoreAudioImportedPayload;
    use wwise_waapi::api::generated::ak::wwise::core::log::CoreLogItemAddedPayload;
    use wwise_waapi::api::generated::ak::wwise::core::object::{
        CoreObjectAttenuationCurveChangedPayload, CoreObjectAttenuationCurveLinkChangedPayload,
        CoreObjectChildAddedPayload, CoreObjectChildRemovedPayload, CoreObjectCurveChangedPayload,
        CoreObjectNotesChangedPayload, CoreObjectPostDeletedPayload, CoreObjectPreDeletedPayload,
        CoreObjectReferenceChangedPayload,
    };
    use wwise_waapi::api::generated::ak::wwise::core::profiler::captureLog::CoreProfilerCaptureLogItemAddedPayload;
    use wwise_waapi::api::generated::ak::wwise::core::profiler::{
        CoreProfilerGameObjectRegisteredPayload, CoreProfilerGameObjectResetPayload,
        CoreProfilerGameObjectUnregisteredPayload, CoreProfilerStateChangedPayload,
        CoreProfilerSwitchChangedPayload,
    };
    use wwise_waapi::api::generated::ak::wwise::core::project::{
        CoreProjectLoadedPayload, CoreProjectPostClosedPayload, CoreProjectPreClosedPayload,
        CoreProjectSavedPayload,
    };
    use wwise_waapi::api::generated::ak::wwise::core::soundbank::{
        CoreSoundbankGeneratedPayload, CoreSoundbankGenerationDonePayload,
    };
    use wwise_waapi::api::generated::ak::wwise::core::switchContainer::{
        CoreSwitchContainerAssignmentAddedPayload, CoreSwitchContainerAssignmentRemovedPayload,
    };
    use wwise_waapi::api::generated::ak::wwise::debug::DebugAssertFailedPayload;
    use wwise_waapi::api::generated::ak::wwise::ui::UiSelectionChangedPayload;
    use wwise_waapi::api::generated::ak::wwise::ui::commands::UiCommandsExecutedPayload;
    use wwise_waapi::api::generated::ak::wwise::ui::signal::{
        UiSignalClickPayload, UiSignalTogglePayload,
    };
    use wwise_waapi::api::generated::primitives::{
        AttenuationCurveType, GameObjectRef, LogChannel, LogSeverity,
    };

    // object.preDeleted / postDeleted
    let pre_deleted: CoreObjectPreDeletedPayload =
        serde_json::from_value(json!({"object": {"id": "{OBJ-1}"}})).unwrap();
    assert_eq!(pre_deleted.object.id.as_deref(), Some("{OBJ-1}"));
    let post_deleted: CoreObjectPostDeletedPayload =
        serde_json::from_value(json!({"object": {"id": "{OBJ-1}"}})).unwrap();
    assert_eq!(post_deleted.object.id.as_deref(), Some("{OBJ-1}"));

    // object.childAdded / childRemoved
    let child_added: CoreObjectChildAddedPayload = serde_json::from_value(json!({
        "parent": {"id": "{PARENT}"},
        "child": {"id": "{CHILD}"}
    }))
    .unwrap();
    assert_eq!(child_added.parent.id.as_deref(), Some("{PARENT}"));
    assert_eq!(child_added.child.id.as_deref(), Some("{CHILD}"));
    let child_removed: CoreObjectChildRemovedPayload = serde_json::from_value(json!({
        "parent": {"id": "{PARENT}"},
        "child": {"id": "{CHILD}"}
    }))
    .unwrap();
    assert_eq!(child_removed.parent.id.as_deref(), Some("{PARENT}"));
    assert_eq!(child_removed.child.id.as_deref(), Some("{CHILD}"));

    // object.notesChanged
    let notes: CoreObjectNotesChangedPayload = serde_json::from_value(json!({
        "object": {"id": "{OBJ-1}"},
        "newNotes": "new note",
        "oldNotes": "old note"
    }))
    .unwrap();
    assert_eq!(notes.new_notes, "new note");
    assert_eq!(notes.old_notes, "old note");

    // object.referenceChanged
    let ref_changed: CoreObjectReferenceChangedPayload = serde_json::from_value(json!({
        "object": {"id": "{OBJ-1}"},
        "reference": "OutputBus",
        "old": {"id": "{OLD}"},
        "new": {"id": "{NEW}"}
    }))
    .unwrap();
    assert_eq!(ref_changed.reference, "OutputBus");
    assert_eq!(ref_changed.old.id.as_deref(), Some("{OLD}"));
    assert_eq!(ref_changed.new.id.as_deref(), Some("{NEW}"));
    assert!(ref_changed.platform.is_none());

    // object.curveChanged
    let curve: CoreObjectCurveChangedPayload = serde_json::from_value(json!({
        "owner": {"id": "{OWNER}"},
        "notification": "curve updated",
        "curve": {"id": "{CURVE}"}
    }))
    .unwrap();
    assert_eq!(curve.owner.id.as_deref(), Some("{OWNER}"));
    assert_eq!(curve.notification, "curve updated");
    assert_eq!(curve.curve.id.as_deref(), Some("{CURVE}"));

    // object.attenuationCurveChanged / LinkChanged
    let atten_curve: CoreObjectAttenuationCurveChangedPayload = serde_json::from_value(json!({
        "curveType": "VolumeDryUsage",
        "attenuation": {"id": "{ATT}"}
    }))
    .unwrap();
    assert_eq!(atten_curve.curve_type, AttenuationCurveType::VolumeDryUsage);
    assert_eq!(atten_curve.attenuation.id.as_deref(), Some("{ATT}"));
    let atten_link: CoreObjectAttenuationCurveLinkChangedPayload = serde_json::from_value(json!({
        "curveType": "LowPassFilterUsage",
        "attenuation": {"id": "{ATT2}"}
    }))
    .unwrap();
    assert_eq!(
        atten_link.curve_type,
        AttenuationCurveType::LowPassFilterUsage
    );
    assert_eq!(atten_link.attenuation.id.as_deref(), Some("{ATT2}"));

    // switchContainer.assignmentAdded / Removed
    let assignment_added: CoreSwitchContainerAssignmentAddedPayload =
        serde_json::from_value(json!({
            "child": {"id": "{CHILD}"},
            "stateOrSwitch": {"id": "{SOS}"},
            "switchContainer": {"id": "{SC}"}
        }))
        .unwrap();
    assert_eq!(assignment_added.child.id.as_deref(), Some("{CHILD}"));
    assert_eq!(
        assignment_added.state_or_switch.id.as_deref(),
        Some("{SOS}")
    );
    assert_eq!(
        assignment_added.switch_container.id.as_deref(),
        Some("{SC}")
    );
    let assignment_removed: CoreSwitchContainerAssignmentRemovedPayload =
        serde_json::from_value(json!({
            "child": {"id": "{CHILD}"},
            "stateOrSwitch": {"id": "{SOS}"},
            "switchContainer": {"id": "{SC}"}
        }))
        .unwrap();
    assert_eq!(
        assignment_removed.switch_container.id.as_deref(),
        Some("{SC}")
    );

    // soundbank.generated
    let generated: CoreSoundbankGeneratedPayload = serde_json::from_value(json!({
        "platform": {"id": "{PLAT}"},
        "soundbank": {"id": "{SB}"},
        "language": {"id": "{LANG}"},
        "bankData": {"size": 10},
        "pluginInfo": {"version": 1},
        "error": null,
        "bankInfo": [{"id": "{BI}"}]
    }))
    .unwrap();
    assert_eq!(generated.platform, json!({"id": "{PLAT}"}));
    assert_eq!(generated.soundbank.id.as_deref(), Some("{SB}"));
    assert_eq!(generated.language, Some(json!({"id": "{LANG}"})));
    assert_eq!(generated.bank_data, Some(json!({"size": 10})));
    assert_eq!(generated.plugin_info, Some(json!({"version": 1})));
    assert!(generated.error.is_none());
    assert_eq!(generated.bank_info, vec![json!({"id": "{BI}"})]);

    // soundbank.generationDone
    let generation_done: CoreSoundbankGenerationDonePayload = serde_json::from_value(json!({
        "logs": [{
            "severity": "Message",
            "time": 123,
            "messageId": "M-1",
            "message": "hello",
            "platform": {"id": "{PLAT}"},
            "parameters": ["a", "b"]
        }],
        "error": null
    }))
    .unwrap();
    assert_eq!(generation_done.logs.len(), 1);
    assert_eq!(generation_done.logs[0].severity, LogSeverity::Message);
    assert_eq!(generation_done.logs[0].time, 123);
    assert_eq!(generation_done.logs[0].message_id, "M-1");
    assert!(generation_done.error.is_none());

    // project empty payloads
    let _pre_closed: CoreProjectPreClosedPayload = serde_json::from_value(json!({})).unwrap();
    let _loaded: CoreProjectLoadedPayload = serde_json::from_value(json!({})).unwrap();
    let _post_closed: CoreProjectPostClosedPayload = serde_json::from_value(json!({})).unwrap();

    // project.saved
    let saved: CoreProjectSavedPayload =
        serde_json::from_value(json!({"modifiedPaths": ["/a.wproj", "/b.wwu"]})).unwrap();
    assert_eq!(saved.modified_paths, vec!["/a.wproj", "/b.wwu"]);

    // audio.imported
    let imported: CoreAudioImportedPayload = serde_json::from_value(json!({
        "files": ["a.wav"],
        "operation": "createNew",
        "objects": [{"id": "{OBJ-1}"}]
    }))
    .unwrap();
    assert_eq!(imported.files, vec!["a.wav"]);
    assert_eq!(imported.operation, "createNew");
    assert_eq!(imported.objects[0].id.as_deref(), Some("{OBJ-1}"));

    // log.itemAdded
    let log_item: CoreLogItemAddedPayload = serde_json::from_value(json!({
        "channel": "soundbankGenerate",
        "item": {
            "severity": "Warning",
            "time": 5,
            "messageId": "M-2",
            "message": "warn"
        }
    }))
    .unwrap();
    assert_eq!(log_item.channel, LogChannel::SoundbankGenerate);
    assert_eq!(log_item.item.severity, LogSeverity::Warning);
    assert_eq!(log_item.item.message, "warn");

    // profiler.gameObjectRegistered / Unregistered
    let registered: CoreProfilerGameObjectRegisteredPayload = serde_json::from_value(json!({
        "time": 10,
        "gameObjectId": 1001,
        "gameObjectName": "Emitter"
    }))
    .unwrap();
    assert_eq!(registered.time, 10);
    assert!(matches!(registered.game_object_id, GameObjectRef::Id(1001)));
    assert_eq!(registered.game_object_name, "Emitter");
    let unregistered: CoreProfilerGameObjectUnregisteredPayload = serde_json::from_value(json!({
        "time": 11,
        "gameObjectId": 1002,
        "gameObjectName": "Listener"
    }))
    .unwrap();
    assert!(matches!(
        unregistered.game_object_id,
        GameObjectRef::Id(1002)
    ));

    // profiler.gameObjectReset (empty)
    let _reset: CoreProfilerGameObjectResetPayload = serde_json::from_value(json!({})).unwrap();

    // profiler.stateChanged
    let state_changed: CoreProfilerStateChangedPayload = serde_json::from_value(json!({
        "state": {"id": "{S1}"},
        "stateGroup": {"id": "{SG1}"}
    }))
    .unwrap();
    assert_eq!(state_changed.state.id.as_deref(), Some("{S1}"));
    assert_eq!(state_changed.state_group.id.as_deref(), Some("{SG1}"));

    // profiler.switchChanged
    let switch_changed: CoreProfilerSwitchChangedPayload = serde_json::from_value(json!({
        "gameObjectID": 42,
        "switchGroup": {"id": "{SG2}"},
        "switch": {"id": "{SW2}"}
    }))
    .unwrap();
    assert_eq!(switch_changed.game_object_id, 42);
    assert_eq!(switch_changed.switch_group.id.as_deref(), Some("{SG2}"));
    assert_eq!(switch_changed.switch.id.as_deref(), Some("{SW2}"));

    // profiler.captureLog.itemAdded
    let capture_log: CoreProfilerCaptureLogItemAddedPayload = serde_json::from_value(json!({
        "type": "StateChanged",
        "severity": "Normal",
        "objectName": "MyObj",
        "time": 20,
        "gameObjectId": 1001,
        "gameObjectName": "Emitter",
        "playingId": 7,
        "description": "desc",
        "objectId": "{OBJ-1}",
        "objectShortId": 9,
        "errorCodeName": "None"
    }))
    .unwrap();
    assert_eq!(capture_log.object_type, "StateChanged");
    assert_eq!(capture_log.severity, "Normal");
    assert_eq!(capture_log.object_name.as_deref(), Some("MyObj"));
    assert_eq!(capture_log.time, 20);
    assert!(matches!(
        capture_log.game_object_id,
        Some(GameObjectRef::Id(1001))
    ));
    assert_eq!(capture_log.game_object_name.as_deref(), Some("Emitter"));
    assert_eq!(capture_log.playing_id, Some(7));
    assert_eq!(capture_log.description, "desc");
    assert_eq!(capture_log.object_id.as_deref(), Some("{OBJ-1}"));
    assert_eq!(capture_log.object_short_id, Some(9));
    assert_eq!(capture_log.error_code_name.as_deref(), Some("None"));

    // debug.assertFailed
    let assert_failed: DebugAssertFailedPayload = serde_json::from_value(json!({
        "message": "boom",
        "fileName": "x.cpp",
        "expression": "a == b",
        "lineNumber": 12,
        "callstack": "stack"
    }))
    .unwrap();
    assert_eq!(assert_failed.message.as_deref(), Some("boom"));
    assert_eq!(assert_failed.file_name, "x.cpp");
    assert_eq!(assert_failed.expression, "a == b");
    assert_eq!(assert_failed.line_number, 12.0);
    assert_eq!(assert_failed.callstack, "stack");

    // ui.selectionChanged
    let selection: UiSelectionChangedPayload = serde_json::from_value(json!({
        "objects": [{"id": "{A}"}, {"id": "{B}"}]
    }))
    .unwrap();
    assert_eq!(selection.objects.len(), 2);
    assert_eq!(selection.objects[0].id.as_deref(), Some("{A}"));

    // ui.commands.executed
    let commands: UiCommandsExecutedPayload = serde_json::from_value(json!({
        "platforms": ["{GUID1}", "Mac"],
        "command": "Run",
        "objects": [{"id": "{C}"}]
    }))
    .unwrap();
    assert_eq!(commands.platforms.len(), 2);
    assert!(
        matches!(&commands.platforms[0], wwise_waapi::api::PlatformRef::Guid(s) if s == "{GUID1}")
    );
    assert!(matches!(&commands.platforms[1], wwise_waapi::api::PlatformRef::Name(s) if s == "Mac"));
    assert_eq!(commands.command, "Run");
    assert_eq!(commands.objects[0].id.as_deref(), Some("{C}"));

    // ui.signal.click / toggle
    let click: UiSignalClickPayload =
        serde_json::from_value(json!({"handleID": "H1", "widgetID": "W1"})).unwrap();
    assert_eq!(click.handle_id, "H1");
    assert_eq!(click.widget_id, "W1");
    let toggle: UiSignalTogglePayload =
        serde_json::from_value(json!({"handleID": "H2", "widgetID": "W2"})).unwrap();
    assert_eq!(toggle.handle_id, "H2");
    assert_eq!(toggle.widget_id, "W2");
}

#[test]
fn object_type_union_preserves_renamed_variants() {
    use wwise_waapi::ObjectType;

    // ActorMixer (Wwise 2024) and PropertyContainer (Wwise 2025) are the same
    // object type; the union keeps both names as distinct variants.
    assert_eq!(ObjectType::ActorMixer.as_str(), "ActorMixer");
    assert_eq!(ObjectType::PropertyContainer.as_str(), "PropertyContainer");
    assert_ne!(ObjectType::ActorMixer, ObjectType::PropertyContainer);

    // Both share the Wwise class ID (0x80010 = 524304).
    assert_eq!(ObjectType::ActorMixer.class_id(), 524304);
    assert_eq!(
        ObjectType::ActorMixer.class_id(),
        ObjectType::PropertyContainer.class_id()
    );

    // Reverse lookup resolves to the canonical (latest Wwise) name.
    assert_eq!(
        ObjectType::from_class_id(524304),
        Some(ObjectType::PropertyContainer)
    );

    // Both names still parse/deserialize independently.
    assert_eq!(
        "ActorMixer".parse::<ObjectType>().unwrap(),
        ObjectType::ActorMixer
    );
    assert_eq!(
        "PropertyContainer".parse::<ObjectType>().unwrap(),
        ObjectType::PropertyContainer
    );
}

#[test]
fn return_field_union_keeps_removed_variants() {
    use wwise_waapi::api::generated::primitives::{
        AudioObjectReturnField, ObjectReturnField, WwiseObject,
    };

    // Fields removed in Wwise 2025 but retained for 2024 compatibility.
    assert_eq!(
        serde_json::to_value(ObjectReturnField::TransitionObject).unwrap(),
        json!("transitionObject")
    );
    assert_eq!(
        serde_json::to_value(ObjectReturnField::ColorRgb).unwrap(),
        json!("colorRGB")
    );
    assert_eq!(
        serde_json::to_value(AudioObjectReturnField::PrevGain).unwrap(),
        json!("prevGain")
    );
    assert_eq!(
        serde_json::to_value(AudioObjectReturnField::NextGain).unwrap(),
        json!("nextGain")
    );
    let obj: WwiseObject =
        serde_json::from_value(json!({"id": "{OBJ-1}", "transitionObject": {"id": "{TRANS-1}"}}))
            .unwrap();
    assert_eq!(obj.transition_object, Some(json!({"id": "{TRANS-1}"})));
}

#[test]
fn new_2025_endpoints_roundtrip() {
    use wwise_waapi::api::generated::ak::wwise::core::mediaPool::{
        CoreMediaPoolGetArgs, CoreMediaPoolGetFieldsResponse, CoreMediaPoolGetOptions,
        CoreMediaPoolGetResponse,
    };
    use wwise_waapi::api::generated::ak::wwise::core::object::CoreObjectStructureChangedPayload;
    use wwise_waapi::api::generated::ak::wwise::core::profiler::captureLog::CoreProfilerCaptureLogSaveArgs;
    use wwise_waapi::api::generated::ak::wwise::core::profiler::{
        CoreProfilerMoveCursorArgs, CoreProfilerMoveCursorResponse, CoreProfilerSetCursorTimeArgs,
        CoreProfilerSetCursorTimeResponse,
    };
    use wwise_waapi::api::generated::ak::wwise::core::workUnit::{
        CoreWorkUnitLoadArgs, CoreWorkUnitUnloadArgs,
    };
    use wwise_waapi::api::generated::ak::wwise::ui::UiGetSelectedFilesResponse;
    use wwise_waapi::api::generated::ak::wwise::ui::layout::{
        UiLayoutCloseViewArgs, UiLayoutResetLayoutsArgs,
    };
    use wwise_waapi::api::primitives::ObjectRef;

    // mediaPool.get: args, options, and wrapped "return" response.
    let media_get = CoreMediaPoolGetArgs::new()
        .databases(vec![ObjectRef::guid("{DB-1}")])
        .search_text("footsteps")
        .max_results(10);
    assert_eq!(
        serde_json::to_value(media_get).unwrap(),
        json!({
            "databases": ["{DB-1}"],
            "searchText": "footsteps",
            "maxResults": 10
        })
    );
    let media_opts = CoreMediaPoolGetOptions::new()
        .return_(vec!["Path".to_string(), "WAV/Duration".to_string()]);
    assert_eq!(
        serde_json::to_value(media_opts).unwrap(),
        json!({"return": ["Path", "WAV/Duration"]})
    );
    let media_resp: serde_json::Value = json!({"return": [{"Path": "a.wav"}]});
    let parsed: CoreMediaPoolGetResponse = serde_json::from_value(media_resp).unwrap();
    assert_eq!(parsed.return_, vec![json!({"Path": "a.wav"})]);

    // mediaPool.getFields: no args, wrapped "return" string list.
    let fields: CoreMediaPoolGetFieldsResponse =
        serde_json::from_value(json!({"return": ["Path", "FileId"]})).unwrap();
    assert_eq!(fields.return_, vec!["Path", "FileId"]);

    // profiler.captureLog.save
    let save = CoreProfilerCaptureLogSaveArgs::new("/tmp/capture.log".to_string());
    assert_eq!(
        serde_json::to_value(save).unwrap(),
        json!({"file": "/tmp/capture.log"})
    );

    // profiler.moveCursor
    let move_cursor = CoreProfilerMoveCursorArgs::new("next".to_string());
    assert_eq!(
        serde_json::to_value(move_cursor).unwrap(),
        json!({"position": "next"})
    );
    let move_resp: CoreProfilerMoveCursorResponse =
        serde_json::from_value(json!({"return": 500})).unwrap();
    assert_eq!(move_resp.return_, 500);

    // profiler.setCursorTime
    let set_time = CoreProfilerSetCursorTimeArgs::new(1234);
    assert_eq!(
        serde_json::to_value(set_time).unwrap(),
        json!({"time": 1234})
    );
    let set_resp: CoreProfilerSetCursorTimeResponse =
        serde_json::from_value(json!({"return": 1234})).unwrap();
    assert_eq!(set_resp.return_, 1234);

    // workUnit.load / unload
    let load = CoreWorkUnitLoadArgs::new(ObjectRef::guid("{WU-1}"));
    assert_eq!(
        serde_json::to_value(load).unwrap(),
        json!({"object": "{WU-1}"})
    );
    let unload = CoreWorkUnitUnloadArgs::new(ObjectRef::guid("{WU-1}"));
    assert_eq!(
        serde_json::to_value(unload).unwrap(),
        json!({"object": "{WU-1}"})
    );

    // ui.getSelectedFiles: no args, plain "files" response.
    let files: UiGetSelectedFilesResponse =
        serde_json::from_value(json!({"files": ["/a.wav", "/b.wav"]})).unwrap();
    assert_eq!(files.files, vec!["/a.wav", "/b.wav"]);

    // ui.layout.closeView
    let close = UiLayoutCloseViewArgs::new("MyView".to_string());
    assert_eq!(
        serde_json::to_value(close).unwrap(),
        json!({"viewID": "MyView"})
    );

    // ui.layout.resetLayouts
    let reset = UiLayoutResetLayoutsArgs::new().layouts(vec!["Layout1".to_string()]);
    assert_eq!(
        serde_json::to_value(reset).unwrap(),
        json!({"layouts": ["Layout1"]})
    );

    // object.structureChanged topic payload
    let structure: CoreObjectStructureChangedPayload = serde_json::from_value(json!({
        "objects": [{"object": {"id": "{OBJ-1}"}, "changes": []}],
        "summary": "Paste"
    }))
    .unwrap();
    assert_eq!(structure.summary, "Paste");
    assert_eq!(structure.objects.len(), 1);
}

#[test]
fn wwise_object_convenience_accessors_roundtrip() {
    use wwise_waapi::api::generated::primitives::WwiseObject;

    let obj: WwiseObject = serde_json::from_value(json!({
        "id": "{12345678-1234-1234-1234-123456789012}",
        "name": "PlayerFootstep",
        "type": "Sound",
        "path": r"\Containers\Default Work Unit\PlayerFootstep",
        "notes": "Footstep SFX",
        "@Volume": -3.5
    }))
    .unwrap();

    assert_eq!(obj.id(), Some("{12345678-1234-1234-1234-123456789012}"));
    assert_eq!(obj.name(), Some("PlayerFootstep"));
    assert_eq!(obj.object_type(), Some(ObjectType::Sound));
    assert_eq!(
        obj.path(),
        Some(r"\Containers\Default Work Unit\PlayerFootstep")
    );
    assert_eq!(obj.notes(), Some("Footstep SFX"));
    assert_eq!(obj.property(Property::Volume), Some(&json!(-3.5)));
}

#[test]
fn schema_discovered_endpoints_roundtrip() {
    use wwise_waapi::api::generated::ak::wwise::core::audioSourcePeaks::{
        CoreAudioSourcePeaksGetMinMaxPeaksInRegionArgs,
        CoreAudioSourcePeaksGetMinMaxPeaksInRegionResponse,
    };
    use wwise_waapi::api::generated::ak::wwise::core::log::{
        CoreLogClearArgs, CoreLogGetArgs, CoreLogGetResponse,
    };
    use wwise_waapi::api::generated::ak::wwise::core::profiler::{
        CoreProfilerGetAudioObjectsArgs, CoreProfilerGetAudioObjectsOptions,
        CoreProfilerGetBussesArgs, CoreProfilerGetBussesOptions, CoreProfilerGetVoicesArgs,
        CoreProfilerGetVoicesOptions,
    };
    use wwise_waapi::api::generated::ak::wwise::core::remote::{
        CoreRemoteGetAvailableConsolesResponse, CoreRemoteGetConnectionStatusResponse,
    };
    use wwise_waapi::api::generated::ak::wwise::debug::DebugEnableAssertsArgs;
    use wwise_waapi::api::generated::ak::wwise::waapi::{
        WaapiGetFunctionsResponse, WaapiGetSchemaArgs, WaapiGetTopicsResponse,
    };
    use wwise_waapi::api::generated::primitives::{
        AudioObjectReturnField, BusReturnField, LogChannel, VoiceReturnField,
    };

    // waapi meta endpoints
    let funcs: WaapiGetFunctionsResponse =
        serde_json::from_value(json!({"functions": ["ak.wwise.core.getInfo"]})).unwrap();
    assert_eq!(funcs.functions, vec!["ak.wwise.core.getInfo"]);

    let topics: WaapiGetTopicsResponse =
        serde_json::from_value(json!({"topics": ["ak.wwise.core.object.created"]})).unwrap();
    assert_eq!(topics.topics, vec!["ak.wwise.core.object.created"]);

    let get_schema = WaapiGetSchemaArgs::new("ak.wwise.core.getInfo".to_string());
    assert_eq!(
        serde_json::to_value(get_schema).unwrap(),
        json!({"uri": "ak.wwise.core.getInfo"})
    );

    // log.get & log.clear
    let log_get = CoreLogGetArgs::new(LogChannel::General);
    assert_eq!(
        serde_json::to_value(log_get).unwrap(),
        json!({"channel": "general"})
    );
    let log_clear = CoreLogClearArgs::new().channel(LogChannel::Waapi);
    assert_eq!(
        serde_json::to_value(log_clear).unwrap(),
        json!({"channel": "waapi"})
    );
    let log_resp: CoreLogGetResponse = serde_json::from_value(json!({"items": []})).unwrap();
    assert!(log_resp.items.is_empty());

    // remote.getAvailableConsoles & getConnectionStatus
    let consoles: CoreRemoteGetAvailableConsolesResponse =
        serde_json::from_value(json!({"consoles": [{"name": "Local", "host": "127.0.0.1"}]}))
            .unwrap();
    assert_eq!(consoles.consoles.len(), 1);
    let conn_status: CoreRemoteGetConnectionStatusResponse =
        serde_json::from_value(json!({"isConnected": false, "status": "Disconnected"})).unwrap();
    assert!(!conn_status.is_connected);

    // profiler getBusses / getVoices / getAudioObjects with typed return field enums
    let busses_args = CoreProfilerGetBussesArgs::new(json!("user")).bus_pipeline_id(42u32);
    assert_eq!(
        serde_json::to_value(busses_args).unwrap(),
        json!({"time": "user", "busPipelineID": 42})
    );
    let busses_opts = CoreProfilerGetBussesOptions::new()
        .return_(vec![BusReturnField::ObjectGuid, BusReturnField::ObjectName]);
    assert_eq!(
        serde_json::to_value(busses_opts).unwrap(),
        json!({"return": ["objectGUID", "objectName"]})
    );

    let voices_args = CoreProfilerGetVoicesArgs::new(json!(1000));
    assert_eq!(
        serde_json::to_value(voices_args).unwrap(),
        json!({"time": 1000})
    );
    let voices_opts = CoreProfilerGetVoicesOptions::new().return_(vec![
        VoiceReturnField::ObjectGuid,
        VoiceReturnField::GameObjectName,
    ]);
    assert_eq!(
        serde_json::to_value(voices_opts).unwrap(),
        json!({"return": ["objectGUID", "gameObjectName"]})
    );

    let audio_objs_args = CoreProfilerGetAudioObjectsArgs::new(json!("capture"));
    assert_eq!(
        serde_json::to_value(audio_objs_args).unwrap(),
        json!({"time": "capture"})
    );
    let audio_objs_opts = CoreProfilerGetAudioObjectsOptions::new().return_(vec![
        AudioObjectReturnField::AudioObjectName,
        AudioObjectReturnField::InstigatorPipelineId,
    ]);
    assert_eq!(
        serde_json::to_value(audio_objs_opts).unwrap(),
        json!({"return": ["audioObjectName", "instigatorPipelineID"]})
    );

    // audioSourcePeaks.getMinMaxPeaksInRegion
    let peaks_args = CoreAudioSourcePeaksGetMinMaxPeaksInRegionArgs::new(
        ObjectRef::guid("{SRC-1}"),
        0.0,
        1.5,
        10,
    )
    .get_cross_channel_peaks(true);
    assert_eq!(
        serde_json::to_value(peaks_args).unwrap(),
        json!({
            "numPeaks": 10,
            "object": "{SRC-1}",
            "timeFrom": 0.0,
            "timeTo": 1.5,
            "getCrossChannelPeaks": true
        })
    );
    let peaks_resp: CoreAudioSourcePeaksGetMinMaxPeaksInRegionResponse =
        serde_json::from_value(json!({
            "peaks": ["AAAA"],
            "maxAbsValue": 0.85,
            "numChannels": 2.0
        }))
        .unwrap();
    assert_eq!(peaks_resp.num_channels, Some(2.0));

    // debug.enableAsserts
    let asserts = DebugEnableAssertsArgs::new(true);
    assert_eq!(
        serde_json::to_value(asserts).unwrap(),
        json!({"enable": true})
    );
}

#[test]
fn ergonomic_constructors_aliases_and_from_roundtrip() {
    use wwise_waapi::api::ak::soundengine::{
        PostEventArgs, RegisterGameObjArgs, SetPositionArgs, SetRtpcValueArgs,
        UnregisterGameObjArgs,
    };
    use wwise_waapi::api::ak::wwise::core::object::{
        CreateArgs, CreatedOptions, CreatedPayload, DeleteArgs, GetArgs, GetOptions, SetArgs,
        SetItem, SetNameArgs, SetPropertyArgs,
    };
    use wwise_waapi::prelude::*;

    // 1. Short aliases + impl Into<T> in Args::new(...)
    let parent_guid = String::from("{11111111-1111-1111-1111-111111111111}");
    let create = CreateArgs::new(&parent_guid, ObjectType::Sound, "Laser");
    assert_eq!(
        serde_json::to_value(create).unwrap(),
        json!({
            "parent": "{11111111-1111-1111-1111-111111111111}",
            "type": "Sound",
            "name": "Laser"
        })
    );

    // 3-tuple From for CreateArgs
    let create_tuple = CreateArgs::from((&parent_guid, ObjectType::Sound, "Plasma"));
    assert_eq!(
        serde_json::to_value(create_tuple).unwrap(),
        json!({
            "parent": "{11111111-1111-1111-1111-111111111111}",
            "type": "Sound",
            "name": "Plasma"
        })
    );

    // 1-required-field From for DeleteArgs (&str and &String)
    let delete_str = DeleteArgs::from(r"\Containers\Default Work Unit\Laser");
    assert_eq!(
        serde_json::to_value(delete_str).unwrap(),
        json!({"object": r"\Containers\Default Work Unit\Laser"})
    );
    let delete_ref = DeleteArgs::from(&parent_guid);
    assert_eq!(
        serde_json::to_value(delete_ref).unwrap(),
        json!({"object": "{11111111-1111-1111-1111-111111111111}"})
    );

    // 2-tuple From for SetNameArgs & 3-tuple From for SetPropertyArgs
    let rename = SetNameArgs::from((&parent_guid, "LaserRenamed"));
    assert_eq!(
        serde_json::to_value(rename).unwrap(),
        json!({
            "object": "{11111111-1111-1111-1111-111111111111}",
            "value": "LaserRenamed"
        })
    );
    let set_prop = SetPropertyArgs::from((&parent_guid, Property::Volume, -4.5));
    assert_eq!(
        serde_json::to_value(set_prop).unwrap(),
        json!({
            "object": "{11111111-1111-1111-1111-111111111111}",
            "property": "Volume",
            "value": -4.5
        })
    );

    // GetArgs From<&str>, From<&String>, From<ObjectType>, and GetArgs::new(impl Into<ObjectQuery>)
    let get_path: GetArgs = r"\Containers\Default Work Unit".into();
    assert_eq!(
        serde_json::to_value(get_path).unwrap(),
        json!({"from": {"path": [r"\Containers\Default Work Unit"]}})
    );
    let get_id = GetArgs::new(&parent_guid);
    assert_eq!(
        serde_json::to_value(get_id).unwrap(),
        json!({"from": {"id": ["{11111111-1111-1111-1111-111111111111}"]}})
    );
    let get_type: GetArgs = ObjectType::Sound.into();
    assert_eq!(
        serde_json::to_value(get_type).unwrap(),
        json!({"from": {"ofType": ["Sound"]}})
    );
    let _opts = GetOptions::new().return_fields([ObjectReturnField::Id, ObjectReturnField::Name]);

    // SetArgs::from(SetItem) and SetArgs::item(SetItem)
    let item = SetItem::new(&parent_guid)
        .name("LaserV2")
        .property(Property::Volume, -6.0);
    let set_from_item = SetArgs::from(item);
    assert_eq!(
        serde_json::to_value(set_from_item).unwrap(),
        json!({
            "objects": [{
                "object": "{11111111-1111-1111-1111-111111111111}",
                "name": "LaserV2",
                "@Volume": -6.0
            }]
        })
    );

    // SoundEngine ergonomic constructors & From impls
    let reg = RegisterGameObjArgs::from((1001u64, "Listener"));
    assert_eq!(
        serde_json::to_value(reg).unwrap(),
        json!({"gameObject": 1001, "name": "Listener"})
    );
    let unreg = UnregisterGameObjArgs::from(1001u64);
    assert_eq!(
        serde_json::to_value(unreg).unwrap(),
        json!({"gameObject": 1001})
    );
    let pos = SetPositionArgs::from((1001u64, Position::origin()));
    assert_eq!(
        serde_json::to_value(pos).unwrap()["gameObject"],
        json!(1001)
    );
    let post = PostEventArgs::from("Play_Laser").game_object(1001u64);
    assert_eq!(
        serde_json::to_value(post).unwrap(),
        json!({"event": "Play_Laser", "gameObject": 1001})
    );
    let rtpc = SetRtpcValueArgs::from(("PlayerSpeed", 75.0));
    assert_eq!(
        serde_json::to_value(rtpc).unwrap(),
        json!({"rtpc": "PlayerSpeed", "value": 75.0})
    );

    // Short topic aliases
    let _created_opts = CreatedOptions::new().return_fields([ObjectReturnField::Id]);
    let created_payload: CreatedPayload =
        serde_json::from_value(json!({"object": {"id": parent_guid}})).unwrap();
    assert_eq!(
        created_payload.object.id(),
        Some("{11111111-1111-1111-1111-111111111111}")
    );
}

#[test]
fn unified_return_fields_options_accessors_and_errors_roundtrip() {
    use wwise_waapi::api::ak::wwise::core::mediaPool::CoreMediaPoolGetOptions;
    use wwise_waapi::api::ak::wwise::core::object::{
        CoreObjectCreateResponse, CoreObjectSetObject, CreatedOptions, GetOptions,
        PropertyChangedOptions,
    };
    use wwise_waapi::api::ak::wwise::core::profiler::{
        CoreProfilerGetAudioObjectsOptions, CoreProfilerGetBussesOptions,
        CoreProfilerGetVoicesOptions,
    };
    use wwise_waapi::api::generated::primitives::{
        AudioObjectReturnField, BusReturnField, VoiceReturnField,
    };
    use wwise_waapi::prelude::*;

    // 1. Unified .return_fields(...) accepting arrays across all Options structs
    let created_opts =
        CreatedOptions::new().return_fields([ObjectReturnField::Id, ObjectReturnField::Name]);
    assert_eq!(
        serde_json::to_value(&created_opts).unwrap(),
        json!({"return": ["id", "name"]})
    );
    let created_from_arr = CreatedOptions::from([ObjectReturnField::Id, ObjectReturnField::Type]);
    assert_eq!(
        serde_json::to_value(created_from_arr).unwrap(),
        json!({"return": ["id", "type"]})
    );
    let created_from_unit = CreatedOptions::from(());
    assert_eq!(serde_json::to_value(created_from_unit).unwrap(), json!({}));

    let prop_changed_opts = PropertyChangedOptions::new("{OBJ-1}", Property::Volume)
        .return_fields([ObjectReturnField::Id, "path".into()]);
    assert_eq!(
        serde_json::to_value(prop_changed_opts).unwrap(),
        json!({"object": "{OBJ-1}", "property": "Volume", "return": ["id", "path"]})
    );

    let busses_opts = CoreProfilerGetBussesOptions::new()
        .return_fields([BusReturnField::ObjectGuid, BusReturnField::ObjectName]);
    assert_eq!(
        serde_json::to_value(busses_opts).unwrap(),
        json!({"return": ["objectGUID", "objectName"]})
    );
    let voices_opts = CoreProfilerGetVoicesOptions::from([
        VoiceReturnField::ObjectGuid,
        VoiceReturnField::GameObjectName,
    ]);
    assert_eq!(
        serde_json::to_value(voices_opts).unwrap(),
        json!({"return": ["objectGUID", "gameObjectName"]})
    );
    let audio_objs_opts = CoreProfilerGetAudioObjectsOptions::new()
        .return_fields([AudioObjectReturnField::AudioObjectName]);
    assert_eq!(
        serde_json::to_value(audio_objs_opts).unwrap(),
        json!({"return": ["audioObjectName"]})
    );
    let media_opts = CoreMediaPoolGetOptions::new().return_fields(["Path", "WAV/Duration"]);
    assert_eq!(
        serde_json::to_value(media_opts).unwrap(),
        json!({"return": ["Path", "WAV/Duration"]})
    );

    // 2. ObjectReturnOptions::basic() and From conversions
    let basic = ObjectReturnOptions::basic();
    assert_eq!(
        serde_json::to_value(basic).unwrap(),
        json!({"return": ["id", "name", "type", "path"]})
    );
    let get_from_unit = GetOptions::from(());
    assert_eq!(serde_json::to_value(get_from_unit).unwrap(), json!({}));
    let get_from_arr = GetOptions::from([ObjectReturnField::Id, ObjectReturnField::Name]);
    assert_eq!(
        serde_json::to_value(get_from_arr).unwrap(),
        json!({"return": ["id", "name"]})
    );
    let get_from_single = GetOptions::from(ObjectReturnField::Id);
    assert_eq!(
        serde_json::to_value(get_from_single).unwrap(),
        json!({"return": ["id"]})
    );

    // 3. Consistent .id(), .name(), and typed property_* accessors on responses
    let obj: WwiseObject = serde_json::from_value(json!({
        "id": "{OBJ-1}",
        "name": "Explosion",
        "type": "Sound",
        "@Volume": -4.5,
        "@Pitch": 120,
        "@IsLoopingEnabled": true,
        "@UserAuxSendVolume0": "custom"
    }))
    .unwrap();
    assert_eq!(obj.id(), Some("{OBJ-1}"));
    assert_eq!(obj.name(), Some("Explosion"));
    assert_eq!(obj.property_f64(Property::Volume), Some(-4.5));
    assert_eq!(obj.property_i64(Property::Pitch), Some(120));
    assert_eq!(obj.property_bool(Property::IsLoopingEnabled), Some(true));
    assert_eq!(
        obj.property_str(Property::UserAuxSendVolume0),
        Some("custom")
    );

    let created_resp: CoreObjectCreateResponse = serde_json::from_value(json!({
        "id": "{CREATED-1}",
        "name": "NewSound",
        "children": [{"id": "{CHILD-1}", "name": "ChildSound"}]
    }))
    .unwrap();
    assert_eq!(created_resp.id(), Some("{CREATED-1}"));
    assert_eq!(created_resp.name(), Some("NewSound"));
    assert_eq!(created_resp.children[0].id(), Some("{CHILD-1}"));
    assert_eq!(created_resp.children[0].name(), Some("ChildSound"));

    let set_obj: CoreObjectSetObject = serde_json::from_value(json!({
        "id": "{SET-1}",
        "name": "ModifiedSound",
        "@Volume": -9.0
    }))
    .unwrap();
    assert_eq!(set_obj.id(), Some("{SET-1}"));
    assert_eq!(set_obj.name(), Some("ModifiedSound"));
    assert_eq!(set_obj.property_f64(Property::Volume), Some(-9.0));

    // 4. Compile-time check that WaapiClient & namespace clients take &self and are Copy
    fn _assert_immutable_and_copy(client: &WaapiClient) {
        let wwise = client.wwise();
        let wwise_copy = wwise;
        let obj1 = wwise.core().object();
        let obj2 = wwise_copy.core().object();
        let se = client.soundengine();
        let ui = client.ui();
        let core = client.core();
        let _ = (obj1, obj2, se, ui, core);
    }

    // 5. Structured WaapiCallError and WaapiError helpers
    let call_err = WaapiCallError::new(
        "ak.wwise.invalid_arguments",
        vec![],
        json!({
            "message": "'parent' already has child with same name.",
            "details": {"argumentName": "name"}
        }),
    );
    assert_eq!(call_err.uri(), "ak.wwise.invalid_arguments");
    assert_eq!(
        call_err.message(),
        Some("'parent' already has child with same name.")
    );
    assert_eq!(call_err.details(), &json!({"argumentName": "name"}));
    let waapi_err = WaapiError::Call(call_err);
    assert_eq!(waapi_err.error_uri(), Some("ak.wwise.invalid_arguments"));
    assert_eq!(
        waapi_err.waapi_message(),
        Some("'parent' already has child with same name.")
    );
    assert!(waapi_err.as_call_error().is_some());
    assert!(
        waapi_err
            .to_string()
            .contains("ak.wwise.invalid_arguments: 'parent' already has child with same name.")
    );
    assert_eq!(
        WaapiError::ClientShutdown.to_string(),
        "connection error: client already shut down"
    );
}

#[test]
fn into_constructor_helpers_roundtrip() {
    use wwise_waapi::api::{
        BlendEdge, BlendFadeMode, CurveShape, LanguageRef, MultiPosition, SoundBankInclusionType,
        SoundBankInfo, SwitchContainerAssignment,
    };
    use wwise_waapi::prelude::*;

    // SoundEngineObjectRef::name / guid / short / short_id
    assert_eq!(
        serde_json::to_value(SoundEngineObjectRef::name("Play_TestSound")).unwrap(),
        json!("Play_TestSound")
    );
    assert_eq!(
        serde_json::to_value(SoundEngineObjectRef::guid(
            "{F546017D-201A-49BD-8D4E-0A28F5DBB28D}"
        ))
        .unwrap(),
        json!("{F546017D-201A-49BD-8D4E-0A28F5DBB28D}")
    );
    assert_eq!(
        serde_json::to_value(SoundEngineObjectRef::short(2932040671)).unwrap(),
        json!(2932040671u32)
    );
    assert_eq!(
        serde_json::to_value(SoundEngineObjectRef::short_id(42)).unwrap(),
        json!(42)
    );

    // LanguageRef::name / guid
    assert_eq!(
        serde_json::to_value(LanguageRef::name("English(US)")).unwrap(),
        json!("English(US)")
    );
    assert_eq!(
        serde_json::to_value(LanguageRef::guid("{LANG-1}")).unwrap(),
        json!("{LANG-1}")
    );

    // GameObjectRef::new / id / game_object
    assert_eq!(
        serde_json::to_value(GameObjectRef::new(1001)).unwrap(),
        json!(1001)
    );
    assert_eq!(
        serde_json::to_value(GameObjectRef::id(1002)).unwrap(),
        json!(1002)
    );
    assert_eq!(
        serde_json::to_value(GameObjectRef::game_object(1003)).unwrap(),
        json!(1003)
    );

    // ObjectQuery constructor helpers
    assert_eq!(
        serde_json::to_value(ObjectQuery::id("{ID-1}")).unwrap(),
        json!({"id": ["{ID-1}"]})
    );
    assert_eq!(
        serde_json::to_value(ObjectQuery::ids(["{ID-1}", "{ID-2}"])).unwrap(),
        json!({"id": ["{ID-1}", "{ID-2}"]})
    );
    assert_eq!(
        serde_json::to_value(ObjectQuery::path(r"\Containers\Default Work Unit")).unwrap(),
        json!({"path": [r"\Containers\Default Work Unit"]})
    );
    assert_eq!(
        serde_json::to_value(ObjectQuery::paths([r"\Containers", r"\Busses"])).unwrap(),
        json!({"path": [r"\Containers", r"\Busses"]})
    );
    assert_eq!(
        serde_json::to_value(ObjectQuery::name("Sound:Laser")).unwrap(),
        json!({"name": ["Sound:Laser"]})
    );
    assert_eq!(
        serde_json::to_value(ObjectQuery::names(["Sound:Laser", "Event:Play_Laser"])).unwrap(),
        json!({"name": ["Sound:Laser", "Event:Play_Laser"]})
    );
    assert_eq!(
        serde_json::to_value(ObjectQuery::search("Laser")).unwrap(),
        json!({"search": ["Laser"]})
    );
    assert_eq!(
        serde_json::to_value(ObjectQuery::of_type(ObjectType::Sound)).unwrap(),
        json!({"ofType": ["Sound"]})
    );
    assert_eq!(
        serde_json::to_value(ObjectQuery::of_types([
            ObjectType::Sound,
            ObjectType::Event
        ]))
        .unwrap(),
        json!({"ofType": ["Sound", "Event"]})
    );
    assert_eq!(
        serde_json::to_value(ObjectQuery::query("{QUERY-1}")).unwrap(),
        json!({"query": ["{QUERY-1}"]})
    );

    // ObjectReturnField::property / custom
    assert_eq!(
        serde_json::to_value(ObjectReturnField::property(Property::Volume)).unwrap(),
        json!("@Volume")
    );
    assert_eq!(
        serde_json::to_value(ObjectReturnField::custom("parent.name")).unwrap(),
        json!("parent.name")
    );

    // PropertyValue constructor helpers
    assert_eq!(
        serde_json::to_value(PropertyValue::number(-6.0)).unwrap(),
        json!(-6.0)
    );
    assert_eq!(
        serde_json::to_value(PropertyValue::boolean(true)).unwrap(),
        json!(true)
    );
    assert_eq!(
        serde_json::to_value(PropertyValue::string("hello")).unwrap(),
        json!("hello")
    );
    assert_eq!(
        serde_json::to_value(PropertyValue::string_list(["a", "b"])).unwrap(),
        json!(["a", "b"])
    );
    let child = ChildObject::new(ObjectType::Sound, "ChildSound");
    assert_eq!(
        serde_json::to_value(PropertyValue::object(child.clone())).unwrap(),
        json!({"type": "Sound", "name": "ChildSound"})
    );
    assert_eq!(
        serde_json::to_value(PropertyValue::object_list([child])).unwrap(),
        json!([{"type": "Sound", "name": "ChildSound"}])
    );

    // SwitchContainerAssignment::new, BlendEdge::new, MultiPosition::new, SoundBankInfo builders
    let assign = SwitchContainerAssignment::new("{CHILD-1}", "{SWITCH-1}");
    assert_eq!(
        serde_json::to_value(assign).unwrap(),
        json!({"child": "{CHILD-1}", "stateOrSwitch": "{SWITCH-1}"})
    );
    let edge =
        BlendEdge::new(BlendFadeMode::Automatic, CurveShape::Linear, 0.25).fade_position(0.5);
    assert_eq!(
        serde_json::to_value(edge).unwrap(),
        json!({
            "fadeMode": "Automatic",
            "fadeShape": "Linear",
            "edgePosition": 0.25,
            "fadePosition": 0.5
        })
    );
    let mp = MultiPosition::new(Vector3::new(1.0, 2.0, 3.0));
    assert_eq!(
        serde_json::to_value(mp).unwrap()["position"]["position"],
        json!({"x": 1.0, "y": 2.0, "z": 3.0})
    );
    let bank = SoundBankInfo::new("MainBank")
        .inclusions([SoundBankInclusionType::Event, SoundBankInclusionType::Media])
        .events(["Play_TestSound"])
        .aux_busses([r"\Busses\Default Work Unit\Main Audio Bus"]);
    assert_eq!(
        serde_json::to_value(bank).unwrap(),
        json!({
            "name": "MainBank",
            "inclusions": ["event", "media"],
            "events": ["Play_TestSound"],
            "auxBusses": [r"\Busses\Default Work Unit\Main Audio Bus"]
        })
    );
}
