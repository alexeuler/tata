use crate::error::{Error, Result};
use futures::prelude::*;
use futures::{AsyncRead, AsyncWrite};
use futures_codec::{Framed, LengthCodec};
use libp2p::{core::UpgradeInfo, InboundUpgrade, OutboundUpgrade};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
/// Metadata exchanged on handshake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeMetadata {
    /// Human name of the peer
    pub name: String,
}

/// Protocol struct that knows how to upgrade
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
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output>> + Send>>;

    fn upgrade_inbound(self, socket: TSocket, _: Self::Info) -> Self::Future {
        log::trace!("Upgrade inbound for private chat");
        Box::pin(async move {
            let mut framed_socket = Framed::new(socket, LengthCodec {});
            let metadata = receive_metadata(&mut framed_socket).await?;
            send_metadata(&mut framed_socket, self.local_metadata).await?;
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
            let mut framed_socket = Framed::new(socket, LengthCodec {});
            send_metadata(&mut framed_socket, self.local_metadata).await?;
            let metadata = receive_metadata(&mut framed_socket).await?;
            Ok((metadata, framed_socket))
        })
    }
}

impl PrivateChatProtocol {
    pub fn new(local_metadata: HandshakeMetadata) -> Self {
        Self { local_metadata }
    }
}

async fn receive_metadata<T>(socket: &mut Framed<T, LengthCodec>) -> Result<HandshakeMetadata>
where
    T: AsyncRead + Unpin,
{
    let metadata_res = socket
        .next()
        .await
        .ok_or("Private chat: upgrade stream is closed")?;
    log::trace!("Received metadata");
    let bytes = metadata_res?;
    let s = String::from_utf8(bytes.to_vec())?;
    let metadata = serde_json::from_str(&s)?;
    Ok(metadata)
}

async fn send_metadata<T>(
    socket: &mut Framed<T, LengthCodec>,
    metadata: HandshakeMetadata,
) -> Result<()>
where
    T: AsyncWrite + Unpin,
{
    let message = serde_json::to_vec(&metadata)?;
    socket.send(message.into()).await?;
    log::trace!("Sent metadata");
    Ok(())
}
