use super::{connect_client, sequence_lock};
use waapi_client::api::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectDeleteArgs, CoreObjectGetArgs, CoreObjectGetOptions,
};
use waapi_client::api::{
    ObjectQuery, ObjectReturnField, OnNameConflict, Transform, TransformSelect,
};
use waapi_client::{ChildObject, ObjectRef, ObjectType, Property};

const DEFAULT_WORK_UNIT_PATH: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";
const EVENTS_WORK_UNIT_PATH: &str = r"\Events\Default Work Unit";
const QUERY_MIXER_NAME: &str = "WaapiQueryTestMixer";
const QUERY_EVENT_NAME: &str = "WaapiQueryTestEvent";
const SWITCH_CONTAINER_NAME: &str = "WaapiQuerySwitchContainer";
const RANDOM_CONTAINER_NAME: &str = "WaapiQueryRandomContainer";
const SOUND_ALPHA_NAME: &str = "WaapiQuerySoundAlpha";
const SOUND_BETA_NAME: &str = "WaapiQuerySoundBeta";

#[tokio::test]
async fn object_get_queries_and_transforms_sequence() {
    let _guard = sequence_lock().await;
    let mut client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    let return_opts = CoreObjectGetOptions::new().return_fields(vec![
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Type,
        ObjectReturnField::Path,
        ObjectReturnField::Parent,
        ObjectReturnField::Workunit,
        ObjectReturnField::Category,
        ObjectReturnField::ChildrenCount,
        ObjectReturnField::IsPlayable,
    ]);

    // 1. Create a test hierarchy under Default Work Unit
    let create_args = CoreObjectCreateArgs::new(
        ObjectRef::from(DEFAULT_WORK_UNIT_PATH),
        ObjectType::ActorMixer,
        QUERY_MIXER_NAME.to_string(),
    )
    .on_name_conflict(OnNameConflict::Replace)
    .children(vec![
        ChildObject::new(ObjectType::SwitchContainer, SWITCH_CONTAINER_NAME),
        ChildObject::new(ObjectType::RandomSequenceContainer, RANDOM_CONTAINER_NAME).children(
            vec![
                ChildObject::new(ObjectType::Sound, SOUND_ALPHA_NAME),
                ChildObject::new(ObjectType::Sound, SOUND_BETA_NAME),
            ],
        ),
    ]);

    let created_mixer = client
        .wwise()
        .core()
        .object()
        .create(create_args)
        .await
        .expect("Failed to create test query hierarchy");
    let mixer_id = created_mixer
        .id
        .expect("Created query ActorMixer missing id");

    // 2. Query by Path
    let alpha_path = format!(
        r"{}\{}\{}\{}",
        DEFAULT_WORK_UNIT_PATH, QUERY_MIXER_NAME, RANDOM_CONTAINER_NAME, SOUND_ALPHA_NAME
    );
    let beta_path = format!(
        r"{}\{}\{}\{}",
        DEFAULT_WORK_UNIT_PATH, QUERY_MIXER_NAME, RANDOM_CONTAINER_NAME, SOUND_BETA_NAME
    );

    let by_path_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::Path(vec![
                alpha_path.clone(),
                beta_path.clone(),
            ])),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query objects by ObjectQuery::Path");
    assert_eq!(by_path_resp.objects.len(), 2);
    let alpha_id = by_path_resp.objects[0]
        .id
        .clone()
        .expect("SoundAlpha missing id");
    let beta_id = by_path_resp.objects[1]
        .id
        .clone()
        .expect("SoundBeta missing id");
    assert_eq!(
        by_path_resp.objects[0]
            .parent
            .as_ref()
            .and_then(|p| p.name.as_deref()),
        Some(RANDOM_CONTAINER_NAME)
    );
    assert_eq!(
        by_path_resp.objects[0]
            .workunit
            .as_ref()
            .and_then(|w| w.name.as_deref()),
        Some("Default Work Unit")
    );

    // 3. Query by multiple GUIDs (`ObjectQuery::from(vec![id1, id2])`)
    let by_ids_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(vec![alpha_id.clone(), beta_id.clone()])),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query objects by multiple GUIDs");
    assert_eq!(by_ids_resp.objects.len(), 2);

    // 4. Query by qualified Name (`"Event:<Name>"` — globally unique named type)
    let created_event = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(EVENTS_WORK_UNIT_PATH),
                ObjectType::Event,
                QUERY_EVENT_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace)
            .children(vec![
                ChildObject::new(ObjectType::Action, "")
                    .property(Property::ActionType, 1)
                    .property(Property::Target, alpha_id.as_str()),
            ]),
        )
        .await
        .expect("Failed to create temporary Event for qualified Name query");
    let event_id = created_event.id.expect("Created Event missing id");
    let action_id = created_event.children[0]
        .id
        .clone()
        .expect("Created Action child missing id");

    let qualified_name = format!("Event:{QUERY_EVENT_NAME}");
    let by_name_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(qualified_name.as_str())),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query object by qualified Name");
    assert_eq!(by_name_resp.objects.len(), 1);
    assert_eq!(
        by_name_resp.objects[0].id.as_deref(),
        Some(event_id.as_str())
    );
    assert_eq!(by_name_resp.objects[0].object_type, Some(ObjectType::Event));

    // 5. Query by Search (`ObjectQuery::Search`)
    let by_search_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::Search(vec!["WaapiQuerySound".to_string()])),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query objects by ObjectQuery::Search");
    assert_eq!(by_search_resp.objects.len(), 2);

    // 6. Query by ObjectType (`ObjectQuery::OfType`)
    let by_type_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(vec![
                ObjectType::SwitchContainer,
                ObjectType::RandomSequenceContainer,
            ])),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query objects by ObjectQuery::OfType");
    assert!(
        by_type_resp
            .objects
            .iter()
            .any(|o| o.name.as_deref() == Some(SWITCH_CONTAINER_NAME)
                && o.object_type == Some(ObjectType::SwitchContainer))
    );
    assert!(
        by_type_resp
            .objects
            .iter()
            .any(|o| o.name.as_deref() == Some(RANDOM_CONTAINER_NAME)
                && o.object_type == Some(ObjectType::RandomSequenceContainer))
    );

    // 7. Query by WAQL (`CoreObjectGetArgs::from_waql`)
    let waql_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::from_waql(r#"$ from type Sound where name : "WaapiQuerySound""#),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query objects using WAQL");
    assert_eq!(waql_resp.objects.len(), 2);

    // 8. TransformSelect: Children, Parent, Ancestors, and ReferencesTo
    let children_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(mixer_id.as_str()))
                .transform(vec![Transform::select(TransformSelect::Children)]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query Children transform");
    assert_eq!(children_resp.objects.len(), 2);

    let parent_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(alpha_id.as_str()))
                .transform(vec![Transform::select(TransformSelect::Parent)]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query Parent transform");
    assert_eq!(parent_resp.objects.len(), 1);
    assert_eq!(
        parent_resp.objects[0].name.as_deref(),
        Some(RANDOM_CONTAINER_NAME)
    );

    let ancestors_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(alpha_id.as_str()))
                .transform(vec![Transform::select(TransformSelect::Ancestors)]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query Ancestors transform");
    assert!(
        ancestors_resp
            .objects
            .iter()
            .any(|o| o.id.as_deref() == Some(mixer_id.as_str()))
    );

    let refs_to_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(alpha_id.as_str()))
                .transform(vec![Transform::select(TransformSelect::ReferencesTo)]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query ReferencesTo transform");
    assert!(
        refs_to_resp
            .objects
            .iter()
            .any(|o| o.id.as_deref() == Some(action_id.as_str())
                && o.object_type == Some(ObjectType::Action))
    );

    // 9. Transform::distinct: Selecting Parent from two sibling sounds produces 2 duplicate
    //    RandomContainer entries without distinct, and 1 entry with Transform::distinct().
    let duplicate_parents_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(vec![alpha_id.clone(), beta_id.clone()]))
                .transform(vec![Transform::select(TransformSelect::Parent)]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query Parent of two sibling sounds");
    assert_eq!(duplicate_parents_resp.objects.len(), 2);

    let distinct_parent_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(vec![alpha_id.clone(), beta_id.clone()]))
                .transform(vec![
                    Transform::select(TransformSelect::Parent),
                    Transform::distinct(),
                ]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query distinct Parent of two sibling sounds");
    assert_eq!(distinct_parent_resp.objects.len(), 1);
    assert_eq!(
        distinct_parent_resp.objects[0].name.as_deref(),
        Some(RANDOM_CONTAINER_NAME)
    );

    // 10. Transform::range: Paginate all 4 descendants in two pages of 2 ([0, 2] and [2, 2])
    let page_one_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(mixer_id.as_str())).transform(vec![
                Transform::select(TransformSelect::Descendants),
                Transform::range(0, 2),
            ]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query Descendants page 1 with Range transform");
    assert_eq!(page_one_resp.objects.len(), 2);

    let page_two_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(mixer_id.as_str())).transform(vec![
                Transform::select(TransformSelect::Descendants),
                Transform::range(2, 2),
            ]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to query Descendants page 2 with Range transform");
    assert_eq!(page_two_resp.objects.len(), 2);
    assert_ne!(page_one_resp.objects[0].id, page_two_resp.objects[0].id);

    // 11. Transform::Where predicates (name:contains, name:matches, type:isIn)
    let where_contains_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(mixer_id.as_str())).transform(vec![
                Transform::select(TransformSelect::Descendants),
                Transform::where_name_contains("Sound"),
            ]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to filter Descendants with where_name_contains");
    assert_eq!(where_contains_resp.objects.len(), 2);

    let where_matches_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(mixer_id.as_str())).transform(vec![
                Transform::select(TransformSelect::Descendants),
                Transform::where_name_matches(".*Alpha$"),
            ]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to filter Descendants with where_name_matches");
    assert_eq!(where_matches_resp.objects.len(), 1);
    assert_eq!(
        where_matches_resp.objects[0].id.as_deref(),
        Some(alpha_id.as_str())
    );

    let where_type_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(mixer_id.as_str())).transform(vec![
                Transform::select(TransformSelect::Descendants),
                Transform::where_type_is_in([
                    ObjectType::SwitchContainer,
                    ObjectType::RandomSequenceContainer,
                ]),
            ]),
            return_opts.clone(),
        )
        .await
        .expect("Failed to filter Descendants with where_type_is_in");
    assert_eq!(where_type_resp.objects.len(), 2);
    assert!(where_type_resp.objects.iter().all(|o| matches!(
        o.object_type,
        Some(ObjectType::SwitchContainer | ObjectType::RandomSequenceContainer)
    )));

    // 12. Multi-step chained Transform pipeline:
    //     Select(Descendants) -> where_type_is_in([Sound]) -> where_name_contains("Beta") -> range(0, 1)
    let chained_transform_resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(mixer_id.as_str())).transform(vec![
                Transform::select(TransformSelect::Descendants),
                Transform::where_type_is_in([ObjectType::Sound]),
                Transform::where_name_contains("Beta"),
                Transform::range(0, 1),
            ]),
            return_opts,
        )
        .await
        .expect("Failed to run multi-step chained Transform pipeline");
    assert_eq!(chained_transform_resp.objects.len(), 1);
    assert_eq!(
        chained_transform_resp.objects[0].id.as_deref(),
        Some(beta_id.as_str())
    );

    // 10. Clean up test hierarchy and temporary Event
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            event_id.as_str(),
        )))
        .await
        .expect("Failed to delete temporary query Event");

    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            mixer_id.as_str(),
        )))
        .await
        .expect("Failed to delete test query ActorMixer");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
