//! Private chat error module
use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

/// Private chat error type
#[derive(Debug, Display, From)]
pub enum Error {
    /// Libp2p transport error
    #[display(fmt = "Base58 decode error: {}", _0)]
    Base58(bs58::decode::Error),
    #[display(fmt = "PeerId decode from bytes error: {:?}", _0)]
    PeerId(Vec<u8>),
    /// Generic error
    #[display(fmt = "{}", _0)]
    Msg(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Base58(e) => Some(e),
            Error::PeerId(_e) => None,
            Error::Msg(_) => None,
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(input: &'a str) -> Self {
        Error::Msg(input.to_string())
    }
}
