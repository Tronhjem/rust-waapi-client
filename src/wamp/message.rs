use serde::de::{self, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

pub type SessionId = u64;
pub type RequestId = u64;
pub type SubscriptionId = u64;

const HELLO: u8 = 1;
const WELCOME: u8 = 2;
const ABORT: u8 = 3;
const GOODBYE: u8 = 6;
const ERROR: u8 = 8;
const SUBSCRIBE: u8 = 32;
const SUBSCRIBED: u8 = 33;
const UNSUBSCRIBE: u8 = 34;
const UNSUBSCRIBED: u8 = 35;
const EVENT: u8 = 36;
const CALL: u8 = 48;
const RESULT: u8 = 50;

#[derive(Debug, Clone)]
pub enum Message {
    Hello(String, Value),
    Welcome(SessionId, Value),
    Abort(Value, String),
    Goodbye(Value, String),
    Error(u8, RequestId, Value, String, Vec<Value>, Value),
    Call(RequestId, Value, String, Vec<Value>, Value),
    Result(RequestId, Value, Vec<Value>, Value),
    Subscribe(RequestId, Value, String),
    Subscribed(RequestId, SubscriptionId),
    Unsubscribe(RequestId, SubscriptionId),
    Unsubscribed(RequestId),
    Event(SubscriptionId, u64, Value, Vec<Value>, Value),
}

impl Message {
    pub fn hello(realm: impl Into<String>) -> Self {
        let mut roles = serde_json::Map::new();
        roles.insert("caller".to_string(), Value::Object(serde_json::Map::new()));
        roles.insert(
            "subscriber".to_string(),
            Value::Object(serde_json::Map::new()),
        );
        let details = serde_json::json!({ "roles": roles });
        Message::Hello(realm.into(), details)
    }
}

impl Serialize for Message {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Message::Hello(realm, details) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(&HELLO)?;
                seq.serialize_element(realm)?;
                seq.serialize_element(details)?;
                seq.end()
            }
            Message::Welcome(session_id, details) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(&WELCOME)?;
                seq.serialize_element(session_id)?;
                seq.serialize_element(details)?;
                seq.end()
            }
            Message::Abort(details, reason) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(&ABORT)?;
                seq.serialize_element(details)?;
                seq.serialize_element(reason)?;
                seq.end()
            }
            Message::Goodbye(details, reason) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(&GOODBYE)?;
                seq.serialize_element(details)?;
                seq.serialize_element(reason)?;
                seq.end()
            }
            Message::Error(request_type, request_id, details, error, args, kwargs) => {
                let mut seq = serializer.serialize_seq(Some(7))?;
                seq.serialize_element(&ERROR)?;
                seq.serialize_element(request_type)?;
                seq.serialize_element(request_id)?;
                seq.serialize_element(details)?;
                seq.serialize_element(error)?;
                seq.serialize_element(args)?;
                seq.serialize_element(kwargs)?;
                seq.end()
            }
            Message::Call(request_id, options, procedure, args, kwargs) => {
                let has_kwargs = !kwargs.is_null();
                let has_args = !args.is_empty() || has_kwargs;
                let len = 4 + usize::from(has_args) + usize::from(has_kwargs);
                let mut seq = serializer.serialize_seq(Some(len))?;
                seq.serialize_element(&CALL)?;
                seq.serialize_element(request_id)?;
                seq.serialize_element(options)?;
                seq.serialize_element(procedure)?;
                if has_args {
                    seq.serialize_element(args)?;
                }
                if has_kwargs {
                    seq.serialize_element(kwargs)?;
                }
                seq.end()
            }
            Message::Result(request_id, details, args, kwargs) => {
                let mut seq = serializer.serialize_seq(Some(5))?;
                seq.serialize_element(&RESULT)?;
                seq.serialize_element(request_id)?;
                seq.serialize_element(details)?;
                seq.serialize_element(args)?;
                seq.serialize_element(kwargs)?;
                seq.end()
            }
            Message::Subscribe(request_id, options, topic) => {
                let mut seq = serializer.serialize_seq(Some(4))?;
                seq.serialize_element(&SUBSCRIBE)?;
                seq.serialize_element(request_id)?;
                seq.serialize_element(options)?;
                seq.serialize_element(topic)?;
                seq.end()
            }
            Message::Subscribed(request_id, subscription_id) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(&SUBSCRIBED)?;
                seq.serialize_element(request_id)?;
                seq.serialize_element(subscription_id)?;
                seq.end()
            }
            Message::Unsubscribe(request_id, subscription_id) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(&UNSUBSCRIBE)?;
                seq.serialize_element(request_id)?;
                seq.serialize_element(subscription_id)?;
                seq.end()
            }
            Message::Unsubscribed(request_id) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&UNSUBSCRIBED)?;
                seq.serialize_element(request_id)?;
                seq.end()
            }
            Message::Event(subscription_id, publication_id, details, args, kwargs) => {
                let mut seq = serializer.serialize_seq(Some(6))?;
                seq.serialize_element(&EVENT)?;
                seq.serialize_element(subscription_id)?;
                seq.serialize_element(publication_id)?;
                seq.serialize_element(details)?;
                seq.serialize_element(args)?;
                seq.serialize_element(kwargs)?;
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_seq(MessageVisitor)
    }
}

struct MessageVisitor;

impl<'de> Visitor<'de> for MessageVisitor {
    type Value = Message;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a WAMP message array")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let code = seq
            .next_element::<u8>()?
            .ok_or_else(|| de::Error::custom("missing message type code"))?;

        match code {
            HELLO => {
                let realm = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing realm"))?;
                let details = seq.next_element::<Value>()?.unwrap_or(Value::Null);
                Ok(Message::Hello(realm, details))
            }
            WELCOME => {
                let session_id = seq
                    .next_element::<SessionId>()?
                    .ok_or_else(|| de::Error::custom("missing session id"))?;
                let details = seq.next_element::<Value>()?.unwrap_or(Value::Null);
                Ok(Message::Welcome(session_id, details))
            }
            ABORT => {
                let details = seq.next_element::<Value>()?.unwrap_or(Value::Null);
                let reason = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing abort reason"))?;
                Ok(Message::Abort(details, reason))
            }
            GOODBYE => {
                let details = seq.next_element::<Value>()?.unwrap_or(Value::Null);
                let reason = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing goodbye reason"))?;
                Ok(Message::Goodbye(details, reason))
            }
            ERROR => {
                let request_type = seq
                    .next_element::<u8>()?
                    .ok_or_else(|| de::Error::custom("missing error request type"))?;
                let request_id = seq
                    .next_element::<RequestId>()?
                    .ok_or_else(|| de::Error::custom("missing error request id"))?;
                let details = seq.next_element::<Value>()?.unwrap_or(Value::Null);
                let error = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing error uri"))?;
                let args = seq.next_element::<Vec<Value>>()?.unwrap_or_default();
                let kwargs = seq
                    .next_element::<Value>()?
                    .unwrap_or(Value::Object(serde_json::Map::new()));
                Ok(Message::Error(
                    request_type,
                    request_id,
                    details,
                    error,
                    args,
                    kwargs,
                ))
            }
            CALL => {
                let request_id = seq
                    .next_element::<RequestId>()?
                    .ok_or_else(|| de::Error::custom("missing call request id"))?;
                let options = seq.next_element::<Value>()?.unwrap_or(Value::Null);
                let procedure = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing procedure"))?;
                let args = seq.next_element::<Vec<Value>>()?.unwrap_or_default();
                let kwargs = seq
                    .next_element::<Value>()?
                    .unwrap_or(Value::Object(serde_json::Map::new()));
                Ok(Message::Call(request_id, options, procedure, args, kwargs))
            }
            RESULT => {
                let request_id = seq
                    .next_element::<RequestId>()?
                    .ok_or_else(|| de::Error::custom("missing result request id"))?;
                let details = seq.next_element::<Value>()?.unwrap_or(Value::Null);
                let args = seq.next_element::<Vec<Value>>()?.unwrap_or_default();
                let kwargs = seq
                    .next_element::<Value>()?
                    .unwrap_or(Value::Object(serde_json::Map::new()));
                Ok(Message::Result(request_id, details, args, kwargs))
            }
            SUBSCRIBE => {
                let request_id = seq
                    .next_element::<RequestId>()?
                    .ok_or_else(|| de::Error::custom("missing subscribe request id"))?;
                let options = seq.next_element::<Value>()?.unwrap_or(Value::Null);
                let topic = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing topic"))?;
                Ok(Message::Subscribe(request_id, options, topic))
            }
            SUBSCRIBED => {
                let request_id = seq
                    .next_element::<RequestId>()?
                    .ok_or_else(|| de::Error::custom("missing subscribed request id"))?;
                let subscription_id = seq
                    .next_element::<SubscriptionId>()?
                    .ok_or_else(|| de::Error::custom("missing subscription id"))?;
                Ok(Message::Subscribed(request_id, subscription_id))
            }
            UNSUBSCRIBE => {
                let request_id = seq
                    .next_element::<RequestId>()?
                    .ok_or_else(|| de::Error::custom("missing unsubscribe request id"))?;
                let subscription_id = seq
                    .next_element::<SubscriptionId>()?
                    .ok_or_else(|| de::Error::custom("missing subscription id"))?;
                Ok(Message::Unsubscribe(request_id, subscription_id))
            }
            UNSUBSCRIBED => {
                let request_id = seq
                    .next_element::<RequestId>()?
                    .ok_or_else(|| de::Error::custom("missing unsubscribed request id"))?;
                Ok(Message::Unsubscribed(request_id))
            }
            EVENT => {
                let subscription_id = seq
                    .next_element::<SubscriptionId>()?
                    .ok_or_else(|| de::Error::custom("missing event subscription id"))?;
                let publication_id = seq
                    .next_element::<u64>()?
                    .ok_or_else(|| de::Error::custom("missing publication id"))?;
                let details = seq.next_element::<Value>()?.unwrap_or(Value::Null);
                let args = seq.next_element::<Vec<Value>>()?.unwrap_or_default();
                let kwargs = seq
                    .next_element::<Value>()?
                    .unwrap_or(Value::Object(serde_json::Map::new()));
                Ok(Message::Event(
                    subscription_id,
                    publication_id,
                    details,
                    args,
                    kwargs,
                ))
            }
            _ => Err(de::Error::custom(format!("unknown message type {code}"))),
        }
    }
}

use std::fmt;
