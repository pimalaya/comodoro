//! The transports carrying the timer protocol.
//!
//! The protocol is JSON-RPC 2.0 framed as NDJSON, which the
//! specification deliberately leaves transport-agnostic, so a connection
//! is just a byte stream. Two of them are supported: a Unix domain
//! socket, which gets filesystem permissions for free and opens no port,
//! and TCP, for the cases a local socket cannot serve.
//!
//! [`TimerAddress`] says where a server listens, [`TimerListener`]
//! accepts connections there, and [`TimerStream`] is one connection,
//! whichever transport carries it. Windows has supported `AF_UNIX`
//! stream sockets since build 1803, reached here through the uds_windows
//! shim, so the same path-based addressing works on every supported
//! platform.

use core::fmt;

use alloc::{format, string::String};

#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};
use std::{
    env, fs,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

use anyhow::{Context, Result, bail};
use log::{debug, warn};
#[cfg(windows)]
use uds_windows::{UnixListener, UnixStream};

/// The socket path used when the configuration names none.
///
/// Resolves to `$XDG_RUNTIME_DIR/comodoro.sock` when the variable is
/// set, which is the per-user directory the socket belongs in, and falls
/// back to the platform temporary directory everywhere else.
pub fn default_socket_path() -> PathBuf {
    let dir = env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .filter(|dir| dir.is_dir())
        .unwrap_or_else(env::temp_dir);

    dir.join("comodoro.sock")
}

/// Where a timer server listens and a timer client connects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimerAddress {
    /// A Unix domain socket, addressed by its path.
    UnixSocket(PathBuf),
    /// A TCP endpoint, addressed by its host and its port.
    Tcp {
        /// The host to reach the server at.
        host: String,
        /// The port the server listens on.
        port: u16,
    },
}

impl fmt::Display for TimerAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnixSocket(path) => write!(f, "{}", path.display()),
            Self::Tcp { host, port } => write!(f, "{host}:{port}"),
        }
    }
}

/// One connection carrying the timer protocol.
///
/// Both variants are plain blocking byte streams, and the protocol above
/// them cannot tell which one it is talking over.
#[derive(Debug)]
pub enum TimerStream {
    /// A connected Unix domain socket.
    UnixSocket(UnixStream),
    /// A connected TCP socket.
    Tcp(TcpStream),
}

impl TimerStream {
    /// Connects to the server listening at `address`.
    pub fn connect(address: &TimerAddress) -> Result<Self> {
        debug!("connect to timer server at {address}");

        match address {
            TimerAddress::UnixSocket(path) => {
                let stream = UnixStream::connect(path)
                    .with_context(|| format!("Connect to timer server at {address} error"))?;
                Ok(Self::UnixSocket(stream))
            }
            TimerAddress::Tcp { host, port } => {
                let stream = TcpStream::connect((host.as_str(), *port))
                    .with_context(|| format!("Connect to timer server at {address} error"))?;
                Ok(Self::Tcp(stream))
            }
        }
    }

    /// Clones the connection, so one half can read while the other
    /// writes.
    pub fn try_clone(&self) -> Result<Self> {
        match self {
            Self::UnixSocket(stream) => Ok(Self::UnixSocket(
                stream.try_clone().context("Clone timer socket error")?,
            )),
            Self::Tcp(stream) => Ok(Self::Tcp(
                stream.try_clone().context("Clone timer socket error")?,
            )),
        }
    }
}

impl Read for TimerStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::UnixSocket(stream) => stream.read(buf),
            Self::Tcp(stream) => stream.read(buf),
        }
    }
}

impl Write for TimerStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::UnixSocket(stream) => stream.write(buf),
            Self::Tcp(stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::UnixSocket(stream) => stream.flush(),
            Self::Tcp(stream) => stream.flush(),
        }
    }
}

/// A bound listener accepting timer connections.
#[derive(Debug)]
pub enum TimerListener {
    /// A bound Unix domain socket.
    UnixSocket(UnixListener),
    /// A bound TCP socket.
    Tcp(TcpListener),
}

impl TimerListener {
    /// Binds `address`, clearing a stale socket left by a crashed
    /// server.
    ///
    /// A socket file outlives the process that created it, so its mere
    /// presence proves nothing. Connecting to it does: a refused
    /// connection means nobody is listening and the file can go, while a
    /// successful one means a live server owns the address.
    pub fn bind(address: &TimerAddress) -> Result<Self> {
        debug!("listen at {address}");

        match address {
            TimerAddress::UnixSocket(path) => {
                if path.exists() {
                    if UnixStream::connect(path).is_ok() {
                        bail!("Socket {address} is already in use");
                    }

                    warn!("remove stale socket at {address}");
                    fs::remove_file(path)
                        .with_context(|| format!("Remove stale socket {address} error"))?;
                }

                let listener = UnixListener::bind(path)
                    .with_context(|| format!("Bind socket {address} error"))?;
                Ok(Self::UnixSocket(listener))
            }
            TimerAddress::Tcp { host, port } => {
                let listener = TcpListener::bind((host.as_str(), *port))
                    .with_context(|| format!("Bind socket {address} error"))?;
                Ok(Self::Tcp(listener))
            }
        }
    }

    /// Blocks until a client connects.
    pub fn accept(&self) -> Result<TimerStream> {
        match self {
            Self::UnixSocket(listener) => {
                let (stream, _) = listener.accept().context("Accept connection error")?;
                Ok(TimerStream::UnixSocket(stream))
            }
            Self::Tcp(listener) => {
                let (stream, _) = listener.accept().context("Accept connection error")?;
                Ok(TimerStream::Tcp(stream))
            }
        }
    }
}
