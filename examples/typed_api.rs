use log::info;
use waapi_client::api::generated::ak::wwise::core::object::{
    WwiseCoreObjectGetArgs, WwiseCoreObjectGetOptions,
};
use waapi_client::api::generated::ak::wwise::ui::WwiseUiGetselectedobjectsOptions;
use waapi_client::api::generated::primitives::ObjectQuery;
use waapi_client::{WaapiClient, WaapiResult};

#[tokio::main]
async fn main() -> WaapiResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let address = "127.0.0.1:8080";
    info!("Connecting to typed WAAPI at {}", address);

    let mut client = WaapiClient::new(Some(address)).await?;
    info!("Successfully connected to WAAPI");

    // getInfo: no arguments, no options.
    let info = client.get_info(()).await?;
    info!(
        "Wwise version: {}",
        info.version.display_name.unwrap_or_default()
    );

    // Get currently selected objects in the UI.
    let selected = client
        .ui_get_selected_objects(
            (),
            WwiseUiGetselectedobjectsOptions::new().return_fields(vec![
                "id".to_string(),
                "name".to_string(),
                "type".to_string(),
            ]),
        )
        .await?;

    if let Some(first) = selected.objects.first() {
        info!("Selected: {:?} {:?}", first.name, first.object_type);

        // Query object details using the typed object.get wrapper.
        let details = client
            .object_get(
                WwiseCoreObjectGetArgs::new(ObjectQuery::Id(vec![
                    first.id.clone().unwrap_or_default(),
                ])),
                WwiseCoreObjectGetOptions::new().return_fields(vec![
                    "id".to_string(),
                    "name".to_string(),
                    "type".to_string(),
                    "notes".to_string(),
                ]),
            )
            .await?;

        for obj in details.objects {
            info!("Object details: {:?}", obj);
        }
    }

    client.shutdown().await?;
    Ok(())
}
