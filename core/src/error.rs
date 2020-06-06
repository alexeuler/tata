//! Global error module for `tata-cli` crate
use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

/// Global error type for `tata-cli` crate
#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Transport Io error: {}", _0)]
    TransportIo(libp2p::TransportError<std::io::Error>),
    #[display(fmt = "Address parse error: {}", _0)]
    Addr(libp2p::core::multiaddr::Error),
    #[display(fmt = "Io error: {}", _0)]
    Io(std::io::Error),
    #[display(fmt = "{}", _0)]
    Msg(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::TransportIo(e) => Some(e),
            Error::Addr(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::Msg(_) => None,
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(input: &'a str) -> Self {
        Error::Msg(input.to_string())
    }
}
