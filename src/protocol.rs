use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, ValueEnum};
use convert_case::{Case, Casing};
use pimalaya_process::Cmd;
use pimalaya_time::{Client, Server, ServerBuilder, ServerEvent, TcpBind, TcpClient, TimerEvent};
use serde::{Deserialize, Serialize};
use std::io;

use crate::{PresetConfig, PresetKind, PresetKindOrCyclesConfig};

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    #[cfg(feature = "tcp-client")]
    Tcp,
    #[default]
    None,
}

impl Protocol {
    pub fn to_server(config: &PresetConfig, protocols: Vec<&Protocol>) -> Result<Server> {
        let mut server = ServerBuilder::new().with_cycles_count(config.cycles_count);

        match &config.preset_or_cycles {
            PresetKindOrCyclesConfig::Preset(PresetKind::PresetPomodoro) => {
                server = server.with_pomodoro_config();
            }
            PresetKindOrCyclesConfig::Preset(PresetKind::Preset52_17) => {
                server = server.with_52_17_config();
            }
            PresetKindOrCyclesConfig::Cycles(cycles) => {
                server = server.with_cycles(cycles.clone());
            }
        }

        let hooks = config.hooks.clone();
        server = server.with_server_handler(move |evt| {
            let hook_name = match evt {
                ServerEvent::Started => String::from("on-server-start"),
                ServerEvent::Stopping => String::from("on-server-stopping"),
                ServerEvent::Stopped => String::from("on-server-stop"),
            };

            if let Some(cmd) = hooks.get(&hook_name) {
                Cmd::from(cmd.as_str()).run().map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("cannot execute server hook {hook_name}: {err}"),
                    )
                })?;
            }

            Ok(())
        });

        let hooks = config.hooks.clone();
        server = server.with_timer_handler(move |evt| {
            let hook_name = match evt {
                TimerEvent::Started => String::from("on-timer-start"),
                TimerEvent::Began(cycle) => format!("on-{}-begin", cycle.name).to_case(Case::Kebab),
                TimerEvent::Running(cycle) => {
                    format!("on-{}-running", cycle.name).to_case(Case::Kebab)
                }
                TimerEvent::Set(cycle) => format!("on-{}-set", cycle.name).to_case(Case::Kebab),
                TimerEvent::Paused(cycle) => {
                    format!("on-{}-pause", cycle.name).to_case(Case::Kebab)
                }
                TimerEvent::Resumed(cycle) => {
                    format!("on-{}-resume", cycle.name).to_case(Case::Kebab)
                }
                TimerEvent::Ended(cycle) => {
                    format!("on-{}-end", cycle.name.to_lowercase()).to_case(Case::Kebab)
                }
                TimerEvent::Stopped => String::from("on-timer-stop"),
            };

            if let Some(cmd) = hooks.get(&hook_name) {
                Cmd::from(cmd.as_str()).run().map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("cannot execute timer hook {hook_name}: {err}"),
                    )
                })?;
            }

            Ok(())
        });

        let protocols = if protocols.is_empty() {
            vec![
                #[cfg(feature = "tcp-binder")]
                &Self::Tcp,
            ]
        } else {
            protocols
        };

        for protocol in protocols {
            match protocol {
                #[cfg(feature = "tcp-binder")]
                Protocol::Tcp => {
                    if let Some(ref tcp_config) = config.tcp {
                        let binder = TcpBind::new(&tcp_config.host, tcp_config.port);
                        server = server.with_binder(binder);
                    }
                }
                Protocol::None => (),
            }
        }

        Ok(server.build()?)
    }

    pub fn to_client(&self, config: &PresetConfig) -> Result<Box<dyn Client>> {
        match self {
            #[cfg(feature = "tcp-client")]
            Self::Tcp => {
                if let Some(ref config) = config.tcp {
                    Ok(TcpClient::new(&config.host, config.port))
                } else {
                    Err(anyhow!("cannot build tcp client: missing tcp config"))
                }
            }
            Self::None => Err(anyhow!("cannot build client: missing protocol")),
        }
    }
}

impl ValueEnum for Protocol {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        match input {
            #[cfg(any(feature = "tcp-binder", feature = "tcp-client"))]
            p if "tcp" == p || ignore_case && p.eq_ignore_ascii_case("tcp") => Ok(Self::Tcp),
            p => Err(format!("invalid protocol {p}")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            #[cfg(any(feature = "tcp-binder", feature = "tcp-client"))]
            Self::Tcp,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            #[cfg(any(feature = "tcp-binder", feature = "tcp-client"))]
            Self::Tcp => Some(PossibleValue::new("tcp")),
            Self::None => None,
        }
    }
}

impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            #[cfg(any(feature = "tcp-binder", feature = "tcp-client"))]
            Self::Tcp => "tcp".into(),
            Self::None => "none".into(),
        }
    }
}
