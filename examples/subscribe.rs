//! Subscribe to WAAPI topics (requires a running Wwise instance).
//!
//! Run with: cargo run --example subscribe

use log::{error, info};
use tokio::time::{timeout, Duration};
use waapi_client::WaapiClient;
use waapi_client::waapi_topics_api::ak;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let test_address: &str = "127.0.0.1:8080";
    info!("Connecting to WAAPI at {}", test_address);

    match WaapiClient::new(Some(test_address)).await {
        Ok(mut client) => {
            info!("Successfully connected to WAAPI");

            let topic = ak::wwise::core::object::created;
            let options = waapi_client::serde_json::json!({ "return": ["id", "name", "type"] });
            match client.subscribe(topic, Some(options)).await {
                Ok((subscription_id, mut events)) => {
                    info!("Subscribed to {topic} with id {subscription_id}");

                    // Print the first event within 60 seconds, then unsubscribe.
                    match timeout(Duration::from_secs(60), events.recv()).await {
                        Ok(Some(event)) => info!("Received event: {event:?}"),
                        Ok(None) => info!("Event channel closed"),
                        Err(_) => info!("No event received within 60 seconds"),
                    }

                    if let Err(e) = client.unsubscribe(subscription_id).await {
                        error!("Unsubscribe failed: {e}");
                    }
                }
                Err(e) => {
                    error!("Subscribe failed: {e}");
                }
            }

            client.shutdown().await.unwrap();
        }
        Err(e) => {
            error!("Failed to connect to WAAPI: {e}");
        }
    }
}
