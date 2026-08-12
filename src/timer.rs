//! Pure timer state machine.
//!
//! The [`Timer`] struct is I/O-free: it never reads the clock itself.
//! Methods that need the current time accept `now: u64` (Unix epoch
//! seconds) as a parameter, so the caller decides where time comes
//! from. This is the whole of Comodoro's logic, and it knows nothing
//! about sockets, JSON-RPC or configuration.

use core::ops::{Deref, DerefMut};

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use serde::{Deserialize, Serialize};

/// Controls how many full loops the timer runs before stopping.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum TimerLoop {
    /// The timer loops indefinitely and never stops by itself.
    ///
    /// The only way to stop such a timer is via [`Timer::stop`].
    #[default]
    Infinite,
    /// The timer stops automatically after the given number of loops.
    Fixed(usize),
}

impl From<usize> for TimerLoop {
    fn from(count: usize) -> Self {
        if count == 0 {
            Self::Infinite
        } else {
            Self::Fixed(count)
        }
    }
}

/// A single step in the timer lifecycle, identified by a name and a
/// duration in seconds.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TimerCycle {
    /// The name of this cycle.
    pub name: String,
    /// Remaining seconds in this cycle.
    ///
    /// From the *configuration* perspective this is the total cycle
    /// duration; from the *running timer* perspective it is the time
    /// remaining before the cycle ends.
    pub duration: usize,
}

impl TimerCycle {
    /// Creates a new cycle with the given name and duration.
    pub fn new(name: impl ToString, duration: usize) -> Self {
        Self {
            name: name.to_string(),
            duration,
        }
    }
}

/// The ordered list of cycles that a timer runs through.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct TimerCycles(Vec<TimerCycle>);

impl<T: IntoIterator<Item = TimerCycle>> From<T> for TimerCycles {
    fn from(cycles: T) -> Self {
        Self(cycles.into_iter().collect())
    }
}

impl Deref for TimerCycles {
    type Target = Vec<TimerCycle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TimerCycles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The current state of a timer.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum TimerState {
    /// The timer is running.
    Running,
    /// The timer has been paused.
    Paused,
    /// The timer is not running.
    #[default]
    Stopped,
}

/// An event emitted by a timer during its lifecycle.
///
/// Serialized adjacently, as `{"event": "began", "cycle": {…}}`, so an
/// event stays self-describing wherever it travels without a method
/// name next to it. See [`crate::protocol`] for the wire mapping.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "event", content = "cycle", rename_all = "camelCase")]
pub enum TimerEvent {
    /// The timer started.
    Started,
    /// The timer began the given cycle.
    Began(TimerCycle),
    /// The timer is running the given cycle (periodic tick).
    ///
    /// Carries the remaining duration as of that tick, so two
    /// consecutive ones never carry the same duration.
    Running(TimerCycle),
    /// The remaining duration was manually set.
    Set(TimerCycle),
    /// The timer was paused at the given cycle.
    Paused(TimerCycle),
    /// The timer was resumed at the given cycle.
    Resumed(TimerCycle),
    /// The timer ended the given cycle.
    Ended(TimerCycle),
    /// The timer stopped.
    Stopped,
}

/// Timer configuration: cycle definitions and loop count.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TimerConfig {
    /// The ordered list of timer cycles.
    pub cycles: TimerCycles,
    /// How many full loops the timer should run.
    pub cycles_count: TimerLoop,
}

impl TimerConfig {
    fn first_cycle(&self) -> TimerCycle {
        self.cycles
            .first()
            .cloned()
            .expect("timer config must have at least one cycle")
    }

    /// The cycles with their durations accumulated, so each one carries
    /// the elapsed time at which it ends and the last one carries the
    /// duration of a full loop.
    ///
    /// This is the view the timer reads the current cycle from: an
    /// elapsed time names a cycle by falling before its end and after
    /// the end of the one preceding it.
    ///
    /// Empty when the cycles are, and when they all last no time, since
    /// no elapsed time can name a cycle in either case.
    fn cumulated_cycles(&self) -> Vec<TimerCycle> {
        let cycles: Vec<_> = self
            .cycles
            .iter()
            .cloned()
            .scan(0, |end, mut cycle| {
                *end += cycle.duration;
                cycle.duration = *end;
                Some(cycle)
            })
            .collect();

        match cycles.last() {
            Some(last) if last.duration > 0 => cycles,
            _ => Vec::new(),
        }
    }
}

/// An I/O-free timer state machine.
///
/// All methods that depend on the current time accept `now: u64`
/// (seconds since the Unix epoch) rather than reading the clock
/// internally, which is what keeps the timer testable without a clock
/// and usable under no_std.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Timer {
    /// The timer configuration.
    pub config: TimerConfig,
    /// The current timer state.
    pub state: TimerState,
    /// The current cycle (with remaining duration).
    pub cycle: TimerCycle,
    /// The configured loop count, decremented as loops complete.
    pub cycles_count: TimerLoop,
    /// Unix epoch seconds at which the timer was last started or
    /// resumed. `None` when the timer is stopped or paused.
    pub started_at: Option<u64>,
    /// Accumulated elapsed seconds from previous runs (before the
    /// last pause or stop).
    pub elapsed: usize,
}

impl Timer {
    /// Creates a new timer from the given configuration.
    ///
    /// # Panics
    ///
    /// Panics if `config` has no cycles.
    pub fn new(config: TimerConfig) -> Self {
        let cycle = config.first_cycle();
        let cycles_count = config.cycles_count.clone();
        Self {
            config,
            cycle,
            cycles_count,
            ..Default::default()
        }
    }

    /// Returns the total elapsed seconds since the timer last started
    /// or resumed, plus any previously accumulated elapsed time.
    pub fn elapsed(&self, now: u64) -> usize {
        let running = self
            .started_at
            .map(|s| now.saturating_sub(s) as usize)
            .unwrap_or(0);
        running + self.elapsed
    }

    /// Advances the timer by one tick and returns any events that
    /// fired.
    ///
    /// Reports the cycle it just computed, never the one it replaced. A
    /// tick staying inside its cycle emits [`TimerEvent::Running`], a
    /// tick crossing into another emits [`TimerEvent::Ended`] then
    /// [`TimerEvent::Began`] instead, and a tick changing nothing emits
    /// nothing, which is what a tick landing less than a second after a
    /// [`Self::set`] does.
    ///
    /// Has no effect when the timer is paused or stopped, nor when its
    /// cycles add up to no time, since no elapsed time can name a cycle
    /// in that configuration.
    pub fn update(&mut self, now: u64) -> impl IntoIterator<Item = TimerEvent> {
        let mut events = Vec::with_capacity(3);

        if let TimerState::Running = self.state {
            let mut elapsed = self.elapsed(now);

            let cycles = self.config.cumulated_cycles();

            let Some(total_duration) = cycles.last().map(|cycle| cycle.duration) else {
                return events;
            };

            if let TimerLoop::Fixed(cycles_count) = self.cycles_count
                && elapsed >= total_duration * cycles_count
            {
                self.state = TimerState::Stopped;
                return events;
            }

            elapsed %= total_duration;

            let last_cycle = cycles[cycles.len() - 1].clone();
            let next_cycle = cycles
                .into_iter()
                .fold(None, |next_cycle, mut cycle| match next_cycle {
                    None if elapsed < cycle.duration => {
                        cycle.duration -= elapsed;
                        Some(cycle)
                    }
                    _ => next_cycle,
                })
                .unwrap_or(last_cycle);

            if self.cycle.name != next_cycle.name {
                let mut prev_cycle = self.cycle.clone();
                prev_cycle.duration = 0;
                events.push(TimerEvent::Ended(prev_cycle));
                events.push(TimerEvent::Began(next_cycle.clone()));
            } else if self.cycle.duration != next_cycle.duration {
                events.push(TimerEvent::Running(next_cycle.clone()));
            }

            self.cycle = next_cycle;
        }

        events
    }

    /// Starts the timer from the first configured cycle.
    ///
    /// Has no effect if the timer is already running or paused.
    pub fn start(&mut self, now: u64) -> impl IntoIterator<Item = TimerEvent> {
        let mut events = Vec::with_capacity(2);

        if matches!(self.state, TimerState::Stopped) {
            self.state = TimerState::Running;
            self.cycle = self.config.first_cycle();
            self.cycles_count = self.config.cycles_count.clone();
            self.started_at = Some(now);
            self.elapsed = 0;
            events.push(TimerEvent::Started);
            events.push(TimerEvent::Began(self.cycle.clone()));
        }

        events
    }

    /// Sets the remaining duration of the current cycle to
    /// `duration_secs`.
    ///
    /// The current cycle is derived from the elapsed time rather than
    /// stored, so this moves the elapsed time to the point that leaves
    /// `duration_secs` remaining, and every later tick recomputes the
    /// value it was given.
    ///
    /// A cycle cannot hold more than its configured length, since a
    /// longer remaining duration would place the timeline inside the
    /// previous cycle and rename the cycle under the caller. The
    /// request is clamped to that length, and the returned event
    /// carries the effective value.
    ///
    /// Has no effect if the timer is stopped, since [`Self::start`]
    /// resets the elapsed time anyway.
    pub fn set(&mut self, now: u64, duration_secs: usize) -> impl IntoIterator<Item = TimerEvent> {
        if matches!(self.state, TimerState::Stopped) {
            return None;
        }

        let cycles = self.config.cumulated_cycles();
        let total_duration = cycles.last().map(|cycle| cycle.duration)?;

        let elapsed = self.elapsed(now);
        let loops = elapsed / total_duration;
        let elapsed_in_loop = elapsed % total_duration;

        // NOTE: the cycle is found by position rather than by name,
        // since a configuration is free to run the same name twice.
        let index = cycles
            .iter()
            .position(|cycle| elapsed_in_loop < cycle.duration)
            .unwrap_or(cycles.len() - 1);

        let end = cycles[index].duration;
        let begin = index.checked_sub(1).map_or(0, |prev| cycles[prev].duration);
        let duration_secs = duration_secs.min(end - begin);

        self.cycle = TimerCycle::new(&cycles[index].name, duration_secs);
        self.elapsed = loops * total_duration + (end - duration_secs);
        self.started_at = matches!(self.state, TimerState::Running).then_some(now);

        Some(TimerEvent::Set(self.cycle.clone()))
    }

    /// Pauses the timer, saving the elapsed time.
    ///
    /// Has no effect if the timer is not running.
    pub fn pause(&mut self, now: u64) -> impl IntoIterator<Item = TimerEvent> {
        if matches!(self.state, TimerState::Running) {
            self.elapsed = self.elapsed(now);
            self.started_at = None;
            self.state = TimerState::Paused;
            Some(TimerEvent::Paused(self.cycle.clone()))
        } else {
            None
        }
    }

    /// Resumes the timer from where it was paused.
    ///
    /// Has no effect if the timer is not paused.
    pub fn resume(&mut self, now: u64) -> impl IntoIterator<Item = TimerEvent> {
        if matches!(self.state, TimerState::Paused) {
            self.state = TimerState::Running;
            self.started_at = Some(now);
            Some(TimerEvent::Resumed(self.cycle.clone()))
        } else {
            None
        }
    }

    /// Stops the timer and resets it to the initial state.
    ///
    /// Acts on a paused timer as well as on a running one, since a
    /// paused timer that cannot be stopped can only be left behind.
    /// Has no effect if the timer is already stopped.
    pub fn stop(&mut self) -> impl IntoIterator<Item = TimerEvent> {
        let mut events = Vec::with_capacity(2);

        if !matches!(self.state, TimerState::Stopped) {
            self.state = TimerState::Stopped;
            events.push(TimerEvent::Ended(self.cycle.clone()));
            events.push(TimerEvent::Stopped);
            self.cycle = self.config.first_cycle();
            self.cycles_count = self.config.cycles_count.clone();
            self.started_at = None;
            self.elapsed = 0;
        }

        events
    }
}

impl Eq for Timer {}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.cycle == other.cycle
            && self.started_at == other.started_at
            && self.elapsed == other.elapsed
    }
}

/// Display precision for the remaining cycle duration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum TimerPrecision {
    /// The remaining duration goes down to the second.
    #[serde(alias = "seconds", alias = "secs", alias = "sec", alias = "s")]
    Second,
    /// The remaining duration stops at the minute.
    #[default]
    #[serde(alias = "minutes", alias = "mins", alias = "min", alias = "m")]
    Minute,
    /// The remaining duration stops at the hour.
    #[serde(alias = "hours", alias = "h")]
    Hour,
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::timer::*;

    fn testing_timer() -> Timer {
        Timer {
            config: TimerConfig {
                cycles: TimerCycles::from([
                    TimerCycle::new("a", 3),
                    TimerCycle::new("b", 2),
                    TimerCycle::new("c", 1),
                ]),
                ..Default::default()
            },
            state: TimerState::Running,
            cycle: TimerCycle::new("a", 3),
            started_at: Some(0),
            ..Default::default()
        }
    }

    #[test]
    fn running_infinite_timer() {
        let mut timer = testing_timer();

        assert_eq!(timer.state, TimerState::Running);
        assert_eq!(timer.cycle, TimerCycle::new("a", 3));

        timer.update(2);
        assert_eq!(timer.state, TimerState::Running);
        assert_eq!(timer.cycle, TimerCycle::new("a", 1));

        timer.update(3);
        assert_eq!(timer.state, TimerState::Running);
        assert_eq!(timer.cycle, TimerCycle::new("b", 2));

        timer.update(5);
        assert_eq!(timer.state, TimerState::Running);
        assert_eq!(timer.cycle, TimerCycle::new("c", 1));

        timer.update(6);
        assert_eq!(timer.state, TimerState::Running);
        assert_eq!(timer.cycle, TimerCycle::new("a", 3));
    }

    #[test]
    fn running_timer_events() {
        let mut timer = testing_timer();
        let mut events = Vec::new();

        events.extend(timer.update(1));
        events.extend(timer.update(2));
        events.extend(timer.update(3));
        events.extend(timer.update(4));

        // Each tick reports what it just computed, so the durations go
        // down without repeating, and the tick ending a cycle says so
        // rather than announcing the cycle it is leaving.
        assert_eq!(
            events,
            vec![
                TimerEvent::Running(TimerCycle::new("a", 2)),
                TimerEvent::Running(TimerCycle::new("a", 1)),
                TimerEvent::Ended(TimerCycle::new("a", 0)),
                TimerEvent::Began(TimerCycle::new("b", 2)),
                TimerEvent::Running(TimerCycle::new("b", 1)),
            ]
        );
    }

    #[test]
    fn paused_timer_not_impacted_by_update() {
        let mut timer = testing_timer();
        timer.state = TimerState::Paused;
        let prev_timer = timer.clone();
        timer.update(10);
        assert_eq!(prev_timer, timer);
    }

    #[test]
    fn stopped_timer_not_impacted_by_update() {
        let mut timer = testing_timer();
        timer.state = TimerState::Stopped;
        let prev_timer = timer.clone();
        timer.update(10);
        assert_eq!(prev_timer, timer);
    }

    #[test]
    fn timer_lifecycle() {
        let mut timer = Timer::new(TimerConfig {
            cycles: TimerCycles::from([
                TimerCycle::new("a", 3),
                TimerCycle::new("b", 2),
                TimerCycle::new("c", 1),
            ]),
            ..Default::default()
        });

        let mut events = Vec::new();

        assert_eq!(timer.state, TimerState::Stopped);
        assert_eq!(timer.cycle, TimerCycle::new("a", 3));

        events.extend(timer.start(0));
        events.extend(timer.set(0, 2));

        assert_eq!(timer.state, TimerState::Running);
        assert_eq!(timer.cycle, TimerCycle::new("a", 2));

        events.extend(timer.pause(0));

        assert_eq!(timer.state, TimerState::Paused);
        assert_eq!(timer.cycle, TimerCycle::new("a", 2));

        events.extend(timer.resume(0));

        assert_eq!(timer.state, TimerState::Running);
        assert_eq!(timer.cycle, TimerCycle::new("a", 2));

        events.extend(timer.stop());

        assert_eq!(timer.state, TimerState::Stopped);
        assert_eq!(timer.cycle, TimerCycle::new("a", 3));

        assert_eq!(
            events,
            vec![
                TimerEvent::Started,
                TimerEvent::Began(TimerCycle::new("a", 3)),
                TimerEvent::Set(TimerCycle::new("a", 2)),
                TimerEvent::Paused(TimerCycle::new("a", 2)),
                TimerEvent::Resumed(TimerCycle::new("a", 2)),
                TimerEvent::Ended(TimerCycle::new("a", 2)),
                TimerEvent::Stopped,
            ]
        );
    }

    #[test]
    fn a_set_duration_survives_the_next_ticks() {
        let mut timer = testing_timer();

        timer.set(0, 2);
        assert_eq!(timer.cycle, TimerCycle::new("a", 2));

        // The cycle is derived from the elapsed time, so the tick that
        // used to discard the set duration now recomputes it.
        timer.update(0);
        assert_eq!(timer.cycle, TimerCycle::new("a", 2));

        timer.update(1);
        assert_eq!(timer.cycle, TimerCycle::new("a", 1));

        timer.update(2);
        assert_eq!(timer.cycle, TimerCycle::new("b", 2));
    }

    #[test]
    fn a_tick_changing_nothing_says_nothing() {
        let mut timer = testing_timer();

        timer.set(0, 2);

        // The tick lands in the same second as the set, so it recomputes
        // the duration the set already announced.
        assert_eq!(timer.update(0).into_iter().count(), 0);
        assert_eq!(timer.cycle, TimerCycle::new("a", 2));

        let events: Vec<_> = timer.update(1).into_iter().collect();
        assert_eq!(events, vec![TimerEvent::Running(TimerCycle::new("a", 1))]);
    }

    #[test]
    fn a_set_duration_is_clamped_to_the_cycle_length() {
        let mut timer = testing_timer();
        timer.update(2);
        assert_eq!(timer.cycle, TimerCycle::new("a", 1));

        // Cycle "a" lasts 3, so asking for 10 restarts it instead of
        // rewinding into the cycle before it, and the event says so.
        let events: Vec<_> = timer.set(2, 10).into_iter().collect();

        assert_eq!(events, vec![TimerEvent::Set(TimerCycle::new("a", 3))]);
        assert_eq!(timer.cycle, TimerCycle::new("a", 3));

        timer.update(3);
        assert_eq!(timer.cycle, TimerCycle::new("a", 2));
    }

    #[test]
    fn setting_a_cycle_to_zero_ends_it() {
        let mut timer = testing_timer();

        timer.set(0, 0);
        assert_eq!(timer.cycle, TimerCycle::new("a", 0));

        let events: Vec<_> = timer.update(0).into_iter().collect();

        assert_eq!(
            events,
            vec![
                TimerEvent::Ended(TimerCycle::new("a", 0)),
                TimerEvent::Began(TimerCycle::new("b", 2)),
            ]
        );
    }

    #[test]
    fn a_set_duration_holds_across_a_pause() {
        let mut timer = testing_timer();
        timer.pause(1);

        timer.set(1, 2);
        assert_eq!(timer.cycle, TimerCycle::new("a", 2));
        assert_eq!(timer.started_at, None);

        // Resuming ten seconds later resumes what was set, rather than
        // counting the pause as elapsed time.
        timer.resume(11);
        timer.update(11);
        assert_eq!(timer.cycle, TimerCycle::new("a", 2));

        timer.update(12);
        assert_eq!(timer.cycle, TimerCycle::new("a", 1));
    }

    #[test]
    fn setting_a_stopped_timer_does_nothing() {
        let mut timer = testing_timer();
        timer.state = TimerState::Stopped;
        let stopped = timer.clone();

        assert_eq!(timer.set(0, 2).into_iter().count(), 0);
        assert_eq!(timer, stopped);
    }

    #[test]
    fn a_paused_timer_can_be_stopped() {
        let mut timer = testing_timer();
        timer.update(2);
        timer.pause(2);

        let events: Vec<_> = timer.stop().into_iter().collect();

        assert_eq!(
            events,
            vec![
                TimerEvent::Ended(TimerCycle::new("a", 1)),
                TimerEvent::Stopped
            ]
        );
        assert_eq!(timer.state, TimerState::Stopped);
        assert_eq!(timer.cycle, TimerCycle::new("a", 3));
        assert_eq!(timer.elapsed, 0);
    }

    #[test]
    fn a_stopped_timer_stops_into_nothing() {
        let mut timer = testing_timer();
        timer.state = TimerState::Stopped;

        assert_eq!(timer.stop().into_iter().count(), 0);
    }

    #[test]
    fn cycles_lasting_no_time_are_inert() {
        let mut timer = Timer::new(TimerConfig {
            cycles: TimerCycles::from([TimerCycle::new("a", 0)]),
            ..Default::default()
        });

        timer.start(0);

        // No elapsed time can name a cycle here, so the tick and the set
        // both bail out rather than divide by the zero-length loop.
        assert_eq!(timer.update(10).into_iter().count(), 0);
        assert_eq!(timer.set(10, 1).into_iter().count(), 0);
    }
}
