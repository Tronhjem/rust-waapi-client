use super::{connect_client, sequence_lock};
use wwise_waapi::api::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectDeleteArgs, CoreObjectGetPropertyAndReferenceNamesArgs,
    CoreObjectGetPropertyInfoArgs, CoreObjectIsLinkedArgs, CoreObjectIsPropertyEnabledArgs,
    CoreObjectSetLinkedArgs, CoreObjectSetStateGroupsArgs, CoreObjectSetStatePropertiesArgs,
};
use wwise_waapi::api::{ObjectRef, PlatformRef};
use wwise_waapi::{ObjectType, Property};

const ACTOR_MIXER_WU: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";
const STATE_GROUPS_WU: &str = r"\States\Default Work Unit";

#[tokio::test]
async fn object_introspection_sequence() {
    let _guard = sequence_lock().await;
    let client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    let project_info = client
        .wwise()
        .core()
        .get_project_info()
        .await
        .expect("get project info");
    let platform = PlatformRef::guid(project_info.current_platform_id.clone());

    // 1. getTypes lists the known Wwise object types.
    let types = client
        .wwise()
        .core()
        .object()
        .get_types()
        .await
        .expect("get types");
    assert!(
        types.types.iter().any(|t| t.name == "Sound"),
        "getTypes should include a Sound type"
    );

    // 2. Create an Actor-Mixer to introspect.
    let mixer = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(ACTOR_MIXER_WU),
                ObjectType::ActorMixer,
                "WaapiSeqIntrospectMixer".to_string(),
            )
            .on_name_conflict(wwise_waapi::api::OnNameConflict::Replace),
        )
        .await
        .expect("create mixer");
    let mixer_id = mixer.id.clone().expect("mixer id");

    // 3. getPropertyAndReferenceNames includes Volume.
    let names = client
        .wwise()
        .core()
        .object()
        .get_property_and_reference_names(
            CoreObjectGetPropertyAndReferenceNamesArgs::new()
                .object(ObjectRef::from(mixer_id.as_str())),
        )
        .await
        .expect("get property and reference names");
    assert!(
        names.names.iter().any(|n| n == "Volume"),
        "names should include Volume, got {names:?}"
    );

    // 4. getPropertyInfo for Volume reports a value type.
    let prop_info = client
        .wwise()
        .core()
        .object()
        .get_property_info(
            CoreObjectGetPropertyInfoArgs::new(Property::Volume)
                .object(ObjectRef::from(mixer_id.as_str())),
        )
        .await
        .expect("get property info");
    assert_eq!(prop_info.name.as_deref(), Some("Volume"));
    assert!(prop_info.object_type.as_deref().is_some());

    // 5. isLinked / setLinked on the Volume property.
    client
        .wwise()
        .core()
        .object()
        .set_linked(CoreObjectSetLinkedArgs::new(
            ObjectRef::from(mixer_id.as_str()),
            Property::Volume,
            platform.clone(),
            false,
        ))
        .await
        .expect("set linked");
    let linked = client
        .wwise()
        .core()
        .object()
        .is_linked(CoreObjectIsLinkedArgs::new(
            ObjectRef::from(mixer_id.as_str()),
            Property::Volume,
            platform.clone(),
        ))
        .await
        .expect("is linked");
    assert_eq!(linked.linked, Some(false));

    // 6. isPropertyEnabled for Volume.
    let enabled = client
        .wwise()
        .core()
        .object()
        .is_property_enabled(CoreObjectIsPropertyEnabledArgs::new(
            ObjectRef::from(mixer_id.as_str()),
            platform.clone(),
            Property::Volume,
        ))
        .await
        .expect("is property enabled");
    assert!(enabled.enabled.is_some());

    // 7. setStateGroups + setStateProperties on a State Group.
    let state_group = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(STATE_GROUPS_WU),
                ObjectType::StateGroup,
                "WaapiSeqIntrospectStateGroup".to_string(),
            )
            .on_name_conflict(wwise_waapi::api::OnNameConflict::Replace),
        )
        .await
        .expect("create state group");
    let state_group_id = state_group.id.clone().expect("state group id");

    client
        .wwise()
        .core()
        .object()
        .set_state_groups(CoreObjectSetStateGroupsArgs::new(
            ObjectRef::from(mixer_id.as_str()),
            vec![ObjectRef::from(state_group_id.as_str())],
        ))
        .await
        .expect("set state groups");
    client
        .wwise()
        .core()
        .object()
        .set_state_properties(CoreObjectSetStatePropertiesArgs::new(
            ObjectRef::from(mixer_id.as_str()),
            vec!["Volume".to_string(), "Pitch".to_string()],
        ))
        .await
        .expect("set state properties");

    // Cleanup.
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            mixer_id.as_str(),
        )))
        .await
        .expect("delete mixer");
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            state_group_id.as_str(),
        )))
        .await
        .expect("delete state group");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
