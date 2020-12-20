use crate::error::Error;
use futures::{AsyncRead, AsyncWrite};
use libp2p::{
    core::{upgrade, UpgradeInfo},
    InboundUpgrade, OutboundUpgrade,
};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

pub struct PrivateChatProtocol {
    local_metadata: HandshakeMetadata,
}

impl UpgradeInfo for PrivateChatProtocol {
    type Info = &'static [u8];
    type InfoIter = std::iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        std::iter::once(b"/private_chat/1.0.0")
    }
}

impl<TSocket> InboundUpgrade<TSocket> for PrivateChatProtocol
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = (HandshakeMetadata, TSocket);
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_inbound(self, mut socket: TSocket, _: Self::Info) -> Self::Future {
        Box::pin(async move {
            let packet = upgrade::read_one(&mut socket, 2048).await?;
            let s = String::from_utf8(packet)?;
            let metadata = serde_json::from_str(&s)?;
            let outbound_message = self.metadata_message();
            upgrade::write_one(&mut socket, &outbound_message).await?;

            Ok((metadata, socket))
        })
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for PrivateChatProtocol
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = (HandshakeMetadata, TSocket);
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_outbound(self, mut socket: TSocket, _: Self::Info) -> Self::Future {
        Box::pin(async move {
            let outbound_message = self.metadata_message();
            upgrade::write_one(&mut socket, &outbound_message).await?;
            let packet = upgrade::read_one(&mut socket, 2048).await?;
            let s = String::from_utf8(packet)?;
            let metadata = serde_json::from_str(&s)?;
            Ok((metadata, socket))
        })
    }
}

impl PrivateChatProtocol {
    pub fn new(local_metadata: HandshakeMetadata) -> Self {
        Self { local_metadata }
    }

    fn metadata_message(&self) -> Vec<u8> {
        serde_json::to_vec(&self.local_metadata).expect("Infallible conversion; qed")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeMetadata {
    pub name: String,
}
