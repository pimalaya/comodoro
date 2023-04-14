use anyhow::Result;
use pimalaya::time::pomodoro::{Client, TimerCycle, TimerState};

pub fn start(client: &dyn Client) -> Result<()> {
    client.start()?;
    Ok(())
}

pub fn get(client: &dyn Client) -> Result<()> {
    let timer = client.get()?;

    let cycle = match timer.cycle {
        TimerCycle::FirstWork => "W1",
        TimerCycle::FirstShortBreak => "SB1",
        TimerCycle::SecondWork => "W2",
        TimerCycle::SecondShortBreak => "SB2",
        TimerCycle::LongBreak => "LB",
    };

    match timer.state {
        TimerState::Stopped => println!("OFF"),
        TimerState::Paused => println!("[{cycle}] paused"),
        TimerState::Running if timer.value < 60 => println!("[{cycle}] {}s", timer.value),
        TimerState::Running => println!("[{cycle}] {}min", timer.value / 60),
    };

    Ok(())
}

pub fn pause(client: &dyn Client) -> Result<()> {
    client.pause()?;
    Ok(())
}

pub fn resume(client: &dyn Client) -> Result<()> {
    client.resume()?;
    Ok(())
}

pub fn stop(client: &dyn Client) -> Result<()> {
    client.stop()?;
    Ok(())
}
