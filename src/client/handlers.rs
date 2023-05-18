use anyhow::Result;
use pimalaya_time::{Client, TimerState};

pub fn start(client: &dyn Client) -> Result<()> {
    client.start()?;
    Ok(())
}

pub fn get(client: &dyn Client) -> Result<()> {
    let timer = client.get()?;
    let cycle = &timer.cycle.name;

    match timer.state {
        TimerState::Stopped => println!("OFF"),
        TimerState::Paused => println!("[{cycle}] paused"),
        TimerState::Running if timer.cycle.duration < 60 => {
            println!("[{cycle}] {}s", timer.cycle.duration)
        }
        TimerState::Running => println!("[{cycle}] {}min", timer.cycle.duration / 60),
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
