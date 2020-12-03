use derive_more::Display;
use serde::{Deserialize, Serialize};

/// A unique hash identifying each peer
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub struct OpaquePeerId(String);

impl From<String> for OpaquePeerId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Into<String> for OpaquePeerId {
    fn into(self) -> String {
        self.0
    }
}
