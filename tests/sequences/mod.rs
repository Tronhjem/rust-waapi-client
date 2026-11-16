use waapi_client::{WaapiClient, WaapiResult};

pub mod audio_import;
pub mod info_ui_and_soundengine;
pub mod object_lifecycle;
pub mod queries_and_transforms;
pub mod subscriptions;

pub async fn connect_client() -> WaapiResult<WaapiClient> {
    let _ = env_logger::builder().is_test(true).try_init();
    WaapiClient::new(None).await
}
