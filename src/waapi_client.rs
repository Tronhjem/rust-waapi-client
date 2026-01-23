use wampire::{Connection};

pub struct WaapiClient {
    wamp_client: wampire::Client
}

impl WaapiClient {
    pub fn new(uri: Option<String>) -> Option<WaapiClient> {

        let connection_url = uri.unwrap_or_else(|| "ws://127.0.0.1:8080/waapi".to_string());

        println!("Connecting...");

        let connection = Connection::new(&connection_url, "waapi_client");
        let client = connection.connect();

        match client {
            Ok(client) => {
                println!("Client connected to {}", connection_url);
                Some(Self {wamp_client: client})
            }
            Err(_e) => {
                println!("Coulnd't connect to {}", connection_url);
                None
            }
        }
    }

    pub async fn call(&self, _uri: String) {

    }

    pub async fn shutdown(&mut self) {
        if let Ok(_res) = self.wamp_client.shutdown().await {
            println!("Disconnected!");
        }
    }
}

impl Drop for WaapiClient {
    fn drop(&mut self) {

    }
}
