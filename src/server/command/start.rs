#[cfg(unix)]
use std::os::unix::net::UnixListener;
use std::{
    io::{Read, Write},
    net::TcpListener,
    process,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use anyhow::{bail, Result};
use clap::Parser;
use convert_case::{Case, Casing};
use io_stream::runtimes::std::handle;
use io_timer::{
    server::coroutines::handle::{HandleRequest, HandleRequestResult},
    timer::{TimerConfig, TimerEvent, TimerLoop},
    Timer,
};
use log::{debug, error, warn};

use crate::{
    account::{arg::AccountNameArg, config::AccountConfig},
    protocol::{arg::ProtocolsArg, Protocol},
};

/// Start the server.
///
/// This command allows you to start the server using the given
/// configuration preset and protocols.
#[derive(Debug, Parser)]
pub struct StartServerCommand {
    #[command(flatten)]
    pub account: AccountNameArg,

    #[command(flatten)]
    pub protocols: ProtocolsArg,
}

impl StartServerCommand {
    pub fn execute(self, account: &AccountConfig) -> Result<()> {
        let timer = Arc::new(Mutex::new(Timer::new(TimerConfig {
            cycles: account.cycles.clone().into(),
            cycles_count: match account.cycles_count {
                Some(n) => TimerLoop::Fixed(n),
                None => TimerLoop::Infinite,
            },
        })));

        let (tx, rx) = mpsc::channel();

        // protocol listeners
        for protocol in Protocol::ALL {
            match protocol {
                #[cfg(unix)]
                Protocol::UnixSocket => {
                    let sock = match &*self.protocols {
                        Some(protocols) => {
                            if !protocols.contains(&Protocol::UnixSocket) {
                                continue;
                            }

                            let Some(sock) = &account.unix_socket else {
                                bail!("missing Unix socket configuration");
                            };

                            sock
                        }
                        None => {
                            let Some(sock) = &account.unix_socket else {
                                continue;
                            };

                            sock
                        }
                    };

                    debug!("enable Unix socket listener");
                    let listener = UnixListener::bind(&sock.path)?;
                    let timer = timer.clone();
                    let tx = tx.clone();

                    thread::spawn(move || loop {
                        debug!("wait for new Unix socket connection");
                        let stream = match listener.accept() {
                            Ok((stream, _)) => {
                                debug!("received new Unix socket connection");
                                stream
                            }
                            Err(err) => {
                                error!("cannot handle Unix socket connection: {err}");
                                continue;
                            }
                        };

                        let timer = timer.clone();
                        let tx = tx.clone();

                        if let Err(err) = handle_request(stream, timer, tx) {
                            error!("cannot handle Unix socket request: {err}");
                        }
                    });
                }
                Protocol::Tcp => {
                    let tcp = match &*self.protocols {
                        Some(protocols) => {
                            if !protocols.contains(&Protocol::Tcp) {
                                continue;
                            }

                            let Some(tcp) = &account.tcp else {
                                bail!("missing TCP configuration");
                            };

                            tcp
                        }
                        None => {
                            let Some(tcp) = &account.tcp else {
                                continue;
                            };

                            tcp
                        }
                    };

                    debug!("enable TCP listener");
                    let listener = TcpListener::bind((tcp.host.as_str(), tcp.port))?;
                    let timer = timer.clone();
                    let tx = tx.clone();

                    thread::spawn(move || loop {
                        debug!("wait for new TCP connection");
                        let stream = match listener.accept() {
                            Ok((stream, _)) => {
                                debug!("received new TCP connection");
                                stream
                            }
                            Err(err) => {
                                error!("cannot handle TCP connection: {err}");
                                continue;
                            }
                        };

                        let timer = timer.clone();
                        let tx = tx.clone();

                        if let Err(err) = handle_request(stream, timer, tx) {
                            error!("cannot handle TCP request: {err}");
                        }
                    });
                }
            }
        }

        // timer tick
        thread::spawn({
            let timer = timer.clone();
            let tx = tx.clone();

            move || loop {
                let events = match timer.lock() {
                    Ok(ref mut timer) => timer.update(),
                    Err(err) => {
                        error!("cannot lock timer: {err}");
                        process::exit(1)
                    }
                };

                for event in events {
                    if let Err(err) = tx.send(event.clone()) {
                        error!("cannot send timer event {event:?}: {err}");
                    }
                }

                thread::sleep(Duration::from_secs(1));
            }
        });

        // event handler
        while let Ok(event) = rx.recv() {
            debug!("received time event {event:?}");

            let hook_name = match event {
                TimerEvent::Started => String::from("on-timer-start"),
                TimerEvent::Began(cycle) => format!("on-{}-begin", cycle.name.to_case(Case::Kebab)),
                TimerEvent::Running(cycle) => {
                    format!("on-{}-running", cycle.name.to_case(Case::Kebab))
                }
                TimerEvent::Set(cycle) => format!("on-{}-set", cycle.name.to_case(Case::Kebab)),
                TimerEvent::Paused(cycle) => {
                    format!("on-{}-pause", cycle.name.to_case(Case::Kebab))
                }
                TimerEvent::Resumed(cycle) => {
                    format!("on-{}-resume", cycle.name.to_case(Case::Kebab))
                }
                TimerEvent::Ended(cycle) => {
                    format!("on-{}-end", cycle.name.to_case(Case::Kebab))
                }
                TimerEvent::Stopped => String::from("on-timer-stop"),
            };

            if let Some(hook) = account.hooks.get(&hook_name) {
                if let Err(err) = hook.exec() {
                    warn!("Error while executing hook: {err}")
                }
            }
        }

        Ok(())
    }
}

fn handle_request(
    mut stream: impl Read + Write,
    timer: Arc<Mutex<Timer>>,
    tx: Sender<TimerEvent>,
) -> Result<()> {
    let mut arg = None;
    let mut handler = HandleRequest::new();

    loop {
        let res = match timer.lock() {
            Ok(ref mut timer) => handler.resume(timer, arg.take()),
            Err(err) => {
                error!("cannot lock timer: {err}");
                process::exit(1);
            }
        };

        match res {
            HandleRequestResult::Ok(events) => {
                break for event in events {
                    if let Err(err) = tx.send(event.clone()) {
                        error!("cannot send timer event {event:?}: {err}");
                    }
                }
            }
            HandleRequestResult::Err(err) => {
                bail!(err);
            }
            HandleRequestResult::Io(io) => {
                arg = Some(handle(&mut stream, io)?);
            }
        }
    }

    Ok(())
}
