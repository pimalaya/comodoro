#[cfg(unix)]
use std::{
    fs,
    io::{Error, ErrorKind},
    os::unix::net::{UnixListener, UnixStream},
};
use std::{
    io::{Read, Write},
    net::TcpListener,
    process::exit,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use anyhow::{bail, Result};
use clap::Parser;
use convert_case::{Case, Casing};
use io_hook::exec::*;
#[cfg(feature = "notify")]
use io_notify::runtimes::std as notify;
#[cfg(feature = "command")]
use io_process::runtimes::std as process;
use io_socket::runtimes::std_stream as socket;
use io_time::{
    coroutines::{
        now::*,
        server::{TimerRequestHandle, TimerRequestHandleArg, TimerRequestHandleResult},
        sleep_until::*,
    },
    runtimes::std as time,
    timer::{Timer, TimerConfig, TimerEvent, TimerLoop},
};
use log::{debug, error};

use crate::{
    config::AccountConfig,
    protocol::{Protocol, ProtocolsArg, ALL_PROTOCOLS},
};

/// Start the server.
///
/// This command allows you to start the server using the given
/// configuration preset and protocols.
#[derive(Debug, Parser)]
pub struct StartServerCommand {
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

        for protocol in ALL_PROTOCOLS {
            match protocol {
                #[cfg(unix)]
                Protocol::UnixSocket => {
                    let sock = match &*self.protocols {
                        Some(protocols) => {
                            if !protocols.contains(&Protocol::UnixSocket) {
                                continue;
                            }

                            let Some(sock) = &account.unix_socket else {
                                bail!("Missing Unix socket configuration");
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
                    let sock_path = sock.path.clone();

                    // check if socket is in use, otherwise remove file
                    if sock_path.exists() {
                        if UnixStream::connect(&sock_path).is_ok() {
                            bail!(Error::new(ErrorKind::AddrInUse, "Socket already in use"))
                        }

                        fs::remove_file(&sock_path)?
                    }

                    let listener = UnixListener::bind(&sock_path)?;
                    let timer = timer.clone();
                    let tx = tx.clone();

                    thread::spawn(move || loop {
                        let path = sock_path.display();
                        debug!("wait for new Unix socket connection at {path}");

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

                        if let Err(err) = handle_connection(stream, timer, tx) {
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
                                bail!("Missing TCP configuration");
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
                    let addr = listener.local_addr()?;
                    let timer = timer.clone();
                    let tx = tx.clone();

                    thread::spawn(move || loop {
                        debug!("wait for new TCP connection at {addr:?}");

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

                        if let Err(err) = handle_connection(stream, timer, tx) {
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
                let mut arg = None;
                let mut coroutine = TimeNow::new();

                let now = loop {
                    match coroutine.resume(arg) {
                        TimeNowResult::Ok { secs, .. } => break secs,
                        TimeNowResult::Io { input } => {
                            arg = Some(time::handle(input).expect("now"))
                        }
                        TimeNowResult::Err { err } => panic!("{err}"),
                    }
                };

                let mut arg = None;
                let mut coroutine = TimeSleepUntil::new(now + 1);

                loop {
                    match coroutine.resume(arg) {
                        TimeSleepUntilResult::Ok => break,
                        TimeSleepUntilResult::Io { input } => {
                            arg = Some(time::handle(input).expect("sleep"))
                        }
                        TimeSleepUntilResult::Err { err } => panic!("{err}"),
                    }
                }

                let events: Vec<TimerEvent> = match timer.lock() {
                    Ok(ref mut timer) => timer.update(now + 1).into_iter().collect(),
                    Err(err) => {
                        error!("cannot lock timer: {err}");
                        exit(1)
                    }
                };

                for event in events {
                    if let Err(err) = tx.send(event.clone()) {
                        error!("cannot send timer event {event:?}: {err}");
                    }
                }
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

            if let Some(hook) = account.hooks.get(&hook_name).cloned() {
                let mut arg = None;
                let mut coroutine = HookExec::new(hook);

                loop {
                    match coroutine.resume(arg.take()) {
                        HookExecResult::Ok => {
                            break;
                        }
                        HookExecResult::NotifyIo { input } => match notify::handle(input) {
                            Ok(output) => arg = Some(output.into()),
                            Err(err) => break error!("notify hook exec failure: {err}"),
                        },
                        HookExecResult::ProcessIo { input } => match process::handle(input) {
                            Ok(output) => arg = Some(output.into()),
                            Err(err) => break error!("process hook exec failure: {err}"),
                        },
                        HookExecResult::Err { err } => {
                            break error!("hook exec failure: {err}");
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn handle_connection(
    mut stream: impl Read + Write,
    timer: Arc<Mutex<Timer>>,
    tx: Sender<TimerEvent>,
) -> Result<()> {
    let mut server = TimerRequestHandle::new();
    let mut arg: Option<TimerRequestHandleArg> = None;

    loop {
        let result = match timer.lock() {
            Ok(ref mut timer) => server.resume(timer, arg.take()),
            Err(err) => {
                error!("cannot lock timer: {err}");
                exit(1);
            }
        };

        match result {
            TimerRequestHandleResult::Ok { events } => {
                for event in events {
                    if let Err(err) = tx.send(event.clone()) {
                        error!("cannot send timer event {event:?}: {err}");
                    }
                }
                break;
            }
            TimerRequestHandleResult::TimeIo { input } => {
                arg = Some(TimerRequestHandleArg::Time(time::handle(input)?));
            }
            TimerRequestHandleResult::Io { input } => {
                let handle = socket::handle(&mut stream, input);
                arg = Some(TimerRequestHandleArg::Socket(handle?));
            }
            TimerRequestHandleResult::Err { err } => bail!("{err}"),
        }
    }

    Ok(())
}
