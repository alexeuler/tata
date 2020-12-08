use crate::Result;
use futures::{future, prelude::*};
use libp2p::{core::UpgradeInfo, swarm::NegotiatedSubstream, InboundUpgrade, OutboundUpgrade};
use serde::{Deserialize, Serialize};
use std::iter;

#[derive(Default, Debug, Copy, Clone)]
pub struct MetadataProtocol;

impl UpgradeInfo for MetadataProtocol {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/tata/metadata/1.0.0")
    }
}

impl InboundUpgrade<NegotiatedSubstream> for MetadataProtocol {
    type Output = NegotiatedSubstream;
    type Error = ();
    type Future = future::Ready<std::result::Result<Self::Output, Self::Error>>;

    fn upgrade_inbound(self, stream: NegotiatedSubstream, _: Self::Info) -> Self::Future {
        future::ok(stream)
    }
}

impl OutboundUpgrade<NegotiatedSubstream> for MetadataProtocol {
    type Output = NegotiatedSubstream;
    type Error = ();
    type Future = future::Ready<std::result::Result<Self::Output, Self::Error>>;

    fn upgrade_outbound(self, stream: NegotiatedSubstream, _: Self::Info) -> Self::Future {
        future::ok(stream)
    }
}

/// Metadata for p2p exchange
#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
}

pub async fn send_metadata<S>(mut stream: S, metadata: Metadata) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let payload = serde_json::to_string(&metadata)?;
    stream.write_all(payload.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}

pub async fn recv_metadata<S>(mut stream: S) -> Result<Metadata>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).await?;
    let s = String::from_utf8(bytes)?;
    let metadata = serde_json::from_str::<Metadata>(&s)?;
    Ok(metadata)
}
