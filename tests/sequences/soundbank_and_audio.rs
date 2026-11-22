use super::audio_import::minimal_wav_base64;
use super::{connect_client, sequence_lock};
use waapi_client::api::ak::wwise::core::audio::{
    CoreAudioImportArgs, CoreAudioImportOptions, CoreAudioMuteArgs, CoreAudioSoloArgs,
};
use waapi_client::api::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectDeleteArgs, CoreObjectGetArgs,
};
use waapi_client::api::ak::wwise::core::soundbank::{
    CoreSoundbankGenerateArgs, CoreSoundbankGetInclusionsArgs, CoreSoundbankSetInclusionsArgs,
};
use waapi_client::api::{
    ImportDefaults, ImportItem, ImportOperation, ObjectQuery, ObjectReturnField,
    ObjectReturnOptions, OnNameConflict, SoundBankInclusion, SoundBankInclusionFilter,
    SoundBankInclusionOperation, SoundBankInfo,
};
use waapi_client::{ObjectRef, ObjectType};

const ACTOR_MIXER_WU: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";
const SOUNDBANKS_WU: &str = r"\SoundBanks\Default Work Unit";

async fn get_sound_fields(
    client: &mut waapi_client::WaapiClient,
    id: &str,
    fields: Vec<ObjectReturnField>,
) -> waapi_client::api::WwiseObject {
    let resp = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(id)),
            ObjectReturnOptions::new().return_fields(fields),
        )
        .await
        .expect("get sound");
    resp.objects.into_iter().next().expect("sound object")
}

#[tokio::test]
async fn soundbank_and_audio_sequence() {
    let _guard = sequence_lock().await;
    let mut client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    let project_info = client
        .wwise()
        .core()
        .get_project_info()
        .await
        .expect("get project info");
    let platform_id = project_info.current_platform_id.clone();

    // Import a WAV so the Sound has an active audio source (generate fails otherwise).
    let import_resp = client
        .wwise()
        .core()
        .audio()
        .import(
            CoreAudioImportArgs::new(vec![
                ImportItem::new(format!(r"{ACTOR_MIXER_WU}\<Sound>WaapiSeqBankSound"))
                    .audio_file_base64(minimal_wav_base64("WaapiSeqBankTone.wav")),
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
    let sound_id = import_resp
        .objects
        .iter()
        .find(|o| o.object_type == Some(ObjectType::Sound))
        .and_then(|o| o.id.as_deref())
        .expect("imported sound id")
        .to_string();

    let soundbank = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(SOUNDBANKS_WU),
                ObjectType::SoundBank,
                "WaapiSeqTestBank".to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("create soundbank");
    let soundbank_id = soundbank.id.clone().expect("soundbank id");

    // 1. setInclusions (add) + getInclusions.
    client
        .wwise()
        .core()
        .soundbank()
        .set_inclusions(CoreSoundbankSetInclusionsArgs::new(
            ObjectRef::from(soundbank_id.as_str()),
            vec![
                SoundBankInclusion::new(ObjectRef::from(sound_id.as_str()))
                    .filter(SoundBankInclusionFilter::Structures)
                    .filter(SoundBankInclusionFilter::Media),
            ],
            SoundBankInclusionOperation::Add,
        ))
        .await
        .expect("set inclusions");

    let inclusions = client
        .wwise()
        .core()
        .soundbank()
        .get_inclusions(CoreSoundbankGetInclusionsArgs::new(ObjectRef::from(
            soundbank_id.as_str(),
        )))
        .await
        .expect("get inclusions");
    assert_eq!(inclusions.inclusions.len(), 1);
    match &inclusions.inclusions[0].object {
        ObjectRef::Guid(g) => assert_eq!(g, &sound_id),
        other => panic!("expected inclusion object GUID, got {other:?}"),
    }

    // 2. generate (in-memory, writeToDisk false).
    let gen_resp = client
        .wwise()
        .core()
        .soundbank()
        .generate(
            CoreSoundbankGenerateArgs::new()
                .platforms(vec![platform_id.clone()])
                .soundbanks(vec![SoundBankInfo::new("WaapiSeqTestBank")])
                .write_to_disk(false),
        )
        .await
        .expect("generate soundbank");
    assert!(
        gen_resp.logs.is_empty()
            || gen_resp.logs.iter().all(|l| {
                l.severity != waapi_client::api::LogSeverity::Error
                    && l.severity != waapi_client::api::LogSeverity::FatalError
            })
    );

    // 3. audio.mute / reset_mute, solo / reset_solo (verified via return fields).
    client
        .wwise()
        .core()
        .audio()
        .mute(CoreAudioMuteArgs::new(
            vec![ObjectRef::from(sound_id.as_str())],
            true,
        ))
        .await
        .expect("mute");
    let muted = get_sound_fields(
        &mut client,
        sound_id.as_str(),
        vec![ObjectReturnField::IsExplicitMute],
    )
    .await;
    assert_eq!(muted.is_explicit_mute, Some(true));

    client
        .wwise()
        .core()
        .audio()
        .reset_mute()
        .await
        .expect("reset mute");
    let unmuted = get_sound_fields(
        &mut client,
        sound_id.as_str(),
        vec![ObjectReturnField::IsExplicitMute],
    )
    .await;
    assert_eq!(unmuted.is_explicit_mute, Some(false));

    client
        .wwise()
        .core()
        .audio()
        .solo(CoreAudioSoloArgs::new(
            vec![ObjectRef::from(sound_id.as_str())],
            true,
        ))
        .await
        .expect("solo");
    let soloed = get_sound_fields(
        &mut client,
        sound_id.as_str(),
        vec![ObjectReturnField::IsExplicitSolo],
    )
    .await;
    assert_eq!(soloed.is_explicit_solo, Some(true));

    client
        .wwise()
        .core()
        .audio()
        .reset_solo()
        .await
        .expect("reset solo");
    let unsoloed = get_sound_fields(
        &mut client,
        sound_id.as_str(),
        vec![ObjectReturnField::IsExplicitSolo],
    )
    .await;
    assert_eq!(unsoloed.is_explicit_solo, Some(false));

    // Cleanup.
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            soundbank_id.as_str(),
        )))
        .await
        .expect("delete soundbank");
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            sound_id.as_str(),
        )))
        .await
        .expect("delete sound");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
