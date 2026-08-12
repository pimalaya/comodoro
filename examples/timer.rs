//! Runs the pure timer state machine with no I/O at all.
//!
//! Time is a parameter, so the whole lifecycle of a timer can be
//! replayed instantly by handing it the timestamps it would have seen.
//! This is the layer that compiles under no_std, and the reason the
//! interesting behaviour is testable without a socket or a clock.

use comodoro::timer::{Timer, TimerConfig, TimerCycle, TimerCycles, TimerLoop};

fn main() {
    let config = TimerConfig {
        cycles: TimerCycles::from([TimerCycle::new("Work", 3), TimerCycle::new("Rest", 2)]),
        cycles_count: TimerLoop::Fixed(2),
    };

    let mut timer = Timer::new(config);

    println!("start at t=0:");
    for event in timer.start(0) {
        println!("  {event:?}");
    }

    for now in 1..=10 {
        let events: Vec<_> = timer.update(now).into_iter().collect();

        if events.is_empty() {
            continue;
        }

        println!(
            "tick at t={now}: [{}] {}s left",
            timer.cycle.name, timer.cycle.duration
        );
        for event in events {
            println!("  {event:?}");
        }
    }

    println!("stop:");
    for event in timer.stop() {
        println!("  {event:?}");
    }
}
