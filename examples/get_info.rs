use std::collections::HashMap;

use waapi_client::waapi_function_api::ak;
use waapi_client::{ReturnType, WaapiArgs, WaapiClient, WaapiOptions, WaapiValue};

#[tokio::main]
async fn main() {
    let test_address: &str = "127.0.0.1:8080";
    let waapi_client = WaapiClient::new(Some(test_address));

    if let Ok(mut client) = waapi_client {
        // let response = client.call(ak::wwise::core::GET_INFO, None, None).await;

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

        let get_respons = client
            .call(ak::wwise::core::object::get, Some(args), Some(options))
            .await
            .unwrap();

        get_respons.print();

        client.shutdown().await.expect("Shutdown failed");
    }
}
