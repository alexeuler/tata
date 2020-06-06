use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub struct OpaquePeerId(String);
