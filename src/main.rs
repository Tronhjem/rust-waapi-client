// use std::collections::HashMap;

mod waapi_client;
use waapi_client::WaapiClient;


#[tokio::main]
async fn main() {

    let test_address = "ws://127.0.0.1:8080/waapi".to_string();
    let waapi_client = WaapiClient::new(Some(test_address));
    // let waapi_client = WaapiClient::new(None);

    if let Some(mut client) = waapi_client {
        client.shutdown().await;
    }

    // let _response = client
    //     .call(URI::new(ak::wwise::core::GET_INFO), None, None)
    //     .await;

    // let id = Value::String("{DC124660-8640-45DA-9871-7E3FB66E07D6}".to_string());
    //
    // let x = Value::String("id".to_string());
    // let y = Value::String("type".to_string());
    // let z = Value::String("name".to_string());
    //
    // let mut kwargs = HashMap::new();
    // let mut ids = HashMap::new();
    // ids.insert("id".to_string(), Value::List(vec![id]));
    // kwargs.insert("from".to_string(), Value::Dict(ids));
    //
    // let mut options = HashMap::new();
    // options.insert("return".to_string(), Value::List(vec![x, y, z]));
    //
    // let response = client
    //     .call_with_options(
    //         URI::new(ak::wwise::core::object::GET),
    //         None,
    //         Some(kwargs),
    //         options,
    //     )
    //     .await;
    //
    // match response {
    //     Ok(result) => {
    //         println!("Success! Response:");
    //         for (key, value) in &result.1 {
    //             println!("{}: {:?}", key, value);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error calling WAAPI: {:?}", e);
    //     }
    // }

    // println!("Disconnected");
}
