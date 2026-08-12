//! The Comodoro method surface, expressed in JSON-RPC 2.0.
//!
//! This module is the contract between a Comodoro server and any
//! client, in this repository or not. It names the methods a server
//! answers, the notifications it pushes to subscribers, and the shape
//! of every parameter and result. The envelope carrying them lives in
//! [`crate::jsonrpc20`], and the framing is NDJSON, one compact JSON
//! value per line.
//!
//! Requests are named after the imperative that performs them
//! (`timer.start`), notifications after the past tense of what just
//! happened (`timer.started`), so the two directions never collide.
//!
//! A notification identifies its event by its method name, and carries
//! only the cycle the event concerns, since repeating the kind in the
//! parameters would be redundant. A result cannot do that, having no
//! method name of its own, so the events inside a result carry their
//! own `event` discriminator.
//!
//! Codes -32000 to -32099 are reserved by the specification for
//! server-defined errors. Comodoro defines none today: every failure
//! it can report is one of the standard codes.

use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    jsonrpc20::{Jsonrpc20Error, Jsonrpc20Id, Jsonrpc20Request},
    timer::{Timer, TimerCycle, TimerEvent},
};

/// A method call a client sends to a timer server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimerRequest {
    /// Returns the timer without touching it.
    Get,
    /// Starts the timer from its first cycle.
    Start,
    /// Pauses the timer, keeping the elapsed time.
    Pause,
    /// Resumes a paused timer.
    Resume,
    /// Stops the timer and resets it.
    Stop,
    /// Overrides the remaining duration of the current cycle.
    ///
    /// Clamped to the configured length of that cycle, and ignored
    /// entirely on a stopped timer. The resulting event carries the
    /// effective duration.
    Set {
        /// The new remaining duration, in seconds.
        duration: usize,
    },
    /// Subscribes this connection to the timer notifications.
    Subscribe,
    /// Stops sending timer notifications on this connection.
    Unsubscribe,
}

impl TimerRequest {
    /// Method name of [`Self::Get`].
    pub const GET: &'static str = "timer.get";
    /// Method name of [`Self::Start`].
    pub const START: &'static str = "timer.start";
    /// Method name of [`Self::Pause`].
    pub const PAUSE: &'static str = "timer.pause";
    /// Method name of [`Self::Resume`].
    pub const RESUME: &'static str = "timer.resume";
    /// Method name of [`Self::Stop`].
    pub const STOP: &'static str = "timer.stop";
    /// Method name of [`Self::Set`].
    pub const SET: &'static str = "timer.set";
    /// Method name of [`Self::Subscribe`].
    pub const SUBSCRIBE: &'static str = "timer.subscribe";
    /// Method name of [`Self::Unsubscribe`].
    pub const UNSUBSCRIBE: &'static str = "timer.unsubscribe";

    /// The method name carrying this request on the wire.
    pub fn method(&self) -> &'static str {
        match self {
            Self::Get => Self::GET,
            Self::Start => Self::START,
            Self::Pause => Self::PAUSE,
            Self::Resume => Self::RESUME,
            Self::Stop => Self::STOP,
            Self::Set { .. } => Self::SET,
            Self::Subscribe => Self::SUBSCRIBE,
            Self::Unsubscribe => Self::UNSUBSCRIBE,
        }
    }

    /// The parameters carrying this request on the wire.
    pub fn params(&self) -> Option<Value> {
        match self {
            Self::Set { duration } => Some(json!({ "duration": duration })),
            _ => None,
        }
    }

    /// Wraps this request in a JSON-RPC envelope.
    pub fn into_jsonrpc(self, id: impl Into<Jsonrpc20Id>) -> Jsonrpc20Request {
        Jsonrpc20Request::new(self.method(), self.params(), id)
    }

    /// Reads a request back from a JSON-RPC envelope.
    ///
    /// Returns the error the server should answer with when the method
    /// is unknown or the parameters do not match it.
    pub fn from_jsonrpc(request: &Jsonrpc20Request) -> Result<Self, Jsonrpc20Error> {
        match request.method.as_str() {
            Self::GET => Ok(Self::Get),
            Self::START => Ok(Self::Start),
            Self::PAUSE => Ok(Self::Pause),
            Self::RESUME => Ok(Self::Resume),
            Self::STOP => Ok(Self::Stop),
            Self::SUBSCRIBE => Ok(Self::Subscribe),
            Self::UNSUBSCRIBE => Ok(Self::Unsubscribe),
            Self::SET => {
                let params = request.params.clone().unwrap_or(Value::Null);
                let params: TimerSetParams = serde_json::from_value(params).map_err(|err| {
                    Jsonrpc20Error::invalid_params(err).with_data(json!(["duration"]))
                })?;
                Ok(Self::Set {
                    duration: params.duration,
                })
            }
            method => Err(Jsonrpc20Error::method_not_found(method)),
        }
    }
}

/// Parameters of the [`TimerRequest::Set`] method.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TimerSetParams {
    /// The new remaining duration of the current cycle, in seconds.
    pub duration: usize,
}

/// The result a server returns for a [`TimerRequest`].
#[derive(Clone, Debug, PartialEq)]
pub enum TimerResponse {
    /// The timer, answering [`TimerRequest::Get`].
    Timer(Timer),
    /// The events the call made the timer emit, possibly none.
    Events(Vec<TimerEvent>),
    /// Whether the connection is now subscribed to notifications.
    Subscription(bool),
}

impl TimerResponse {
    /// Encodes this result as the JSON-RPC `result` member.
    pub fn to_value(&self) -> Value {
        match self {
            Self::Timer(timer) => json!(timer),
            Self::Events(events) => json!({ "events": events }),
            Self::Subscription(subscribed) => json!({ "subscribed": subscribed }),
        }
    }

    /// Decodes the `result` member of the answer to `request`.
    ///
    /// The shape depends on the method that was called, since JSON-RPC
    /// results carry no discriminator of their own.
    pub fn from_value(request: &TimerRequest, value: Value) -> Result<Self, serde_json::Error> {
        match request {
            TimerRequest::Get => serde_json::from_value(value).map(Self::Timer),
            TimerRequest::Subscribe | TimerRequest::Unsubscribe => {
                let params: TimerSubscriptionResult = serde_json::from_value(value)?;
                Ok(Self::Subscription(params.subscribed))
            }
            _ => {
                let result: TimerEventsResult = serde_json::from_value(value)?;
                Ok(Self::Events(result.events))
            }
        }
    }
}

/// The result of every method that can make the timer emit events.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TimerEventsResult {
    /// The events emitted, in the order the timer produced them.
    pub events: Vec<TimerEvent>,
}

/// The result of [`TimerRequest::Subscribe`] and
/// [`TimerRequest::Unsubscribe`].
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TimerSubscriptionResult {
    /// Whether the connection now receives timer notifications.
    pub subscribed: bool,
}

/// Parameters of every notification concerning a cycle.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TimerCycleParams {
    /// The cycle the notification is about.
    pub cycle: TimerCycle,
}

/// Protocol mapping of the events a timer emits.
///
/// Lives here rather than next to [`TimerEvent`] because it is a wire
/// concern: the timer itself knows nothing about JSON-RPC.
impl TimerEvent {
    /// Notification method name of [`Self::Started`].
    pub const STARTED: &'static str = "timer.started";
    /// Notification method name of [`Self::Began`].
    pub const BEGAN: &'static str = "timer.began";
    /// Notification method name of [`Self::Running`].
    pub const RUNNING: &'static str = "timer.running";
    /// Notification method name of [`Self::Set`].
    pub const DURATION_SET: &'static str = "timer.durationSet";
    /// Notification method name of [`Self::Paused`].
    pub const PAUSED: &'static str = "timer.paused";
    /// Notification method name of [`Self::Resumed`].
    pub const RESUMED: &'static str = "timer.resumed";
    /// Notification method name of [`Self::Ended`].
    pub const ENDED: &'static str = "timer.ended";
    /// Notification method name of [`Self::Stopped`].
    pub const STOPPED: &'static str = "timer.stopped";

    /// The notification method name carrying this event.
    pub fn method(&self) -> &'static str {
        match self {
            Self::Started => Self::STARTED,
            Self::Began(_) => Self::BEGAN,
            Self::Running(_) => Self::RUNNING,
            Self::Set(_) => Self::DURATION_SET,
            Self::Paused(_) => Self::PAUSED,
            Self::Resumed(_) => Self::RESUMED,
            Self::Ended(_) => Self::ENDED,
            Self::Stopped => Self::STOPPED,
        }
    }

    /// The cycle this event concerns, when it concerns one.
    pub fn cycle(&self) -> Option<&TimerCycle> {
        match self {
            Self::Started | Self::Stopped => None,
            Self::Began(cycle)
            | Self::Running(cycle)
            | Self::Set(cycle)
            | Self::Paused(cycle)
            | Self::Resumed(cycle)
            | Self::Ended(cycle) => Some(cycle),
        }
    }

    /// Wraps this event in the notification a server pushes.
    pub fn into_notification(self) -> Jsonrpc20Request {
        let method = self.method();
        let params = self.cycle().map(|cycle| json!({ "cycle": cycle }));
        Jsonrpc20Request::notification(method, params)
    }

    /// Reads an event back from a notification a server pushed.
    ///
    /// Returns `None` when the method is not a timer notification, so a
    /// client can ignore what it does not understand rather than fail.
    pub fn from_notification(request: &Jsonrpc20Request) -> Option<Self> {
        let cycle = || -> Option<TimerCycle> {
            let params = request.params.clone()?;
            let params: TimerCycleParams = serde_json::from_value(params).ok()?;
            Some(params.cycle)
        };

        match request.method.as_str() {
            Self::STARTED => Some(Self::Started),
            Self::STOPPED => Some(Self::Stopped),
            Self::BEGAN => cycle().map(Self::Began),
            Self::RUNNING => cycle().map(Self::Running),
            Self::DURATION_SET => cycle().map(Self::Set),
            Self::PAUSED => cycle().map(Self::Paused),
            Self::RESUMED => cycle().map(Self::Resumed),
            Self::ENDED => cycle().map(Self::Ended),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::{string::ToString, vec};

    use crate::protocol::*;

    #[test]
    fn request_round_trips_through_jsonrpc() {
        for request in [
            TimerRequest::Get,
            TimerRequest::Start,
            TimerRequest::Pause,
            TimerRequest::Resume,
            TimerRequest::Stop,
            TimerRequest::Set { duration: 60 },
            TimerRequest::Subscribe,
            TimerRequest::Unsubscribe,
        ] {
            let envelope = request.clone().into_jsonrpc(1);
            let parsed = TimerRequest::from_jsonrpc(&envelope).unwrap();
            assert_eq!(parsed, request);
        }
    }

    #[test]
    fn set_wire_shape_is_stable() {
        let line =
            serde_json::to_string(&TimerRequest::Set { duration: 90 }.into_jsonrpc(7)).unwrap();
        assert_eq!(
            line,
            r#"{"jsonrpc":"2.0","method":"timer.set","params":{"duration":90},"id":7}"#
        );
    }

    #[test]
    fn unknown_method_is_method_not_found() {
        let envelope = Jsonrpc20Request::new("timer.explode", None, 1);
        let err = TimerRequest::from_jsonrpc(&envelope).unwrap_err();
        assert_eq!(err.code, Jsonrpc20Error::METHOD_NOT_FOUND);
    }

    #[test]
    fn set_without_duration_is_invalid_params() {
        let envelope = Jsonrpc20Request::new(TimerRequest::SET, None, 1);
        let err = TimerRequest::from_jsonrpc(&envelope).unwrap_err();
        assert_eq!(err.code, Jsonrpc20Error::INVALID_PARAMS);
    }

    #[test]
    fn notification_round_trips() {
        for event in [
            TimerEvent::Started,
            TimerEvent::Stopped,
            TimerEvent::Began(TimerCycle::new("Work", 1500)),
            TimerEvent::Running(TimerCycle::new("Work", 1499)),
            TimerEvent::Set(TimerCycle::new("Work", 60)),
            TimerEvent::Paused(TimerCycle::new("Work", 42)),
            TimerEvent::Resumed(TimerCycle::new("Work", 42)),
            TimerEvent::Ended(TimerCycle::new("Work", 0)),
        ] {
            let notification = event.clone().into_notification();
            assert!(notification.is_notification());
            assert_eq!(TimerEvent::from_notification(&notification), Some(event));
        }
    }

    #[test]
    fn notification_wire_shape_is_stable() {
        // NOTE: members of a JSON object built through serde_json::Value
        // come out in alphabetical order, since Value maps are backed by
        // a BTreeMap. Object member order carries no meaning in JSON, so
        // this is stable rather than canonical.
        let began = TimerEvent::Began(TimerCycle::new("Work", 1500)).into_notification();
        assert_eq!(
            serde_json::to_string(&began).unwrap(),
            r#"{"jsonrpc":"2.0","method":"timer.began","params":{"cycle":{"duration":1500,"name":"Work"}}}"#
        );

        let started = TimerEvent::Started.into_notification();
        assert_eq!(
            serde_json::to_string(&started).unwrap(),
            r#"{"jsonrpc":"2.0","method":"timer.started"}"#
        );
    }

    #[test]
    fn unknown_notification_is_ignored() {
        let other = Jsonrpc20Request::notification("editor.opened", None);
        assert_eq!(TimerEvent::from_notification(&other), None);
    }

    #[test]
    fn results_round_trip_per_method() {
        let events = TimerResponse::Events(vec![TimerEvent::Started]);
        let value = events.to_value();
        assert_eq!(value.to_string(), r#"{"events":[{"event":"started"}]}"#);
        assert_eq!(
            TimerResponse::from_value(&TimerRequest::Start, value).unwrap(),
            events
        );

        let subscribed = TimerResponse::Subscription(true);
        let value = subscribed.to_value();
        assert_eq!(value.to_string(), r#"{"subscribed":true}"#);
        assert_eq!(
            TimerResponse::from_value(&TimerRequest::Subscribe, value).unwrap(),
            subscribed
        );
    }

    #[test]
    fn events_inside_a_result_carry_their_kind() {
        let value = TimerResponse::Events(vec![
            TimerEvent::Ended(TimerCycle::new("Work", 0)),
            TimerEvent::Stopped,
        ])
        .to_value();

        assert_eq!(
            value.to_string(),
            r#"{"events":[{"cycle":{"duration":0,"name":"Work"},"event":"ended"},{"event":"stopped"}]}"#
        );
    }

    #[test]
    fn error_displays_its_code() {
        let err = Jsonrpc20Error::method_not_found("timer.nope");
        assert_eq!(err.to_string(), "Unknown method `timer.nope` (code -32601)");
    }
}
