use anyhow::Result;
use time::{Client, TimerState};

use crate::{PresetConfig, TimerPrecision};

pub fn start(client: &dyn Client) -> Result<()> {
    client.start()?;
    Ok(())
}

pub fn get(preset: &PresetConfig, client: &dyn Client) -> Result<()> {
    let timer = client.get()?;
    let cycle = &timer.cycle.name;

    match timer.state {
        TimerState::Stopped => println!("OFF"),
        TimerState::Paused => println!("[{cycle}] paused"),
        TimerState::Running if timer.cycle.duration < 60 => {
            println!("[{cycle}] {}s", timer.cycle.duration)
        }
        TimerState::Running if timer.cycle.duration < 3600 => match preset.timer_precision {
            TimerPrecision::Second => println!(
                "[{cycle}] {}min {}s",
                timer.cycle.duration / 60,
                timer.cycle.duration % 60
            ),
            TimerPrecision::Minute | TimerPrecision::Hour => {
                println!("[{cycle}] {}min", timer.cycle.duration / 60,)
            }
        },
        TimerState::Running => match preset.timer_precision {
            TimerPrecision::Second => println!(
                "[{cycle}] {}h {}min {}s",
                timer.cycle.duration / 3600,
                (timer.cycle.duration % 3600) / 60,
                (timer.cycle.duration % 3600) % 60,
            ),
            TimerPrecision::Minute => println!(
                "[{cycle}] {}h {}min",
                timer.cycle.duration / 3600,
                (timer.cycle.duration % 3600) / 60,
            ),
            TimerPrecision::Hour => println!("[{cycle}] {}h", timer.cycle.duration / 3600),
        },
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
