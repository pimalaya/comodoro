//! The JSON-RPC 2.0 envelope, as defined by the [specification].
//!
//! This module carries the transport-agnostic half of the protocol:
//! the request, response and error shapes, and nothing about how bytes
//! move. The specification deliberately leaves framing to the
//! application, and Comodoro frames with NDJSON, one compact JSON
//! value per line. Compact JSON contains no raw newline, so a line
//! break is an unambiguous separator.
//!
//! The Comodoro method surface built on top of these types lives in
//! [`crate::protocol`].
//!
//! [specification]: https://www.jsonrpc.org/specification

use core::fmt;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Unexpected},
};
use serde_json::Value;

/// The only protocol version this module speaks.
pub const JSONRPC20_VERSION: &str = "2.0";

/// The `jsonrpc` member, which the specification pins to the exact
/// string `2.0`.
///
/// Modelled as a type rather than a plain string so a peer speaking
/// another version is rejected at deserialization instead of halfway
/// through dispatch.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Jsonrpc20Version;

impl Serialize for Jsonrpc20Version {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(JSONRPC20_VERSION)
    }
}

impl<'de> Deserialize<'de> for Jsonrpc20Version {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let version = String::deserialize(deserializer)?;

        if version != JSONRPC20_VERSION {
            let unexpected = Unexpected::Str(&version);
            return Err(D::Error::invalid_value(unexpected, &JSONRPC20_VERSION));
        }

        Ok(Self)
    }
}

/// The identifier correlating a request with its response.
///
/// The specification allows a string or a number, and discourages
/// fractional numbers. A request carrying no identifier at all is a
/// notification, which is modelled by the absence of
/// [`Jsonrpc20Request::id`] rather than by a variant here.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Jsonrpc20Id {
    /// A numeric identifier, the usual choice for a counter.
    Number(i64),
    /// A string identifier.
    String(String),
}

impl fmt::Display for Jsonrpc20Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => f.write_str(s),
        }
    }
}

impl From<i64> for Jsonrpc20Id {
    fn from(id: i64) -> Self {
        Self::Number(id)
    }
}

/// A call from a client to a server.
///
/// The request is a notification when [`Self::id`] is absent, which
/// tells the server to run the method and send nothing back. Comodoro
/// uses notifications in the server-to-client direction only, to push
/// timer events to subscribers.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Jsonrpc20Request {
    /// The protocol version tag.
    pub jsonrpc: Jsonrpc20Version,
    /// The name of the method to invoke.
    pub method: String,
    /// The method arguments, by name or by position.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    /// The correlation identifier, absent on a notification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Jsonrpc20Id>,
}

impl Jsonrpc20Request {
    /// Builds a call expecting a response.
    pub fn new(method: impl ToString, params: Option<Value>, id: impl Into<Jsonrpc20Id>) -> Self {
        Self {
            jsonrpc: Jsonrpc20Version,
            method: method.to_string(),
            params,
            id: Some(id.into()),
        }
    }

    /// Builds a notification, which expects no response.
    pub fn notification(method: impl ToString, params: Option<Value>) -> Self {
        Self {
            jsonrpc: Jsonrpc20Version,
            method: method.to_string(),
            params,
            id: None,
        }
    }

    /// Whether this request is a notification.
    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}

/// The outcome of a call, either a result or an error but never both.
///
/// Flattened into [`Jsonrpc20Response`], so the variant name becomes
/// the `result` or `error` member. A response carrying both is
/// unrepresentable here, so this side of the specification's
/// exclusivity rule is enforced by the type. Reading is deliberately
/// lenient in the other direction: a malformed response carrying both
/// members parses as its `result`, since failing to parse a peer's
/// answer helps nobody.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Jsonrpc20Outcome {
    /// The method succeeded and returned this value.
    Result(Value),
    /// The method failed with this error.
    Error(Jsonrpc20Error),
}

/// An answer from a server to a client.
///
/// The identifier is echoed from the request, and is null when the
/// request could not be parsed well enough to recover one.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Jsonrpc20Response {
    /// The protocol version tag.
    pub jsonrpc: Jsonrpc20Version,
    /// The result or the error.
    #[serde(flatten)]
    pub outcome: Jsonrpc20Outcome,
    /// The identifier of the request being answered.
    pub id: Option<Jsonrpc20Id>,
}

impl Jsonrpc20Response {
    /// Builds a successful answer to the request carrying `id`.
    pub fn result(value: Value, id: Option<Jsonrpc20Id>) -> Self {
        Self {
            jsonrpc: Jsonrpc20Version,
            outcome: Jsonrpc20Outcome::Result(value),
            id,
        }
    }

    /// Builds a failed answer to the request carrying `id`.
    pub fn error(error: Jsonrpc20Error, id: Option<Jsonrpc20Id>) -> Self {
        Self {
            jsonrpc: Jsonrpc20Version,
            outcome: Jsonrpc20Outcome::Error(error),
            id,
        }
    }
}

/// The reason a call failed.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Jsonrpc20Error {
    /// The error code, one of the reserved codes below or a
    /// server-defined one.
    pub code: i64,
    /// A single sentence describing the failure.
    pub message: String,
    /// Optional structured detail about the failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Jsonrpc20Error {
    /// Invalid JSON was received.
    pub const PARSE: i64 = -32700;
    /// The JSON received is not a valid request object.
    pub const INVALID_REQUEST: i64 = -32600;
    /// The requested method does not exist.
    pub const METHOD_NOT_FOUND: i64 = -32601;
    /// The parameters do not match the method signature.
    pub const INVALID_PARAMS: i64 = -32602;
    /// The server failed for an internal reason.
    pub const INTERNAL: i64 = -32603;
    /// The first code of the range reserved for server-defined errors.
    pub const SERVER_MIN: i64 = -32099;
    /// The last code of the range reserved for server-defined errors.
    pub const SERVER_MAX: i64 = -32000;

    /// Builds an error with an arbitrary code and message.
    pub fn new(code: i64, message: impl ToString) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
        }
    }

    /// Attaches structured detail to the error.
    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }

    /// Builds a [`Self::PARSE`] error.
    pub fn parse(message: impl ToString) -> Self {
        Self::new(Self::PARSE, message)
    }

    /// Builds an [`Self::INVALID_REQUEST`] error.
    pub fn invalid_request(message: impl ToString) -> Self {
        Self::new(Self::INVALID_REQUEST, message)
    }

    /// Builds a [`Self::METHOD_NOT_FOUND`] error naming the method.
    pub fn method_not_found(method: &str) -> Self {
        Self::new(Self::METHOD_NOT_FOUND, format!("Unknown method `{method}`"))
    }

    /// Builds an [`Self::INVALID_PARAMS`] error.
    pub fn invalid_params(message: impl ToString) -> Self {
        Self::new(Self::INVALID_PARAMS, message)
    }

    /// Builds an [`Self::INTERNAL`] error.
    pub fn internal(message: impl ToString) -> Self {
        Self::new(Self::INTERNAL, message)
    }
}

impl fmt::Display for Jsonrpc20Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self.message, self.code)
    }
}

/// One request or a batch of them, as a peer may send either.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Jsonrpc20Incoming {
    /// A lone request or notification.
    Single(Jsonrpc20Request),
    /// Several requests to run in one round trip.
    Batch(Vec<Jsonrpc20Request>),
}

/// One response or a batch of them, mirroring [`Jsonrpc20Incoming`].
///
/// A batch made only of notifications produces no response at all,
/// which the specification requires and which callers signal by
/// sending nothing rather than by an empty batch.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Jsonrpc20Outgoing {
    /// A lone response.
    Single(Jsonrpc20Response),
    /// The responses to a batch, in any order.
    Batch(Vec<Jsonrpc20Response>),
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use serde_json::json;

    use crate::jsonrpc20::*;

    #[test]
    fn request_round_trips() {
        let line = r#"{"jsonrpc":"2.0","method":"timer.set","params":{"duration":60},"id":1}"#;
        let request: Jsonrpc20Request = serde_json::from_str(line).unwrap();

        assert_eq!(request.method, "timer.set");
        assert_eq!(request.id, Some(Jsonrpc20Id::Number(1)));
        assert!(!request.is_notification());
        assert_eq!(serde_json::to_string(&request).unwrap(), line);
    }

    #[test]
    fn notification_omits_id() {
        let notification = Jsonrpc20Request::notification("timer.running", None);
        let line = serde_json::to_string(&notification).unwrap();

        assert_eq!(line, r#"{"jsonrpc":"2.0","method":"timer.running"}"#);
        assert!(notification.is_notification());
    }

    #[test]
    fn wrong_version_is_rejected() {
        let line = r#"{"jsonrpc":"1.0","method":"timer.get","id":1}"#;
        let err = serde_json::from_str::<Jsonrpc20Request>(line).unwrap_err();

        assert!(err.to_string().contains("2.0"), "{err}");
    }

    #[test]
    fn result_and_error_are_exclusive() {
        let ok = Jsonrpc20Response::result(json!("pong"), Some(1.into()));
        assert_eq!(
            serde_json::to_string(&ok).unwrap(),
            r#"{"jsonrpc":"2.0","result":"pong","id":1}"#
        );

        let ko = Jsonrpc20Response::error(Jsonrpc20Error::method_not_found("nope"), None);
        assert_eq!(
            serde_json::to_string(&ko).unwrap(),
            r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"Unknown method `nope`"},"id":null}"#
        );

        // NOTE: writing both members is unrepresentable, but reading
        // one that does resolves to the result rather than failing.
        let both = r#"{"jsonrpc":"2.0","result":1,"error":{"code":-1,"message":"x"},"id":1}"#;
        let lenient: Jsonrpc20Response = serde_json::from_str(both).unwrap();
        assert!(matches!(lenient.outcome, Jsonrpc20Outcome::Result(_)));
    }

    #[test]
    fn batch_and_single_are_both_accepted() {
        let single = r#"{"jsonrpc":"2.0","method":"timer.get","id":1}"#;
        assert!(matches!(
            serde_json::from_str::<Jsonrpc20Incoming>(single).unwrap(),
            Jsonrpc20Incoming::Single(_)
        ));

        let batch = r#"[{"jsonrpc":"2.0","method":"timer.get","id":1},{"jsonrpc":"2.0","method":"timer.start","id":2}]"#;
        let Jsonrpc20Incoming::Batch(requests) = serde_json::from_str(batch).unwrap() else {
            panic!("expected a batch");
        };
        assert_eq!(requests.len(), 2);
    }

    #[test]
    fn string_ids_are_preserved() {
        let line = r#"{"jsonrpc":"2.0","result":null,"id":"abc"}"#;
        let response: Jsonrpc20Response = serde_json::from_str(line).unwrap();

        assert_eq!(response.id, Some(Jsonrpc20Id::String(String::from("abc"))));
        assert_eq!(serde_json::to_string(&response).unwrap(), line);
    }

    #[test]
    fn error_data_is_optional() {
        let bare = Jsonrpc20Error::internal("boom");
        assert_eq!(
            serde_json::to_string(&bare).unwrap(),
            r#"{"code":-32603,"message":"boom"}"#
        );

        let detailed = Jsonrpc20Error::invalid_params("bad").with_data(json!(["duration"]));
        assert_eq!(
            serde_json::to_string(&detailed).unwrap(),
            r#"{"code":-32602,"message":"bad","data":["duration"]}"#
        );

        let _ = vec![bare, detailed];
    }
}
