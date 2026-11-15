//! # Watch command
//!
//! Streams the timer on every change the server pushes, as events, until
//! a termination signal stops it with one result.

use core::{fmt, time::Duration};

use std::{
    sync::mpsc::{self, RecvTimeoutError, Sender},
    thread,
};

use anyhow::{Error, Result};
use clap::Parser;
use pimalaya_cli::{printer::Printer, signal::Shutdown};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    cli::{account::Account, client::timer::TimerOutput, transport::TransportArg},
    client::std::{TimerClient, TimerClientError},
    timer::{Timer, TimerPrecision},
};

/// How often the watch looks whether a signal asked it to stop.
const SHUTDOWN_POLL: Duration = Duration::from_millis(100);

/// Watch the timer.
///
/// Subscribes to the server and prints the timer every time it changes,
/// until interrupted. Suited to a status bar, which no longer has to
/// poll.
#[derive(Debug, Parser)]
pub struct TimerWatchCommand {
    /// The transport used to reach the server.
    #[command(flatten)]
    pub transport: TransportArg,
}

/// What `watch` observes while it runs.
#[derive(Clone, Debug, JsonSchema, Serialize)]
#[serde(tag = "event", rename_all = "kebab-case")]
pub enum TimerWatchEvent {
    /// The timer as the watch starts, then after each visible change.
    TimerChanged {
        /// The timer, its keys beside `event`.
        #[serde(flatten)]
        timer: TimerOutput,
    },
}

impl TimerWatchEvent {
    /// The change to `timer`, rendered at `precision`.
    fn changed(timer: Timer, precision: TimerPrecision) -> Self {
        let timer = TimerOutput::new(timer, precision);
        Self::TimerChanged { timer }
    }
}

impl fmt::Display for TimerWatchEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimerChanged { timer } => timer.fmt(f),
        }
    }
}

/// What stopped a watch that ended cleanly.
#[derive(Clone, Copy, Debug, Eq, PartialEq, JsonSchema, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimerWatchStop {
    /// A termination signal: SIGINT, SIGTERM or SIGHUP, or Ctrl-C.
    Signal,
}

/// The result `watch` prints once it stops, after its events.
#[derive(Clone, Debug, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerWatchOutput {
    /// What stopped the watch.
    pub stopped_by: TimerWatchStop,
}

impl fmt::Display for TimerWatchOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.stopped_by {
            TimerWatchStop::Signal => writeln!(f, "Watch stopped"),
        }
    }
}

impl TimerWatchCommand {
    /// Prints the timer as an event on start and on every change the
    /// server pushes, then the result once a signal stops it.
    ///
    /// The server closing the connection fails coded `server-closed`.
    pub fn execute(self, printer: &mut impl Printer, account: &Account) -> Result<()> {
        let shutdown = Shutdown::install()?;

        let address = account.address(self.transport.transport);
        let mut client = TimerClient::connect(&address)?;
        client.subscribe()?;

        let mut last = client.get()?;
        let precision = &account.precision;
        printer.event(TimerWatchEvent::changed(last.clone(), precision.clone()))?;

        // NOTE: the client blocks on the socket, so it reads on a thread
        // of its own while this one polls the shutdown.
        let (timers, changes) = mpsc::channel();
        thread::spawn(move || read_changes(client, timers));

        loop {
            if shutdown.is_requested() {
                let output = TimerWatchOutput {
                    stopped_by: TimerWatchStop::Signal,
                };
                return printer.out(output);
            }

            let timer = match changes.recv_timeout(SHUTDOWN_POLL) {
                Ok(timer) => timer?,
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => return Err(closed()),
            };

            // NOTE: a cycle transition emits several events in the same
            // tick, all rendering the same line, so comparing the timer
            // keeps the output at one line per visible change.
            if timer == last {
                continue;
            }

            last = timer.clone();
            printer.event(TimerWatchEvent::changed(timer, precision.clone()))?;
        }
    }
}

/// Sends the timer after every event the server pushes, until the
/// connection fails or closes, which it sends as the last item.
fn read_changes(mut client: TimerClient, timers: Sender<Result<Timer>>) {
    loop {
        let timer = match client.next_event() {
            Ok(Some(_)) => client.get(),
            Ok(None) => Err(closed()),
            Err(err) => Err(err),
        };

        let failed = timer.is_err();

        if timers.send(timer).is_err() || failed {
            return;
        }
    }
}

/// The failure of a watch whose server closed the connection.
fn closed() -> Error {
    Error::new(TimerClientError::Closed).context("Watch timer error")
}
