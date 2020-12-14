use serde::{Deserialize, Serialize};
/// Metadata for p2p exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
}
