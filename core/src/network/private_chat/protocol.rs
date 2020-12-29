use crate::error::Error;
use futures::{AsyncRead, AsyncWrite};
use futures_codec::{Framed, LengthCodec};
use libp2p::{
    core::{upgrade, UpgradeInfo},
    InboundUpgrade, OutboundUpgrade,
};
use primitives::PlainTextMessage;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

use super::error::Result;
use futures::prelude::*;

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
    type Output = (HandshakeMetadata, Framed<TSocket, LengthCodec>);
    type Error = Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_inbound(self, socket: TSocket, _: Self::Info) -> Self::Future {
        log::trace!("Upgrade inbound for private chat");
        Box::pin(async move {
            let mut framed_socket = Framed::new(socket, LengthCodec {});
            let metadata = if let Some(res) = framed_socket.next().await {
                let bytes = res?;
                let s = String::from_utf8(bytes.to_vec())?;
                serde_json::from_str(&s)?
            } else {
                Err("Private chat: upgrade stream is closed")?
            };
            let outbound_message = self.metadata_message();
            framed_socket.send(outbound_message.into()).await?;
            Ok((metadata, framed_socket))
        })
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for PrivateChatProtocol
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = (HandshakeMetadata, Framed<TSocket, LengthCodec>);
    type Error = Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_outbound(self, socket: TSocket, _: Self::Info) -> Self::Future {
        log::trace!("Upgrade outbound for private chat");
        Box::pin(async move {
            let outbound_message = self.metadata_message();
            let mut framed_socket = Framed::new(socket, LengthCodec {});
            framed_socket.send(outbound_message.into()).await?;
            if let Some(res) = framed_socket.next().await {
                let bytes = res?;
                let s = String::from_utf8(bytes.to_vec())?;
                let metadata = serde_json::from_str(&s)?;
                Ok((metadata, framed_socket))
            } else {
                Err("Private chat: upgrade stream is closed".into())
            }
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
