//! # Blocking client
//!
//! Blocking timer client speaking JSON-RPC 2.0 over one connection.

use std::{
    collections::VecDeque,
    error, fmt,
    io::{BufRead, BufReader, Write},
};

use anyhow::{Context, Result, anyhow};
use log::{debug, trace};

use crate::{
    jsonrpc20::{Jsonrpc20Id, Jsonrpc20Outcome, Jsonrpc20Request, Jsonrpc20Response},
    protocol::{TimerRequest, TimerResponse},
    timer::{Timer, TimerEvent},
    transport::{TimerAddress, TimerStream, TimerTransportError},
};

/// Why a call to a timer server failed.
///
/// Attached to the error a [`TimerClient`] method returns, so a caller
/// tells the causes apart with `downcast_ref`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerClientError {
    /// No server answers at the address: none runs, or it listens
    /// elsewhere.
    Unreachable,
    /// The server closed the connection before answering.
    Closed,
    /// The server answered with a JSON-RPC error.
    Server,
    /// The server's answer cannot be read, as from a server of another
    /// Comodoro version.
    Mismatch,
}

impl fmt::Display for TimerClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Unreachable => "Cannot reach the timer server",
            Self::Closed => "Timer server closed the connection before answering",
            Self::Server => "Timer server refused the request",
            Self::Mismatch => {
                "Cannot read the timer server's answer, it may run another Comodoro version: restart it with this one"
            }
        })
    }
}

impl error::Error for TimerClientError {}

/// Blocking client driving one timer server over one connection.
///
/// Requests are numbered from one, and the client reads lines until the
/// response carrying the identifier it sent. A notification met while
/// waiting is buffered, so a subscriber never loses one to a request.
pub struct TimerClient {
    reader: BufReader<TimerStream>,
    writer: TimerStream,
    pending: VecDeque<TimerEvent>,
    id: i64,
}

impl TimerClient {
    /// Connects to the server listening at the given address.
    ///
    /// A socket path the platform refuses keeps its
    /// [`TimerTransportError`], no server being at fault.
    pub fn connect(address: &TimerAddress) -> Result<Self> {
        let stream = match TimerStream::connect(address) {
            Ok(stream) => stream,
            Err(err) if err.is::<TimerTransportError>() => return Err(err),
            Err(err) => return Err(err.context(TimerClientError::Unreachable)),
        };

        let reader = BufReader::new(stream.try_clone()?);

        Ok(Self {
            reader,
            writer: stream,
            pending: VecDeque::new(),
            id: 0,
        })
    }

    /// Returns the timer without touching it.
    pub fn get(&mut self) -> Result<Timer> {
        match self.send(TimerRequest::Get)? {
            TimerResponse::Timer(timer) => Ok(timer),
            other => Err(anyhow!("Invalid response {other:?}, expected a timer")
                .context(TimerClientError::Mismatch)),
        }
    }

    /// Starts the timer from its first cycle.
    pub fn start(&mut self) -> Result<Vec<TimerEvent>> {
        self.events(TimerRequest::Start)
    }

    /// Pauses the timer, keeping the elapsed time.
    pub fn pause(&mut self) -> Result<Vec<TimerEvent>> {
        self.events(TimerRequest::Pause)
    }

    /// Resumes a paused timer.
    pub fn resume(&mut self) -> Result<Vec<TimerEvent>> {
        self.events(TimerRequest::Resume)
    }

    /// Stops the timer and resets it.
    pub fn stop(&mut self) -> Result<Vec<TimerEvent>> {
        self.events(TimerRequest::Stop)
    }

    /// Overrides the remaining duration of the current cycle.
    ///
    /// The server clamps the request to that cycle's configured length,
    /// so the returned event is what to read the result from.
    pub fn set(&mut self, duration: usize) -> Result<Vec<TimerEvent>> {
        self.events(TimerRequest::Set { duration })
    }

    /// Subscribes this connection to the timer notifications.
    ///
    /// Call [`Self::next_event`] afterwards to consume them.
    pub fn subscribe(&mut self) -> Result<()> {
        match self.send(TimerRequest::Subscribe)? {
            TimerResponse::Subscription(true) => Ok(()),
            other => Err(
                anyhow!("Invalid response {other:?}, expected a subscription")
                    .context(TimerClientError::Mismatch),
            ),
        }
    }

    /// Blocks until the next event, or returns `None` at end of stream.
    ///
    /// Requires a prior [`Self::subscribe`], since a server pushes
    /// nothing to a connection that did not ask. Events buffered while
    /// waiting for a response come out first, in arrival order.
    pub fn next_event(&mut self) -> Result<Option<TimerEvent>> {
        loop {
            if let Some(event) = self.pending.pop_front() {
                return Ok(Some(event));
            }

            let Some(line) = self.read_line()? else {
                debug!("timer server closed the connection");
                return Ok(None);
            };

            self.buffer_notification(&line);
        }
    }

    fn events(&mut self, request: TimerRequest) -> Result<Vec<TimerEvent>> {
        match self.send(request)? {
            TimerResponse::Events(events) => Ok(events),
            other => Err(anyhow!("Invalid response {other:?}, expected events")
                .context(TimerClientError::Mismatch)),
        }
    }

    fn send(&mut self, request: TimerRequest) -> Result<TimerResponse> {
        self.id += 1;
        let id = Jsonrpc20Id::Number(self.id);
        let envelope = request.clone().into_jsonrpc(self.id);

        let mut line =
            serde_json::to_string(&envelope).context("Cannot serialize the timer request")?;
        trace!("send: {line}");
        line.push('\n');

        self.writer
            .write_all(line.as_bytes())
            .context(TimerClientError::Closed)?;
        self.writer.flush().context(TimerClientError::Closed)?;

        loop {
            let Some(line) = self.read_line()? else {
                return Err(TimerClientError::Closed.into());
            };

            let Ok(response) = serde_json::from_str::<Jsonrpc20Response>(&line) else {
                self.buffer_notification(&line);
                continue;
            };

            if response.id.as_ref() != Some(&id) {
                trace!("skip response for another request: {line}");
                continue;
            }

            return match response.outcome {
                Jsonrpc20Outcome::Error(err) => {
                    Err(anyhow!("{err}").context(TimerClientError::Server))
                }
                Jsonrpc20Outcome::Result(value) => {
                    TimerResponse::from_value(&request, value).context(TimerClientError::Mismatch)
                }
            };
        }
    }

    /// Queues `line` when it holds a timer notification.
    ///
    /// Anything else is dropped, so a notification this client does not
    /// know about is not an error.
    fn buffer_notification(&mut self, line: &str) {
        let Ok(notification) = serde_json::from_str::<Jsonrpc20Request>(line) else {
            trace!("skip unparsable line: {line}");
            return;
        };

        match TimerEvent::from_notification(&notification) {
            Some(event) => self.pending.push_back(event),
            None => trace!("skip unknown notification: {}", notification.method),
        }
    }

    fn read_line(&mut self) -> Result<Option<String>> {
        let mut line = String::new();

        let n = self
            .reader
            .read_line(&mut line)
            .context(TimerClientError::Closed)?;

        if n == 0 {
            return Ok(None);
        }

        trace!("recv: {}", line.trim_end());
        Ok(Some(line))
    }
}

#[cfg(test)]
mod tests;
