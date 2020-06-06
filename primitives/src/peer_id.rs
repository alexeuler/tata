use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub struct OpaquePeerId(String);

impl From<String> for OpaquePeerId {
    fn from(s: String) -> Self {
        Self(s)
    }
}
