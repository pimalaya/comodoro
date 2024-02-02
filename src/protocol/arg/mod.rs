#[cfg(feature = "server")]
mod many;
#[cfg(feature = "client")]
mod one;

#[cfg(feature = "server")]
pub use self::many::ProtocolsArg;
#[cfg(feature = "client")]
pub use self::one::ProtocolArg;
