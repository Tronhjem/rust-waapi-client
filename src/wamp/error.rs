use std::fmt;

use serde_json::Value;

pub type WampResult<T> = std::result::Result<T, WampError>;

#[derive(Debug)]
pub enum WampError {
    WebSocket(String),
    Serialize(serde_json::Error),
    Deserialize(serde_json::Error),
    UnexpectedMessage(String),
    ConnectionClosed,
    Shutdown,
    CallError {
        uri: String,
        args: Vec<Value>,
        kwargs: Value,
    },
}

impl fmt::Display for WampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WampError::WebSocket(msg) => write!(f, "websocket error: {msg}"),
            WampError::Serialize(e) => write!(f, "serialize error: {e}"),
            WampError::Deserialize(e) => write!(f, "deserialize error: {e}"),
            WampError::UnexpectedMessage(msg) => write!(f, "unexpected message: {msg}"),
            WampError::ConnectionClosed => write!(f, "connection closed"),
            WampError::Shutdown => write!(f, "client shutdown"),
            WampError::CallError { uri, .. } => write!(f, "WAMP error for {uri}"),
        }
    }
}

impl std::error::Error for WampError {}

impl From<serde_json::Error> for WampError {
    fn from(err: serde_json::Error) -> Self {
        WampError::Deserialize(err)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for WampError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        WampError::WebSocket(err.to_string())
    }
}
