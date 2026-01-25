use std::collections::HashMap;

use log::{debug, error, info};
use waapi_client::waapi_function_api::ak;
use waapi_client::{ReturnType, WaapiArgs, WaapiClient, WaapiOptions, WaapiValue};

#[tokio::main]
async fn main() {
    WaapiClient::initialize_logger(Some("info"));

    let test_address: &str = "127.0.0.1:8080";
    info!("Connecting to WAAPI at {}", test_address);

    let waapi_client = WaapiClient::new(Some(test_address));

    if let Ok(mut client) = waapi_client {
        info!("Successfully connected to WAAPI");

        debug!("Calling getInfo endpoint");
        let info_response = client
            .call(ak::wwise::core::getInfo, None, None)
            .await
            .unwrap();

        info_response.print();

        let sel_options = WaapiOptions::with_return(&[ReturnType::Id]);
        let selected = client
            .call(ak::wwise::ui::getSelectedObjects, None, Some(sel_options))
            .await
            .unwrap();

        selected.print();

        let sel_id = selected.get_return_value(ReturnType::Id).unwrap();

        let mut from_dict = HashMap::new();
        from_dict.insert("id".to_string(), WaapiValue::List(vec![sel_id]));

        let args = WaapiArgs::new().insert("from", WaapiValue::Dict(from_dict));

        let options = WaapiOptions::with_return(&[
            ReturnType::Id,
            ReturnType::Type,
            ReturnType::Name,
            ReturnType::Notes,
        ]);

        info!("Getting object details for selected item");
        let get_respons = client
            .call(ak::wwise::core::object::get, Some(args), Some(options))
            .await
            .unwrap();

        get_respons.print();

        info!("Shutting down WAAPI client");
        client.shutdown().await.unwrap();
    } else {
        error!("Failed to connect to WAAPI at {}", test_address);
    }
}
