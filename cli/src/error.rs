//! Global error module for `tata-cli` crate
use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

/// Global error type for `tata-cli` crate
#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Base58 error: {}", _0)]
    Base58(bs58::decode::Error),
    #[display(fmt = "{}", _0)]
    Msg(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Base58(e) => Some(e),
            Error::Msg(_) => None,
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(input: &'a str) -> Self {
        Error::Msg(input.to_string())
    }
}
