use anyhow::Result;
use clap::Parser;
use log::info;
use time::timer::TimerState;

use crate::{
    config::TomlConfig,
    preset::{arg::PresetNameArg, config::TimerPrecision},
    protocol::arg::ProtocolArg,
};

/// Get the timer.
///
/// This command allows you to send a request to the server in order
/// to get the actual timer and display its value.
#[derive(Debug, Parser)]
pub struct GetTimerCommand {
    #[command(flatten)]
    pub preset: PresetNameArg,

    #[command(flatten)]
    pub protocol: ProtocolArg,
}

impl GetTimerCommand {
    pub async fn execute(self, config: &TomlConfig) -> Result<()> {
        info!("executing get timer command");

        let preset = config.get_preset(&self.preset.name)?;
        let client = self.protocol.to_client(&preset)?;

        let timer = client.get().await?;
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
}
