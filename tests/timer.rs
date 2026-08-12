//! End-to-end coverage of clients talking to a server over real
//! connections, exercising the JSON-RPC surface rather than the
//! internals.

use std::{
    env, fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

use comodoro::{
    client::TimerClient,
    server::std::TimerServer,
    timer::{TimerConfig, TimerCycle, TimerCycles, TimerEvent, TimerLoop, TimerState},
    transport::TimerAddress,
};

static NEXT_SOCKET: AtomicUsize = AtomicUsize::new(0);

/// The cycles every server in this file runs.
fn config() -> TimerConfig {
    TimerConfig {
        cycles: TimerCycles::from([TimerCycle::new("Work", 1500), TimerCycle::new("Break", 300)]),
        cycles_count: TimerLoop::Infinite,
    }
}

/// Starts a server on a socket unique to this test, and returns a
/// client connected to it.
fn serve() -> (TimerClient, PathBuf) {
    let id = NEXT_SOCKET.fetch_add(1, Ordering::Relaxed);
    let path = env::temp_dir().join(format!("comodoro-test-{}-{id}.sock", std::process::id()));
    let _ = fs::remove_file(&path);

    let address = TimerAddress::UnixSocket(path.clone());

    let events = TimerServer {
        config: config(),
        addresses: vec![address.clone()],
    }
    .serve()
    .expect("serve timer");

    thread::spawn(move || while events.recv().is_ok() {});

    let client = TimerClient::connect(&address).expect("connect to timer");
    (client, path)
}

#[test]
fn get_returns_a_stopped_timer() {
    let (mut client, path) = serve();

    let timer = client.get().unwrap();

    assert_eq!(timer.state, TimerState::Stopped);
    assert_eq!(timer.cycle.name, "Work");
    let _ = fs::remove_file(path);
}

#[test]
fn start_returns_started_then_began() {
    let (mut client, path) = serve();

    let events = client.start().unwrap();

    assert_eq!(events.len(), 2);
    assert_eq!(events[0], TimerEvent::Started);
    assert!(matches!(events[1], TimerEvent::Began(_)));
    assert_eq!(client.get().unwrap().state, TimerState::Running);
    let _ = fs::remove_file(path);
}

#[test]
fn start_on_a_running_timer_is_a_noop() {
    let (mut client, path) = serve();

    client.start().unwrap();
    assert!(client.start().unwrap().is_empty());
    let _ = fs::remove_file(path);
}

#[test]
fn stop_returns_ended_then_stopped() {
    let (mut client, path) = serve();

    client.start().unwrap();
    let events = client.stop().unwrap();

    assert_eq!(events.len(), 2);
    assert!(matches!(events[0], TimerEvent::Ended(_)));
    assert_eq!(events[1], TimerEvent::Stopped);
    assert_eq!(client.get().unwrap().state, TimerState::Stopped);
    let _ = fs::remove_file(path);
}

#[test]
fn stop_on_a_stopped_timer_is_a_noop() {
    let (mut client, path) = serve();

    assert!(client.stop().unwrap().is_empty());
    let _ = fs::remove_file(path);
}

#[test]
fn pause_then_resume_round_trips_the_state() {
    let (mut client, path) = serve();

    client.start().unwrap();

    let paused = client.pause().unwrap();
    assert_eq!(paused.len(), 1);
    assert!(matches!(paused[0], TimerEvent::Paused(_)));
    assert_eq!(client.get().unwrap().state, TimerState::Paused);

    let resumed = client.resume().unwrap();
    assert_eq!(resumed.len(), 1);
    assert!(matches!(resumed[0], TimerEvent::Resumed(_)));
    assert_eq!(client.get().unwrap().state, TimerState::Running);
    let _ = fs::remove_file(path);
}

#[test]
fn set_overrides_the_remaining_duration() {
    let (mut client, path) = serve();
    client.start().unwrap();

    let events = client.set(60).unwrap();

    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], TimerEvent::Set(_)));
    assert_eq!(client.get().unwrap().cycle.duration, 60);

    // The server ticks every second, and the tick used to recompute the
    // cycle from the elapsed time and discard what was set. Reading
    // across one is the whole point of this assertion.
    thread::sleep(Duration::from_millis(1500));
    let duration = client.get().unwrap().cycle.duration;
    assert!((57..=59).contains(&duration), "{duration}");

    let _ = fs::remove_file(path);
}

#[test]
fn a_paused_timer_can_be_stopped() {
    let (mut client, path) = serve();

    client.start().unwrap();
    client.pause().unwrap();

    let events = client.stop().unwrap();

    assert_eq!(events.len(), 2);
    assert!(matches!(events[0], TimerEvent::Ended(_)));
    assert_eq!(events[1], TimerEvent::Stopped);
    assert_eq!(client.get().unwrap().state, TimerState::Stopped);
    let _ = fs::remove_file(path);
}

#[test]
fn a_subscriber_receives_the_events_its_own_calls_emit() {
    let (mut client, path) = serve();

    client.subscribe().unwrap();
    client.start().unwrap();

    assert_eq!(client.next_event().unwrap(), Some(TimerEvent::Started));
    assert!(matches!(
        client.next_event().unwrap(),
        Some(TimerEvent::Began(_))
    ));
    let _ = fs::remove_file(path);
}

#[test]
fn a_subscriber_receives_the_events_another_client_emits() {
    let (mut watcher, path) = serve();
    let address = TimerAddress::UnixSocket(path.clone());
    let mut driver = TimerClient::connect(&address).expect("connect a second client");

    watcher.subscribe().unwrap();
    driver.start().unwrap();

    assert_eq!(watcher.next_event().unwrap(), Some(TimerEvent::Started));
    assert!(matches!(
        watcher.next_event().unwrap(),
        Some(TimerEvent::Began(_))
    ));
    let _ = fs::remove_file(path);
}

#[test]
fn unsubscribing_stops_the_notifications_but_not_the_requests() {
    let (mut client, path) = serve();

    client.subscribe().unwrap();
    client.start().unwrap();
    client.next_event().unwrap();
    client.next_event().unwrap();

    client.unsubscribe().unwrap();

    // The connection still answers requests after unsubscribing, and
    // the events these emit no longer come back as notifications.
    let events = client.stop().unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(client.get().unwrap().state, TimerState::Stopped);
    let _ = fs::remove_file(path);
}

#[test]
fn a_second_server_refuses_a_socket_in_use() {
    let (_client, path) = serve();

    let err = TimerServer {
        config: config(),
        addresses: vec![TimerAddress::UnixSocket(path.clone())],
    }
    .serve()
    .unwrap_err();

    assert!(err.to_string().contains("already in use"), "{err}");
    let _ = fs::remove_file(path);
}

#[test]
fn both_transports_serve_the_same_timer() {
    let id = NEXT_SOCKET.fetch_add(1, Ordering::Relaxed);
    let path = env::temp_dir().join(format!("comodoro-test-{}-{id}.sock", std::process::id()));
    let _ = fs::remove_file(&path);

    let socket = TimerAddress::UnixSocket(path.clone());
    let tcp = TimerAddress::Tcp {
        host: "127.0.0.1".into(),
        // NOTE: the port the OS picks cannot be read back, since a bound
        // listener stays inside the server, so this one is fixed and
        // owned by this test alone.
        port: 47821,
    };

    let events = TimerServer {
        config: config(),
        addresses: vec![socket.clone(), tcp.clone()],
    }
    .serve()
    .expect("serve timer");

    thread::spawn(move || while events.recv().is_ok() {});

    let mut watcher = TimerClient::connect(&socket).expect("connect over the socket");
    let mut driver = TimerClient::connect(&tcp).expect("connect over TCP");

    watcher.subscribe().expect("subscribe over the socket");
    driver.start().expect("start over TCP");

    // The events a TCP client emits reach a subscriber connected over
    // the socket, and both connections read the same timer.
    assert_eq!(watcher.next_event().unwrap(), Some(TimerEvent::Started));
    assert_eq!(watcher.get().unwrap(), driver.get().unwrap());

    let _ = fs::remove_file(path);
}

#[test]
fn a_second_server_refuses_a_port_in_use() {
    let address = TimerAddress::Tcp {
        host: "127.0.0.1".into(),
        port: 47822,
    };

    let events = TimerServer {
        config: config(),
        addresses: vec![address.clone()],
    }
    .serve()
    .expect("serve timer");

    thread::spawn(move || while events.recv().is_ok() {});

    let err = TimerServer {
        config: config(),
        addresses: vec![address],
    }
    .serve()
    .unwrap_err();

    assert!(
        err.to_string().contains("Bind socket 127.0.0.1:47822"),
        "{err}"
    );
}
