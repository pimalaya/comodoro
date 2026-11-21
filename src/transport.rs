//! # Transport
//!
//! The byte streams carrying the timer protocol. JSON-RPC 2.0 leaves the
//! transport alone, so a connection is just a byte stream: a Unix domain
//! socket, which gets filesystem permissions for free and opens no port,
//! or TCP, for the cases a local socket cannot serve.
//!
//! [`TimerAddress`] says where a server listens, [`TimerListener`]
//! accepts connections there, and [`TimerStream`] is one connection,
//! whichever transport carries it. [`TimerTransportError`] tags the
//! failures a caller acts on: a busy address, a refused socket path.
//!
//! Windows has supported `AF_UNIX` stream sockets since build 1803,
//! reached here through the uds_windows shim, so the same path-based
//! addressing works on every supported platform.

#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};
use std::{
    env, error, fmt, fs,
    io::{self, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

use anyhow::{Context, Error, Result};
use log::{debug, warn};
#[cfg(windows)]
use uds_windows::{UnixListener, UnixStream};

/// The socket path used when the configuration names none.
///
/// `$XDG_RUNTIME_DIR/comodoro.sock` when the variable is set, that being
/// the per-user directory a socket belongs in, and the platform
/// temporary directory everywhere else.
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

/// Why a transport could not be bound or connected, attached to the
/// error so a caller tells the causes apart with `downcast_ref`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerTransportError {
    /// Something already listens at the address: another timer server
    /// on a socket, any program on a TCP port.
    AddressBusy,
    /// The platform refuses the socket path, as one longer than a socket
    /// address holds.
    SocketPathInvalid,
}

impl fmt::Display for TimerTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::AddressBusy => "The address is already in use",
            Self::SocketPathInvalid => "The platform refuses the socket path",
        })
    }
}

impl error::Error for TimerTransportError {}

/// One connection carrying the timer protocol.
///
/// Both variants are plain blocking byte streams, and the protocol above
/// them cannot tell which one it talks over.
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
                    .map_err(|err| tag(err, address))
                    .with_context(|| format!("Cannot connect to the timer server at {address}"))?;
                Ok(Self::UnixSocket(stream))
            }
            TimerAddress::Tcp { host, port } => {
                let stream = TcpStream::connect((host.as_str(), *port))
                    .with_context(|| format!("Cannot connect to the timer server at {address}"))?;
                Ok(Self::Tcp(stream))
            }
        }
    }

    /// Clones the connection, so one half reads while the other writes.
    pub fn try_clone(&self) -> Result<Self> {
        match self {
            Self::UnixSocket(stream) => Ok(Self::UnixSocket(
                stream
                    .try_clone()
                    .context("Cannot clone the timer connection")?,
            )),
            Self::Tcp(stream) => Ok(Self::Tcp(
                stream
                    .try_clone()
                    .context("Cannot clone the timer connection")?,
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
    /// Binds `address`, clearing a stale socket left by a crashed server.
    ///
    /// A socket file outlives its process, so its presence proves
    /// nothing. Connecting does: a refused connection means the file can
    /// go, a successful one that a live server owns the address, a busy
    /// one as [`TimerTransportError::AddressBusy`] says.
    pub fn bind(address: &TimerAddress) -> Result<Self> {
        debug!("listen at {address}");

        match address {
            TimerAddress::UnixSocket(path) => {
                if path.exists() {
                    match UnixStream::connect(path) {
                        Ok(_) => {
                            let err = Error::new(TimerTransportError::AddressBusy);
                            return Err(err.context(format!("Cannot listen at {address}")));
                        }
                        Err(err) if err.kind() == ErrorKind::InvalidInput => {
                            let err = tag(err, address);
                            return Err(err.context(format!("Cannot listen at {address}")));
                        }
                        Err(_) => {
                            warn!("remove stale socket at {address}");
                            fs::remove_file(path).with_context(|| {
                                format!("Cannot remove the stale socket {address}")
                            })?;
                        }
                    }
                }

                let listener = UnixListener::bind(path)
                    .map_err(|err| tag(err, address))
                    .with_context(|| format!("Cannot listen at {address}"))?;
                Ok(Self::UnixSocket(listener))
            }
            TimerAddress::Tcp { host, port } => {
                let listener = TcpListener::bind((host.as_str(), *port))
                    .map_err(|err| tag(err, address))
                    .with_context(|| format!("Cannot listen at {address}"))?;
                Ok(Self::Tcp(listener))
            }
        }
    }

    /// Blocks until a client connects.
    pub fn accept(&self) -> Result<TimerStream> {
        match self {
            Self::UnixSocket(listener) => {
                let (stream, _) = listener.accept().context("Cannot accept a connection")?;
                Ok(TimerStream::UnixSocket(stream))
            }
            Self::Tcp(listener) => {
                let (stream, _) = listener.accept().context("Cannot accept a connection")?;
                Ok(TimerStream::Tcp(stream))
            }
        }
    }
}

/// Tags an I/O error of a transport with the [`TimerTransportError`] it
/// means, when it means one: a busy address, or a socket path the
/// platform refuses.
fn tag(err: io::Error, address: &TimerAddress) -> Error {
    let kind = err.kind();
    let err = Error::new(err);

    match (address, kind) {
        (_, ErrorKind::AddrInUse) => err.context(TimerTransportError::AddressBusy),
        (TimerAddress::UnixSocket(_), ErrorKind::InvalidInput) => {
            err.context(TimerTransportError::SocketPathInvalid)
        }
        _ => err,
    }
}
