pub mod arg;

use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, ValueEnum};
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[cfg(feature = "tcp-client")]
use time::client::tcp::TcpClient;
#[cfg(feature = "tcp-binder")]
use time::server::tcp::TcpBind;
use time::{
    client::Client,
    server::{Server, ServerBuilder, ServerEvent},
    timer::TimerEvent,
};

use crate::preset::config::{PresetConfig, PresetKind};

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    #[cfg(feature = "tcp-client")]
    Tcp,
    #[default]
    None,
}

impl Protocol {
    pub async fn into_server(
        config: Arc<PresetConfig>,
        protocols: Vec<Protocol>,
    ) -> Result<Server> {
        let mut server = ServerBuilder::new().with_cycles_count(config.cycles_count);

        match config.preset.as_ref() {
            Some(PresetKind::PresetPomodoro) => {
                server = server.with_pomodoro_config();
            }
            Some(PresetKind::Preset52_17) => {
                server = server.with_52_17_config();
            }
            None => {
                server = server.with_cycles(config.cycles.clone());
            }
        }

        let config_clone = config.clone();
        server = server.with_server_handler(move |evt| {
            let hook_name = match evt {
                ServerEvent::Started => String::from("on-server-start"),
                ServerEvent::Stopping => String::from("on-server-stopping"),
                ServerEvent::Stopped => String::from("on-server-stop"),
            };

            let hook = config_clone.hooks.get(&hook_name).cloned();

            async move {
                if let Some(hook) = hook {
                    hook.exec(&hook_name).await;
                }

                Ok(())
            }
        });

        let config_clone = config.clone();
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

            let hook = config_clone.hooks.get(&hook_name).cloned();

            async move {
                if let Some(hook) = hook {
                    hook.exec(&hook_name).await;
                }

                Ok(())
            }
        });

        let protocols = if protocols.is_empty() {
            vec![
                #[cfg(feature = "tcp-binder")]
                Self::Tcp,
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
            #[cfg(feature = "tcp-any")]
            p if "tcp" == p || ignore_case && p.eq_ignore_ascii_case("tcp") => Ok(Self::Tcp),
            p => Err(format!("invalid protocol {p}")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            #[cfg(feature = "tcp-any")]
            Self::Tcp,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            #[cfg(feature = "tcp-any")]
            Self::Tcp => Some(PossibleValue::new("tcp")),
            Self::None => None,
        }
    }
}

impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            #[cfg(feature = "tcp-any")]
            Self::Tcp => "tcp".into(),
            Self::None => "none".into(),
        }
    }
}
