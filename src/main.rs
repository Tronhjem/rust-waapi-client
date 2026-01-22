use std::collections::HashMap;
use wampire::{Connection, Value, URI};

pub mod ak {
    pub mod wwise {
        pub mod core {
            pub const GET_INFO: &str = "ak.wwise.core.getInfo";

            pub mod object {
                pub const GET: &str = "ak.wwise.core.object.get";
            }
        }
        pub mod ui {
            pub const GET_SELECTED_OBJECTS: &str = "ak.wwise.ui.getSelectedObjects";
        }
    }
}

pub fn waapi_client(url: &str) -> Result<wampire::Client, ()> {
    println!("Connecting...");
    let connection = Connection::new(url, "waapi_client");

    let client = connection.connect();
    match client {
        Ok(client) => {
            println!("Client connected to {}", url);
            Ok(client)
        }
        Err(_e) => {
            println!("Coulnd't connect to {}", url);
            Err(())
        }
    }
}

#[tokio::main]
async fn main() {
    // env_logger::init();

    let url = "ws://127.0.0.1:8080/waapi";

    let mut client = match waapi_client(url) {
        Ok(client) => client,
        Err(_error) => panic!("Error"),
    };

    let _response = client
        .call(URI::new(ak::wwise::core::GET_INFO), None, None)
        .await;

    let id = Value::String("{DC124660-8640-45DA-9871-7E3FB66E07D6}".to_string());

    let x = Value::String("id".to_string());
    let y = Value::String("type".to_string());
    let z = Value::String("name".to_string());

    let mut kwargs = HashMap::new();
    let mut ids = HashMap::new();
    ids.insert("id".to_string(), Value::List(vec![id]));
    kwargs.insert("from".to_string(), Value::Dict(ids));

    let mut options = HashMap::new();
    options.insert("return".to_string(), Value::List(vec![x, y, z]));

    let response = client
        .call_with_options(
            URI::new(ak::wwise::core::object::GET),
            None,
            Some(kwargs),
            options,
        )
        .await;

    match response {
        Ok(result) => {
            println!("Success! Response:");
            for (key, value) in &result.1 {
                println!("{}: {:?}", key, value);
            }
        }
        Err(e) => {
            println!("Error calling WAAPI: {:?}", e);
        }
    }

    client.shutdown().await.unwrap();
    println!("Disconnected");
}
