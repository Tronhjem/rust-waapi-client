use super::connect_client;
use waapi_client::api::ak::wwise::core::audio::{CoreAudioImportArgs, CoreAudioImportOptions};
use waapi_client::api::ak::wwise::core::object::{
    CoreObjectCreateArgs, CoreObjectDeleteArgs, CoreObjectGetArgs, CoreObjectGetOptions,
};
use waapi_client::api::{
    ImportDefaults, ImportItem, ImportOperation, ObjectQuery, ObjectReturnField, OnNameConflict,
    Transform, TransformSelect,
};
use waapi_client::{ObjectRef, ObjectType, Property};

const DEFAULT_WORK_UNIT_PATH: &str = r"\Actor-Mixer Hierarchy\Default Work Unit";
const IMPORT_MIXER_NAME: &str = "WaapiImportTestMixer";
const IMPORTED_SOUND_NAME: &str = "WaapiImportedSound";

fn encode_base64(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        out.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        out.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn minimal_wav_base64(filename: &str) -> String {
    let sample_rate: u32 = 8000;
    let num_samples: u32 = 400; // 50 ms mono 16-bit PCM
    let data_size: u32 = num_samples * 2;
    let riff_size: u32 = 36 + data_size;

    let mut wav = Vec::with_capacity(44 + data_size as usize);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&riff_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // 1 channel
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    for i in 0..num_samples {
        let sample: i16 = if (i / 10) % 2 == 0 { 2000 } else { -2000 };
        wav.extend_from_slice(&sample.to_le_bytes());
    }

    format!("{}|{}", filename, encode_base64(&wav))
}

#[tokio::test]
async fn audio_import_base64_sequence() {
    let mut client = connect_client()
        .await
        .expect("Failed to connect to Wwise WAAPI on ws://127.0.0.1:8080/waapi");

    // 1. Create a temporary ActorMixer to hold imported audio objects
    let created_mixer = client
        .wwise()
        .core()
        .object()
        .create(
            CoreObjectCreateArgs::new(
                ObjectRef::from(DEFAULT_WORK_UNIT_PATH),
                ObjectType::ActorMixer,
                IMPORT_MIXER_NAME.to_string(),
            )
            .on_name_conflict(OnNameConflict::Replace),
        )
        .await
        .expect("Failed to create ActorMixer for audio import");
    let mixer_id = created_mixer
        .id
        .expect("Created import ActorMixer missing id");

    // 2. Import an in-memory Base64 WAV into the temporary ActorMixer
    let object_path = format!(
        r"{}\{}\<Sound>{}",
        DEFAULT_WORK_UNIT_PATH, IMPORT_MIXER_NAME, IMPORTED_SOUND_NAME
    );
    let wav_b64 = minimal_wav_base64("WaapiSeqTestTone.wav");

    let import_args = CoreAudioImportArgs::new(vec![
        ImportItem::new(object_path.clone())
            .audio_file_base64(wav_b64)
            .notes("Imported via base64 test")
            .property(Property::Volume, -4.5),
    ])
    .import_operation(ImportOperation::ReplaceExisting)
    .default(ImportDefaults::new().object_type(ObjectType::Sound));

    let import_opts = CoreAudioImportOptions::new().return_fields(vec![
        ObjectReturnField::Id,
        ObjectReturnField::Name,
        ObjectReturnField::Type,
        ObjectReturnField::Notes,
        ObjectReturnField::Property(Property::Volume),
    ]);

    let import_resp = client
        .wwise()
        .core()
        .audio()
        .import(import_args, import_opts)
        .await
        .expect("Failed to import base64 WAV via ak.wwise.core.audio.import");

    assert!(!import_resp.objects.is_empty());
    let imported_sound = import_resp
        .objects
        .iter()
        .find(|o| o.name.as_deref() == Some(IMPORTED_SOUND_NAME))
        .expect("Imported Sound missing from CoreAudioImportResponse.objects");
    assert_eq!(imported_sound.object_type, Some(ObjectType::Sound));
    assert_eq!(
        imported_sound.notes.as_deref(),
        Some("Imported via base64 test")
    );
    assert_eq!(
        imported_sound
            .property(Property::Volume)
            .and_then(|v| v.as_f64()),
        Some(-4.5)
    );
    let imported_sound_id = imported_sound
        .id
        .as_deref()
        .expect("Imported Sound missing id");

    // 3. Query the imported Sound's children to verify the AudioFileSource was created
    let sound_children = client
        .wwise()
        .core()
        .object()
        .get(
            CoreObjectGetArgs::new(ObjectQuery::from(imported_sound_id)).transform(vec![
                Transform::Select {
                    select: vec![TransformSelect::Children],
                },
            ]),
            CoreObjectGetOptions::new().return_fields(vec![
                ObjectReturnField::Id,
                ObjectReturnField::Name,
                ObjectReturnField::Type,
            ]),
        )
        .await
        .expect("Failed to query imported Sound children");

    assert!(
        sound_children
            .objects
            .iter()
            .any(|c| c.object_type == Some(ObjectType::AudioFileSource)),
        "Expected an AudioFileSource child under imported Sound, got: {:?}",
        sound_children.objects
    );

    // 4. Clean up the temporary ActorMixer
    client
        .wwise()
        .core()
        .object()
        .delete(CoreObjectDeleteArgs::new(ObjectRef::from(
            mixer_id.as_str(),
        )))
        .await
        .expect("Failed to delete import test ActorMixer");

    client
        .shutdown()
        .await
        .expect("Failed to shut down WaapiClient");
}
