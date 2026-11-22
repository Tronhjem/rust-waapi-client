use super::audio_import::minimal_wav_base64;
use super::{connect_client, sequence_lock};
use wwise_waapi::api::ak::wwise::core::audio::{CoreAudioImportArgs, CoreAudioImportOptions};
use wwise_waapi::api::ak::wwise::core::gameParameter::CoreGameParameterSetRangeArgs;
use wwise_waapi::api::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectDeleteArgs, CoreObjectGetArgs,
};
use wwise_waapi::api::ak::wwise::core::sound::CoreSoundSetActiveSourceArgs;
use wwise_waapi::api::{
    ImportDefaults, ImportItem, ImportOperation, ObjectQuery, ObjectReturnField,
    ObjectReturnOptions, OnCurveUpdate, OnNameConflict, Transform, TransformSelect,
};
use wwise_waapi::{ObjectRef, ObjectType, Property};

const ACTOR_MIXER_WU: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";
const GAME_PARAMS_WU: &str = r"\Game Parameters\Default Work Unit";

#[tokio::test]
async fn set_active_source_set_range_and_ping_sequence() {
    let _guard = sequence_lock().await;
    let client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    // 1. core.ping.
    let ping = client.wwise().core().ping().await.expect("ping");
    assert!(ping.is_available);

    // 2. Import a WAV into a Sound so we have a source to activate.
    let import_resp = client
        .wwise()
        .core()
        .audio()
        .import(
            CoreAudioImportArgs::new(vec![
                ImportItem::new(format!(
                    r"{ACTOR_MIXER_WU}\<Sound>WaapiSeqActiveSourceSound"
                ))
                .audio_file_base64(minimal_wav_base64("WaapiSeqActiveSourceTone.wav")),
            ])
            .import_operation(ImportOperation::ReplaceExisting)
            .default(ImportDefaults::new().object_type(ObjectType::Sound)),
            CoreAudioImportOptions::new().return_fields(vec![
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Type,
            ]),
        )
        .await
        .expect("import wav");
    let sound = import_resp
        .objects
        .iter()
        .find(|o| o.object_type == Some(ObjectType::Sound))
        .expect("imported sound");
    let sound_id = sound.id.as_deref().expect("imported sound id").to_string();

    // The source is created as a child of the imported Sound.
    let children = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(sound_id.as_str())).transform(vec![
                Transform::Select {
                    select: vec![TransformSelect::Children],
                },
            ]),
            ObjectReturnOptions::new()
                .return_fields(vec![ObjectReturnField::Id, ObjectReturnField::Type]),
        )
        .await
        .expect("query sound children");
    let source = children
        .objects
        .iter()
        .find(|o| o.object_type == Some(ObjectType::AudioFileSource))
        .expect("audio file source child");
    let source_id = source
        .id
        .as_deref()
        .expect("audio file source id")
        .to_string();

    // 3. sound.setActiveSource.
    client
        .wwise()
        .core()
        .sound()
        .set_active_source(CoreSoundSetActiveSourceArgs::new(
            ObjectRef::from(sound_id.as_str()),
            ObjectRef::from(source_id.as_str()),
        ))
        .await
        .expect("set active source");

    // 4. gameParameter.setRange, then verify Min/Max.
    let game_param = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(GAME_PARAMS_WU),
                ObjectType::GameParameter,
                "WaapiSeqRangeParam".to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create game parameter");
    let game_param_id = game_param.id.clone().expect("game param id");

    client
        .wwise()
        .core()
        .game_parameter()
        .set_range(CoreGameParameterSetRangeArgs::new(
            ObjectRef::from(game_param_id.as_str()),
            0.0,
            100.0,
            OnCurveUpdate::Stretch,
        ))
        .await
        .expect("set range");

    let fetched = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(game_param_id.as_str())),
            ObjectReturnOptions::new().return_fields(vec![
                ObjectReturnField::Property(Property::Min),
                ObjectReturnField::Property(Property::Max),
            ]),
        )
        .await
        .expect("get game parameter");
    assert_eq!(
        fetched.objects[0]
            .property(Property::Min)
            .and_then(|v| v.as_f64()),
        Some(0.0)
    );
    assert_eq!(
        fetched.objects[0]
            .property(Property::Max)
            .and_then(|v| v.as_f64()),
        Some(100.0)
    );

    // Cleanup.
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            sound_id.as_str(),
        )))
        .await
        .expect("delete imported sound");
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            game_param_id.as_str(),
        )))
        .await
        .expect("delete game parameter");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
