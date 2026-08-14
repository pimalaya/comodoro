//! Blocking timer server speaking JSON-RPC 2.0 over its transports.
//!
//! Written against the standard library, hence the module name: an
//! asynchronous server would be a sibling rather than a replacement.
//!
//! One thread accepts connections per bound address, one thread ticks
//! the timer every second, and every connection gets a reader thread and
//! a writer thread. The split matters: a subscribed connection blocks
//! forever on its next request, so notifications could not be delivered
//! from the thread that reads them. Every line leaving the server
//! therefore goes through one channel per connection, which also
//! serializes writes without locking the socket.

use alloc::{string::String, vec::Vec};

use std::{
    io::{BufRead, BufReader, Write},
    sync::{
        Arc, Mutex, MutexGuard,
        atomic::{AtomicUsize, Ordering},
        mpsc::{self, Receiver, Sender},
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result};
use log::{debug, error, trace, warn};

use crate::{
    jsonrpc20::{
        Jsonrpc20Error, Jsonrpc20Incoming, Jsonrpc20Outgoing, Jsonrpc20Request, Jsonrpc20Response,
    },
    protocol::{TimerRequest, TimerResponse},
    timer::{Timer, TimerEvent, TimerSchedule},
    transport::{TimerAddress, TimerListener, TimerStream},
};

/// Blocking timer server.
///
/// Owns the [`Timer`] behind a mutex, answers requests on every bound
/// address,
/// and pushes every [`TimerEvent`] both to the subscribed connections
/// and to the receiver [`serve`] returns, which is what the CLI drives
/// its hooks from.
///
/// [`serve`]: TimerServer::serve
pub struct TimerServer {
    /// What the timer it owns runs: its cycles, and how many loops of
    /// them.
    pub schedule: TimerSchedule,
    /// The addresses to bind, one listener each.
    pub addresses: Vec<TimerAddress>,
}

impl TimerServer {
    /// Binds every address and spawns the accept and tick threads.
    ///
    /// Returns the receiver carrying every event the timer emits, in
    /// the order it emitted them.
    ///
    /// Binding happens before any thread is spawned, so a server that
    /// cannot bind one of its addresses starts none of them.
    pub fn serve(self) -> Result<Receiver<TimerEvent>> {
        let listeners = self
            .addresses
            .iter()
            .map(TimerListener::bind)
            .collect::<Result<Vec<_>>>()?;

        let timer = Arc::new(Mutex::new(Timer::new(self.schedule)));
        let (tx, rx) = mpsc::channel();
        let broadcast = TimerBroadcast::new(tx);

        for listener in listeners {
            thread::spawn({
                let timer = timer.clone();
                let broadcast = broadcast.clone();
                move || {
                    loop {
                        let stream = match listener.accept() {
                            Ok(stream) => stream,
                            Err(err) => {
                                error!("cannot accept connection: {err}");
                                continue;
                            }
                        };

                        let timer = timer.clone();
                        let broadcast = broadcast.clone();

                        thread::spawn(move || {
                            if let Err(err) = serve_connection(stream, timer, broadcast) {
                                error!("cannot serve connection: {err}");
                            }
                        });
                    }
                }
            });
        }

        thread::spawn({
            let timer = timer.clone();
            let broadcast = broadcast.clone();
            move || {
                loop {
                    thread::sleep(Duration::from_secs(1));
                    let events: Vec<_> = lock(&timer).update(now()).into_iter().collect();
                    broadcast.emit(events);
                }
            }
        });

        Ok(rx)
    }
}

/// Fan-out of the events a timer emits.
///
/// Holds the sender feeding the caller of [`TimerServer::serve`] plus
/// the line sender of every subscribed connection. A subscriber whose
/// channel is closed has gone away and is dropped on the next emit, so
/// nothing has to reap them.
#[derive(Clone)]
struct TimerBroadcast {
    events: Sender<TimerEvent>,
    subscribers: TimerSubscribers,
}

/// The line sender of every subscribed connection, by connection id.
type TimerSubscribers = Arc<Mutex<Vec<(usize, Sender<String>)>>>;

impl TimerBroadcast {
    fn new(events: Sender<TimerEvent>) -> Self {
        Self {
            events,
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Registers connection `id` as a subscriber, replacing any earlier
    /// subscription it held so subscribing twice is idempotent.
    fn subscribe(&self, id: usize, lines: Sender<String>) {
        let mut subscribers = lock(&self.subscribers);
        subscribers.retain(|(other, _)| *other != id);
        subscribers.push((id, lines));
        debug!("connection {id} subscribed, {} total", subscribers.len());
    }

    /// Removes the subscription of connection `id`, if it holds one.
    fn unsubscribe(&self, id: usize) {
        let mut subscribers = lock(&self.subscribers);
        subscribers.retain(|(other, _)| *other != id);
        debug!("connection {id} unsubscribed, {} left", subscribers.len());
    }

    /// Sends `events` to the hook receiver and to every subscriber.
    fn emit(&self, events: impl IntoIterator<Item = TimerEvent>) {
        for event in events {
            trace!("emit {event:?}");

            match serde_json::to_string(&event.clone().into_notification()) {
                Ok(line) => {
                    let mut subscribers = lock(&self.subscribers);
                    subscribers.retain(|(_, lines)| lines.send(line.clone()).is_ok());
                }
                Err(err) => error!("cannot serialize notification: {err}"),
            }

            if self.events.send(event).is_err() {
                trace!("no hook receiver left");
            }
        }
    }
}

/// Serves one connection until the peer disconnects.
fn serve_connection(
    stream: TimerStream,
    timer: Arc<Mutex<Timer>>,
    broadcast: TimerBroadcast,
) -> Result<()> {
    let id = NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);
    debug!("begin connection {id}");

    let reader = BufReader::new(stream.try_clone()?);
    let (lines, outgoing) = mpsc::channel::<String>();

    let writer = thread::spawn({
        let mut stream = stream;
        move || {
            for mut line in outgoing {
                trace!("send: {line}");
                line.push('\n');

                if stream.write_all(line.as_bytes()).is_err() || stream.flush().is_err() {
                    debug!("connection {id} went away while writing");
                    break;
                }
            }
        }
    });

    for line in reader.lines() {
        let line = line.context("Read request error")?;

        if line.trim().is_empty() {
            continue;
        }

        trace!("recv: {line}");

        let Some(response) = handle_line(&line, id, &timer, &broadcast, &lines) else {
            continue;
        };

        match serde_json::to_string(&response) {
            Ok(line) => {
                if lines.send(line).is_err() {
                    break;
                }
            }
            Err(err) => error!("cannot serialize response: {err}"),
        }
    }

    broadcast.unsubscribe(id);
    drop(lines);
    let _ = writer.join();

    debug!("end of connection {id}");
    Ok(())
}

/// Answers one received line, or nothing when it held only
/// notifications.
fn handle_line(
    line: &str,
    id: usize,
    timer: &Arc<Mutex<Timer>>,
    broadcast: &TimerBroadcast,
    lines: &Sender<String>,
) -> Option<Jsonrpc20Outgoing> {
    let incoming: Jsonrpc20Incoming = match serde_json::from_str(line) {
        Ok(incoming) => incoming,
        Err(err) => {
            let err = Jsonrpc20Error::parse(err);
            return Some(Jsonrpc20Outgoing::Single(Jsonrpc20Response::error(
                err, None,
            )));
        }
    };

    match incoming {
        Jsonrpc20Incoming::Single(request) => {
            handle_request(request, id, timer, broadcast, lines).map(Jsonrpc20Outgoing::Single)
        }
        Jsonrpc20Incoming::Batch(requests) if requests.is_empty() => {
            let err = Jsonrpc20Error::invalid_request("A batch must not be empty");
            Some(Jsonrpc20Outgoing::Single(Jsonrpc20Response::error(
                err, None,
            )))
        }
        Jsonrpc20Incoming::Batch(requests) => {
            let responses: Vec<_> = requests
                .into_iter()
                .filter_map(|request| handle_request(request, id, timer, broadcast, lines))
                .collect();

            (!responses.is_empty()).then_some(Jsonrpc20Outgoing::Batch(responses))
        }
    }
}

/// Runs one request, returning nothing when it was a notification.
fn handle_request(
    request: Jsonrpc20Request,
    id: usize,
    timer: &Arc<Mutex<Timer>>,
    broadcast: &TimerBroadcast,
    lines: &Sender<String>,
) -> Option<Jsonrpc20Response> {
    let outcome = TimerRequest::from_jsonrpc(&request)
        .map(|parsed| dispatch(parsed, id, timer, broadcast, lines));

    if request.is_notification() {
        if let Err(err) = outcome {
            debug!("ignored failing notification {}: {err}", request.method);
        }
        return None;
    }

    Some(match outcome {
        Ok(response) => Jsonrpc20Response::result(response.to_value(), request.id),
        Err(err) => Jsonrpc20Response::error(err, request.id),
    })
}

/// Applies one parsed request to the timer.
fn dispatch(
    request: TimerRequest,
    id: usize,
    timer: &Arc<Mutex<Timer>>,
    broadcast: &TimerBroadcast,
    lines: &Sender<String>,
) -> TimerResponse {
    let events: Vec<TimerEvent> = match request {
        TimerRequest::Get => return TimerResponse::Timer(lock(timer).clone()),
        TimerRequest::Subscribe => {
            broadcast.subscribe(id, lines.clone());
            return TimerResponse::Subscription(true);
        }
        TimerRequest::Unsubscribe => {
            broadcast.unsubscribe(id);
            return TimerResponse::Subscription(false);
        }
        TimerRequest::Start => lock(timer).start(now()).into_iter().collect(),
        TimerRequest::Pause => lock(timer).pause(now()).into_iter().collect(),
        TimerRequest::Resume => lock(timer).resume(now()).into_iter().collect(),
        TimerRequest::Stop => lock(timer).stop().into_iter().collect(),
        TimerRequest::Set { duration } => lock(timer).set(now(), duration).into_iter().collect(),
    };

    broadcast.emit(events.clone());
    TimerResponse::Events(events)
}

/// Returns the current time as whole seconds since the Unix epoch.
///
/// A clock set before the epoch is not a recoverable condition for a
/// timer, so it saturates at zero rather than threading an error
/// through every call site.
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|now| now.as_secs())
        .unwrap_or_default()
}

/// Locks `mutex`, recovering the guard when a holder panicked.
///
/// Every value guarded here is a plain struct, so a panic mid-update
/// can leave it stale but never torn. Recovering keeps one bad request
/// from taking the whole daemon down with it.
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|err| {
        warn!("recovering from a poisoned lock");
        err.into_inner()
    })
}

/// Source of the identifiers distinguishing connections in the logs and
/// in the subscriber registry.
static NEXT_CONNECTION_ID: AtomicUsize = AtomicUsize::new(0);
