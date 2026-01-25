use std::collections::HashMap;
use wampire::{Connection, URI};

use crate::types::{ReturnType, WaapiArgs, WaapiOptions, WaapiValue};

fn make_url(ip: &str) -> String {
    format!("ws://{}/waapi", ip)
}

#[derive(Debug)]
pub enum WaapiError {
    ConnectionFailed(String),
    CallFailed(String),
    ShutdownNotCalled,
}

impl std::fmt::Display for WaapiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaapiError::ConnectionFailed(reason) => write!(f, "Connection failed: {}", reason),
            WaapiError::CallFailed(reason) => write!(f, "Call failed: {}", reason),
            WaapiError::ShutdownNotCalled => {
                write!(f, "shutdown() was not called before dropping WaapiClient")
            }
        }
    }
}

impl std::error::Error for WaapiError {}

pub type WaapiResult<T> = Result<T, WaapiError>;

#[derive(Debug)]
pub struct WaapiResponse {
    pub args: Vec<WaapiValue>,
    pub kwargs: HashMap<String, WaapiValue>,
}

impl WaapiResponse {
    pub fn print(&self) {
        println!("Response:");

        if !self.args.is_empty() {
            println!("\nArgs:");
            for (i, arg) in self.args.iter().enumerate() {
                println!("  [{}]: {:?}", i, arg);
            }
        }

        if !self.kwargs.is_empty() {
            println!("\nKwargs:");
            for (key, value) in &self.kwargs {
                println!("  {}: {:?}", key, value);
            }
        }
    }

    /// Get a value from kwargs by key
    pub fn get(&self, key: &str) -> Option<&WaapiValue> {
        self.kwargs.get(key)
    }

    /// Get the first object from the "return" list (common pattern in WAAPI)
    pub fn get_return_object(&self) -> Option<&HashMap<String, WaapiValue>> {
        if let Some(WaapiValue::List(list)) = self.kwargs.get("return") {
            if let Some(WaapiValue::Dict(dict)) = list.first() {
                return Some(dict);
            }
        } else if let Some(WaapiValue::List(list)) = self.kwargs.get("objects") {
            if let Some(WaapiValue::Dict(dict)) = list.first() {
                return Some(dict);
            }
        }
        None
    }

    /// Get all objects from the "return" list
    pub fn get_return_objects(&self) -> Vec<&HashMap<String, WaapiValue>> {
        if let Some(WaapiValue::List(list)) = self.kwargs.get("return") {
            return list
                .iter()
                .filter_map(|v| {
                    if let WaapiValue::Dict(dict) = v {
                        Some(dict)
                    } else {
                        None
                    }
                })
                .collect();
        } else if let Some(WaapiValue::List(list)) = self.kwargs.get("objects") {
            return list
                .iter()
                .filter_map(|v| {
                    if let WaapiValue::Dict(dict) = v {
                        Some(dict)
                    } else {
                        None
                    }
                })
                .collect();
        }
        Vec::new()
    }

    pub fn get_return_string(&self, field: &str) -> Option<&str> {
        self.get_return_object().and_then(|obj| {
            if let Some(WaapiValue::String(s)) = obj.get(field) {
                Some(s.as_str())
            } else {
                None
            }
        })
    }

    pub fn get_return_value(&self, field: ReturnType) -> Option<WaapiValue> {
        self.get_return_object()
            .and_then(|obj| obj.get(field.as_str()))
            .cloned()
    }
}

pub struct WaapiClient {
    wamp_client: wampire::Client,
    shutdown_called: bool,
}

impl WaapiClient {
    pub fn new(url: Option<&str>) -> WaapiResult<WaapiClient> {
        let connection_url = make_url(url.unwrap_or("127.0.0.1:8080"));
        let connection = Connection::new(&connection_url, "waapi_client");
        let client = connection
            .connect()
            .map_err(|e| WaapiError::ConnectionFailed(format!("{:?}", e)))?;

        Ok(Self {
            wamp_client: client,
            shutdown_called: false,
        })
    }

    pub async fn call(
        &mut self,
        url: &str,
        args: Option<WaapiArgs>,
        options: Option<WaapiOptions>,
    ) -> WaapiResult<WaapiResponse> {
        // Convert WaapiArgs and WaapiOptions to wampire types
        let wamp_kwargs: Option<HashMap<String, wampire::Value>> = args.map(|a| {
            a.into_hashmap()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect()
        });

        let wamp_options: Option<HashMap<String, wampire::Value>> = options.map(|opts| {
            opts.to_hashmap()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect()
        });

        let result = if let Some(opt) = wamp_options {
            self.wamp_client
                .call_with_options(URI::new(url), None, wamp_kwargs, opt)
                .await
        } else {
            self.wamp_client
                .call(URI::new(url), None, wamp_kwargs)
                .await
        };

        match result {
            Ok((args, kwargs)) => Ok(WaapiResponse {
                args: args.into_iter().map(|v| v.into()).collect(),
                kwargs: kwargs.into_iter().map(|(k, v)| (k, v.into())).collect(),
            }),
            Err(e) => Err(WaapiError::CallFailed(format!("{:?}", e.get_reason()))),
        }
    }

    pub async fn shutdown(mut self) -> WaapiResult<()> {
        self.shutdown_called = true;
        self.wamp_client
            .shutdown()
            .await
            .map_err(|e| WaapiError::ConnectionFailed(format!("{:?}", e)))
    }
}

impl Drop for WaapiClient {
    fn drop(&mut self) {
        if !self.shutdown_called {
            eprintln!("Warning: {}", WaapiError::ShutdownNotCalled);
        }
    }
}
