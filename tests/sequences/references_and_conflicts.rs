use super::{connect_client, sequence_lock};
use waapi_client::api::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectDeleteArgs, CoreObjectGetArgs, CoreObjectGetOptions,
    CoreObjectSetArgs, CoreObjectSetItem, CoreObjectSetOptions,
};
use waapi_client::api::{
    ListMode, ObjectQuery, ObjectReturnField, OnNameConflict, Transform, TransformSelect,
};
use waapi_client::{ChildObject, ObjectRef, ObjectType, Property};

const DEFAULT_WORK_UNIT_PATH: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";
const EVENTS_WORK_UNIT_PATH: &str = r"\Events\Default Work Unit";
const ATTENUATIONS_WORK_UNIT_PATH: &str = r"\Attenuations\Default Work Unit";
const MASTER_AUDIO_BUS_PATH: &str = r"\Master-Mixer Hierarchy\Default Work Unit\Master Audio Bus";

#[tokio::test]
async fn object_references_and_event_wiring_sequence() {
    let _guard = sequence_lock().await;
    let mut client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    const REF_MIXER_NAME: &str = "WaapiRefTestMixer";
    const REF_SOUND_NAME: &str = "WaapiRefTargetSound";
    const REF_ATTENUATION_NAME: &str = "WaapiRefTestAttenuation";
    const REF_EVENT_NAME: &str = "WaapiRefTestEvent";

    // 1. Create a temporary ActorMixer and target Sound
    let created_mixer = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(DEFAULT_WORK_UNIT_PATH),
                ObjectType::ActorMixer,
                REF_MIXER_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace)
            .children(vec![ChildObject::new(ObjectType::Sound, REF_SOUND_NAME)]),
        )
        .await
        .expect("Failed to create reference test ActorMixer and Sound");

    let mixer_id = created_mixer.id.expect("Created ActorMixer missing id");
    let sound_id = created_mixer.children[0]
        .id
        .clone()
        .expect("Created target Sound missing id");

    // 2. Create a temporary Attenuation ShareSet under \Attenuations\Default Work Unit
    let created_attenuation = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(ATTENUATIONS_WORK_UNIT_PATH),
                ObjectType::Attenuation,
                REF_ATTENUATION_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("Failed to create test Attenuation ShareSet");
    let attenuation_id = created_attenuation
        .id
        .expect("Created Attenuation missing id");

    // 3. Wire OutputBus and Attenuation references on the Sound via Property enum
    let set_refs_args = CoreObjectSetArgs::new(vec![
        CoreObjectSetItem::new(ObjectRef::from(sound_id.as_str()))
            .property(Property::OverrideOutput, true)
            .property(Property::OutputBus, MASTER_AUDIO_BUS_PATH)
            .property(Property::OverridePositioning, true)
            .property(Property::EnableAttenuation, true)
            .property(Property::Attenuation, attenuation_id.as_str()),
    ]);

    client
        .wwise()
        .core()
        .object()
        .set(set_refs_args, CoreObjectSetOptions::new())
        .await
        .expect("Failed to set OutputBus and Attenuation references on Sound");

    // 4. Create an Event with a Play Action targeting the Sound via Property::Target
    let created_event = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(EVENTS_WORK_UNIT_PATH),
                ObjectType::Event,
                REF_EVENT_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace)
            .children(vec![
                ChildObject::new(ObjectType::Action, "")
                    .property(Property::ActionType, 1)
                    .property(Property::Target, sound_id.as_str()),
            ]),
        )
        .await
        .expect("Failed to create Event with Play Action");
    let event_id = created_event.id.expect("Created Event missing id");
    let action_id = created_event.children[0]
        .id
        .clone()
        .expect("Created Action child missing id");

    // 5. Read back OutputBus and Attenuation references on the Sound
    let sound_ref_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(sound_id.as_str())),
            CoreObjectGetOptions::new().return_fields(vec![
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Property(Property::OutputBus),
                ObjectReturnField::Property(Property::Attenuation),
            ]),
        )
        .await
        .expect("Failed to get Sound references");

    assert_eq!(sound_ref_resp.objects.len(), 1);
    let sound_obj = &sound_ref_resp.objects[0];

    let output_bus_ref = sound_obj
        .property(Property::OutputBus)
        .expect("Missing @OutputBus on Sound");
    assert_eq!(
        output_bus_ref.get("name").and_then(|v| v.as_str()),
        Some("Master Audio Bus")
    );

    let attenuation_ref = sound_obj
        .property(Property::Attenuation)
        .expect("Missing @Attenuation on Sound");
    assert_eq!(
        attenuation_ref.get("id").and_then(|v| v.as_str()),
        Some(attenuation_id.as_str())
    );
    assert_eq!(
        attenuation_ref.get("name").and_then(|v| v.as_str()),
        Some(REF_ATTENUATION_NAME)
    );

    // 6. Read back Target reference on the Action
    let action_ref_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(action_id.as_str())),
            CoreObjectGetOptions::new().return_fields(vec![
                ObjectReturnField::Id,
                ObjectReturnField::Type,
                ObjectReturnField::Property(Property::Target),
            ]),
        )
        .await
        .expect("Failed to get Action target reference");

    assert_eq!(action_ref_resp.objects.len(), 1);
    let action_obj = &action_ref_resp.objects[0];
    assert_eq!(action_obj.object_type, Some(ObjectType::Action));
    let target_ref = action_obj
        .property(Property::Target)
        .expect("Missing @Target on Action");
    assert_eq!(
        target_ref.get("id").and_then(|v| v.as_str()),
        Some(sound_id.as_str())
    );
    assert_eq!(
        target_ref.get("name").and_then(|v| v.as_str()),
        Some(REF_SOUND_NAME)
    );

    // 7. Follow reverse references using TransformSelect::ReferencesTo
    let refs_to_sound = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(sound_id.as_str())).transform(vec![
                Transform::Select {
                    select: vec![TransformSelect::ReferencesTo],
                },
            ]),
            CoreObjectGetOptions::new().return_fields(vec![
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Type,
            ]),
        )
        .await
        .expect("Failed to query referencesTo on Sound");

    assert!(
        refs_to_sound
            .objects
            .iter()
            .any(|o| o.id.as_deref() == Some(action_id.as_str())),
        "Expected Action to appear in Sound's referencesTo: {:?}",
        refs_to_sound.objects
    );

    let refs_to_attenuation = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(attenuation_id.as_str())).transform(vec![
                Transform::Select {
                    select: vec![TransformSelect::ReferencesTo],
                },
            ]),
            CoreObjectGetOptions::new().return_fields(vec![
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Type,
            ]),
        )
        .await
        .expect("Failed to query referencesTo on Attenuation");

    assert!(
        refs_to_attenuation
            .objects
            .iter()
            .any(|o| o.id.as_deref() == Some(sound_id.as_str())),
        "Expected Sound to appear in Attenuation's referencesTo: {:?}",
        refs_to_attenuation.objects
    );

    // 8. Clean up Event, Attenuation, and ActorMixer
    for id in [&event_id, &attenuation_id, &mixer_id] {
        client
            .wwise()
            .core()
            .object()
            .delete(CoreObjectDeleteArgs::new(ObjectRef::from(id.as_str())))
            .await
            .expect("Failed to delete reference test object");
    }

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}

#[tokio::test]
async fn conflict_strategies_and_list_modes_sequence() {
    let _guard = sequence_lock().await;
    let mut client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    const CONFLICT_MIXER_NAME: &str = "WaapiConflictTestMixer";
    const RENAME_SOUND_NAME: &str = "ConflictSound";
    const MERGE_CONTAINER_NAME: &str = "MergeContainer";
    const MERGE_CHILD_A: &str = "MergeChildA";
    const MERGE_CHILD_B: &str = "MergeChildB";

    let base_opts = CoreObjectGetOptions::new().return_fields(vec![
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Type,
        ObjectReturnField::Property(Property::Volume),
        ObjectReturnField::Property(Property::Pitch),
        ObjectReturnField::Property(Property::Effects),
    ]);

    // 1. Create a clean parent ActorMixer
    let created_mixer = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(DEFAULT_WORK_UNIT_PATH),
                ObjectType::ActorMixer,
                CONFLICT_MIXER_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("Failed to create conflict test ActorMixer");
    let mixer_id = created_mixer.id.expect("Created ActorMixer missing id");

    // 2. Test OnNameConflict::Rename
    let first_sound = client
        .wwise()
        .core()
        .object()
        .create(CoreObjectCreateArgs::new(
            ObjectRef::from(mixer_id.as_str()),
            ObjectType::Sound,
            RENAME_SOUND_NAME.to_string(),
        ))
        .await
        .expect("Failed to create first ConflictSound");
    let first_sound_id = first_sound.id.expect("First ConflictSound missing id");

    let second_sound = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(mixer_id.as_str()),
                ObjectType::Sound,
                RENAME_SOUND_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Rename),
        )
        .await
        .expect("Failed to create second ConflictSound with OnNameConflict::Rename");
    let second_sound_id = second_sound.id.expect("Second ConflictSound missing id");
    assert_ne!(first_sound_id, second_sound_id);

    let renamed_got = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(second_sound_id.as_str())),
            base_opts.clone(),
        )
        .await
        .expect("Failed to query auto-renamed Sound");
    assert_eq!(renamed_got.objects.len(), 1);
    let renamed_name = renamed_got.objects[0]
        .name
        .as_deref()
        .expect("Auto-renamed Sound missing name");
    assert!(
        renamed_name.starts_with(RENAME_SOUND_NAME) && renamed_name != RENAME_SOUND_NAME,
        "Expected Wwise to auto-suffix ConflictSound, got: {}",
        renamed_name
    );

    // 3. Test OnNameConflict::Merge
    let merge_initial = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(mixer_id.as_str()),
                ObjectType::RandomSequenceContainer,
                MERGE_CONTAINER_NAME.to_string(),
            )
            .property(Property::Volume, -3.5)
            .children(vec![ChildObject::new(ObjectType::Sound, MERGE_CHILD_A)]),
        )
        .await
        .expect("Failed to create initial MergeContainer");
    let merge_id = merge_initial.id.expect("MergeContainer missing id");

    let merge_second = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(mixer_id.as_str()),
                ObjectType::RandomSequenceContainer,
                MERGE_CONTAINER_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Merge)
            .property(Property::Pitch, 250)
            .children(vec![ChildObject::new(ObjectType::Sound, MERGE_CHILD_B)]),
        )
        .await
        .expect("Failed to merge into MergeContainer with OnNameConflict::Merge");
    assert_eq!(merge_second.id.as_deref(), Some(merge_id.as_str()));

    let merged_container_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(merge_id.as_str())),
            base_opts.clone(),
        )
        .await
        .expect("Failed to get merged container");
    let merged_container = &merged_container_resp.objects[0];
    assert_eq!(
        merged_container
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-3.5)
    );
    assert_eq!(
        merged_container
            .property(Property::Pitch)
            .and_then(|v| v.as_i64()),
        Some(250)
    );

    let merged_children_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(merge_id.as_str())).transform(vec![
                Transform::Select {
                    select: vec![TransformSelect::Children],
                },
            ]),
            base_opts.clone(),
        )
        .await
        .expect("Failed to get merged container children");
    assert_eq!(merged_children_resp.objects.len(), 2);

    // 4. Test ListMode::Append and ListMode::ReplaceAll on @Effects ObjectList
    client
        .wwise()
        .core()
        .object()
        .set(
            CoreObjectSetArgs::new(vec![
                CoreObjectSetItem::new(ObjectRef::from(merge_id.as_str()))
                    .list_mode(ListMode::ReplaceAll)
                    .property(Property::OverrideEffect, true)
                    .property(
                        Property::Effects,
                        vec![ChildObject::new(ObjectType::EffectSlot, "")],
                    ),
            ]),
            CoreObjectSetOptions::new(),
        )
        .await
        .expect("Failed to set initial @Effects list with ListMode::ReplaceAll");

    client
        .wwise()
        .core()
        .object()
        .set(
            CoreObjectSetArgs::new(vec![
                CoreObjectSetItem::new(ObjectRef::from(merge_id.as_str()))
                    .list_mode(ListMode::Append)
                    .property(
                        Property::Effects,
                        vec![ChildObject::new(ObjectType::EffectSlot, "")],
                    ),
            ]),
            CoreObjectSetOptions::new(),
        )
        .await
        .expect("Failed to append to @Effects list with ListMode::Append");

    let after_append = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(merge_id.as_str())),
            base_opts.clone(),
        )
        .await
        .expect("Failed to query container after ListMode::Append");
    let effects_after_append = after_append.objects[0]
        .property(Property::Effects)
        .and_then(|v| v.as_array())
        .expect("Expected @Effects array after append");
    assert_eq!(effects_after_append.len(), 2);

    client
        .wwise()
        .core()
        .object()
        .set(
            CoreObjectSetArgs::new(vec![
                CoreObjectSetItem::new(ObjectRef::from(merge_id.as_str())).property(
                    Property::Effects,
                    vec![ChildObject::new(ObjectType::EffectSlot, "")],
                ),
            ])
            .list_mode(ListMode::ReplaceAll),
            CoreObjectSetOptions::new(),
        )
        .await
        .expect("Failed to replace @Effects list with top-level ListMode::ReplaceAll");

    let after_replace_all = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(merge_id.as_str())),
            base_opts,
        )
        .await
        .expect("Failed to query container after ListMode::ReplaceAll");
    let effects_after_replace = after_replace_all.objects[0]
        .property(Property::Effects)
        .and_then(|v| v.as_array())
        .expect("Expected @Effects array after replaceAll");
    assert_eq!(effects_after_replace.len(), 1);

    // 5. Clean up
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            mixer_id.as_str(),
        )))
        .await
        .expect("Failed to delete conflict test ActorMixer");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
