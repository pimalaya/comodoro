//! Runs a server and a client over a real local socket.
//!
//! Starts a server on a throwaway socket, subscribes a client to it,
//! drives the timer, and prints both the results the client gets back
//! and the notifications the server pushes on its own.

use std::{env, fs, thread, time::Duration};

use comodoro::{
    client::std::TimerClient,
    server::std::TimerServer,
    timer::{TimerConfig, TimerCycle, TimerCycles, TimerLoop},
    transport::TimerAddress,
};

fn main() {
    let path = env::temp_dir().join("comodoro-example.sock");
    let _ = fs::remove_file(&path);

    let config = TimerConfig {
        cycles: TimerCycles::from([TimerCycle::new("Work", 2), TimerCycle::new("Rest", 2)]),
        cycles_count: TimerLoop::Infinite,
    };

    let address = TimerAddress::UnixSocket(path.clone());

    let events = TimerServer {
        config,
        addresses: vec![address.clone()],
    }
    .serve()
    .expect("serve timer");

    // NOTE: the server binds before serve() returns, so a client can
    // connect straight away. This thread only drains the hook channel,
    // which the CLI uses to fire hooks.
    thread::spawn(move || while events.recv().is_ok() {});

    let mut client = TimerClient::connect(&address).expect("connect to timer");
    client.subscribe().expect("subscribe to timer");

    println!("start: {:?}", client.start().expect("start timer"));
    println!("get:   {:?}", client.get().expect("get timer").state);

    for _ in 0..4 {
        let event = client.next_event().expect("read event");
        println!("event: {event:?}");
    }

    println!("pause: {:?}", client.pause().expect("pause timer"));
    thread::sleep(Duration::from_millis(100));
    println!("stop:  {:?}", client.stop().expect("stop timer"));

    let _ = fs::remove_file(&path);
}
