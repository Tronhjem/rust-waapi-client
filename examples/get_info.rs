use log::{debug, error, info};
use serde_json::json;
use waapi_client::WaapiClient;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let test_address: &str = "127.0.0.1:8080";
    info!("Connecting to WAAPI at {}", test_address);

    match WaapiClient::new(Some(test_address)).await {
        Ok(mut client) => {
            info!("Successfully connected to WAAPI");

            debug!("Calling getInfo endpoint");
            match client.get_info(()).await {
                Ok(info) => {
                    info!("Wwise: {} ({})", info.display_name, info.platform);
                    info!("Version: {:?}", info.version);
                }
                Err(e) => error!("getInfo failed: {e}"),
            }

            let options = waapi_client::api::generated::ak::wwise::ui::WwiseUiGetselectedobjectsOptions::new()
                .return_fields(vec!["id".to_string()]);

            match client.ui_get_selected_objects((), options).await {
                Ok(selected) => {
                    info!("Selected {} objects", selected.objects.len());
                    for obj in &selected.objects {
                        info!("  selected id: {:?}", obj.id);
                    }
                }
                Err(e) => error!("getSelectedObjects failed: {e}"),
            }

            let args = json!({
                "from": { "path": [r"\Actor-Mixer Hierarchy\Default Work Unit"] }
            });
            let options = json!({
                "return": ["id", "type", "name", "notes"]
            });

            info!("Getting object details");
            match client.call("ak.wwise.core.object.get", Some(args), Some(options)).await {
                Ok(value) => info!("object.get result: {value}"),
                Err(e) => error!("object.get failed: {e}"),
            }

            info!("Shutting down WAAPI client");
            if let Err(e) = client.shutdown().await {
                error!("Shutdown failed: {e}");
            }
        }
        Err(e) => {
            error!("Failed to connect to WAAPI at {}: {}", test_address, e);
        }
    }
}
