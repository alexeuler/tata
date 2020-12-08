//! Global error module
use std::string::FromUtf8Error;

use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

/// Global error type
#[derive(Debug, Display, From)]
pub enum Error {
    /// Libp2p transport error
    #[display(fmt = "Transport Io error: {}", _0)]
    TransportIo(libp2p::TransportError<std::io::Error>),
    /// Address parase error
    #[display(fmt = "Address parse error: {}", _0)]
    Addr(libp2p::core::multiaddr::Error),
    /// Json conversion error
    #[display(fmt = "Json conversion error: {}", _0)]
    Json(serde_json::Error),
    /// UTF-8 conversion error
    #[display(fmt = "UTF-8 conversion error: {}", _0)]
    Utf8(FromUtf8Error),
    /// Generic I/O error
    #[display(fmt = "Io error: {}", _0)]
    Io(std::io::Error),
    /// Generic error
    #[display(fmt = "{}", _0)]
    Msg(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::TransportIo(e) => Some(e),
            Error::Addr(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::Utf8(e) => Some(e),
            Error::Msg(_) => None,
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(input: &'a str) -> Self {
        Error::Msg(input.to_string())
    }
}
