use std::fmt;

use serde_json::Value;

pub type WampResult<T> = std::result::Result<T, WampError>;

/// Low-level WAMP protocol and WebSocket transport errors.
#[derive(Debug)]
pub enum WampError {
    /// WebSocket connection or frame error.
    WebSocket(String),
    /// Failed to serialize an outbound WAMP message.
    Serialize(serde_json::Error),
    /// Failed to deserialize an inbound WAMP message.
    Deserialize(serde_json::Error),
    /// Received an unexpected WAMP message during handshake or session.
    UnexpectedMessage(String),
    /// The underlying WebSocket connection was closed.
    ConnectionClosed,
    /// The WAMP client has already been shut down.
    Shutdown,
    /// The WAMP router/peer returned an `ERROR` response for a request.
    CallError {
        /// Error URI returned by the WAMP peer.
        uri: String,
        /// Positional error arguments.
        args: Vec<Value>,
        /// Keyword error details.
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
