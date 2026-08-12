//! Blocking timer client speaking JSON-RPC 2.0 over one connection.

use alloc::{string::String, vec::Vec};
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Write},
};

use anyhow::{Context, Result, bail};
use log::{debug, trace};

use crate::{
    jsonrpc20::{Jsonrpc20Id, Jsonrpc20Outcome, Jsonrpc20Request, Jsonrpc20Response},
    protocol::{TimerRequest, TimerResponse},
    timer::{Timer, TimerEvent},
    transport::{TimerAddress, TimerStream},
};

/// Blocking client driving one timer server over one connection.
///
/// Requests are numbered from one, and the client reads lines until it
/// finds the response carrying the identifier it just sent. Once
/// subscribed, notifications arrive interleaved with those responses,
/// so any notification met while waiting for a response is buffered
/// rather than dropped. That is what lets a subscribed connection keep
/// issuing requests without losing a single event.
pub struct TimerClient {
    reader: BufReader<TimerStream>,
    writer: TimerStream,
    pending: VecDeque<TimerEvent>,
    id: i64,
}

impl TimerClient {
    /// Connects to the server listening at the given address.
    pub fn connect(address: &TimerAddress) -> Result<Self> {
        let stream = TimerStream::connect(address)?;
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
            other => bail!("Invalid response {other:?}, expected a timer"),
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
    pub fn set(&mut self, duration: usize) -> Result<Vec<TimerEvent>> {
        self.events(TimerRequest::Set { duration })
    }

    /// Subscribes this connection to the timer notifications.
    ///
    /// Call [`Self::next_event`] afterwards to consume them.
    pub fn subscribe(&mut self) -> Result<()> {
        match self.send(TimerRequest::Subscribe)? {
            TimerResponse::Subscription(true) => Ok(()),
            other => bail!("Invalid response {other:?}, expected a subscription"),
        }
    }

    /// Stops sending timer notifications on this connection.
    pub fn unsubscribe(&mut self) -> Result<()> {
        match self.send(TimerRequest::Unsubscribe)? {
            TimerResponse::Subscription(false) => Ok(()),
            other => bail!("Invalid response {other:?}, expected a subscription"),
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
            other => bail!("Invalid response {other:?}, expected events"),
        }
    }

    fn send(&mut self, request: TimerRequest) -> Result<TimerResponse> {
        self.id += 1;
        let id = Jsonrpc20Id::Number(self.id);
        let envelope = request.clone().into_jsonrpc(self.id);

        let mut line = serde_json::to_string(&envelope).context("Serialize timer request error")?;
        trace!("send: {line}");
        line.push('\n');

        self.writer
            .write_all(line.as_bytes())
            .context("Write timer request error")?;
        self.writer.flush().context("Flush timer request error")?;

        loop {
            let Some(line) = self.read_line()? else {
                bail!("Timer server closed the connection before answering");
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
                Jsonrpc20Outcome::Error(err) => bail!("{err}"),
                Jsonrpc20Outcome::Result(value) => TimerResponse::from_value(&request, value)
                    .context("Deserialize timer response error"),
            };
        }
    }

    /// Queues `line` when it holds a timer notification.
    ///
    /// Anything else is dropped, so a server pushing a notification this
    /// client does not know about is not an error.
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
            .context("Read timer response error")?;

        if n == 0 {
            return Ok(None);
        }

        trace!("recv: {}", line.trim_end());
        Ok(Some(line))
    }
}
