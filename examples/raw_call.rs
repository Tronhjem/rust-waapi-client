//! Demonstrates the string-based escape hatch for endpoints that have no
//! typed binding yet.
//!
//! Requires Wwise running with WAAPI enabled on 127.0.0.1:8080.
//! Run with: cargo run --example raw_call

use log::{error, info};
use waapi_client::WaapiClient;
use waapi_client::serde_json::json;
use waapi_client::waapi_function_api::ak;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut client = WaapiClient::new(Some("127.0.0.1:8080"))
        .await
        .expect("Failed to connect to WAAPI");

    // ======================================================================== 
    // Simple get info call with ak.wwise.core.getInfo
    // ======================================================================== 
    info!("Getting info:");
    match client
        .call(ak::wwise::core::getInfo, None, None)
        .await
    {
        Ok(value) => info!("getinfo result: {value}"),
        Err(e) => error!("getInfo failed: {e}"),
    }

    // ======================================================================== 
    // ak.wwise.core.object.get
    // ======================================================================== 
    info!("Getting Objects:");
    // Any WAAPI URI works; args and options are plain serde_json values.
    let args = json!({
        "from": { "path": [r"\Actor-Mixer Hierarchy\Default Work Unit"] }
    });
    let options = json!({
        "return": ["id", "type", "name", "notes"]
    });

    match client
        .call(ak::wwise::core::object::get, Some(args), Some(options))
        .await
    {
        Ok(value) => info!("object.get result: {value}"),
        Err(e) => error!("object.get failed: {e}"),
    }
    // ======================================================================== 

    client.shutdown().await.unwrap();
}
