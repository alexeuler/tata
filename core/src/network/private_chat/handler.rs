use super::protocol::{HandshakeMetadata, PrivateChatProtocol};
use crate::error::Error;
use futures::prelude::*;
use futures_codec::{Framed, LengthCodec};
use libp2p::swarm::{
    KeepAlive, NegotiatedSubstream, ProtocolsHandler, ProtocolsHandlerEvent,
    ProtocolsHandlerUpgrErr, SubstreamProtocol,
};
use primitives::{ErrorMessage, Event, PlainTextMessage};
use std::collections::VecDeque;
use std::task::{Context, Poll};

/// Protocol handler for private chat. Handles sending and receiving messages
/// and sending peer metadata after the handshake.
pub struct PrivateChatHandler {
    local_metadata: HandshakeMetadata,
    framed_socket: Option<Framed<NegotiatedSubstream, LengthCodec>>,
    pending_metadata: Option<HandshakeMetadata>,
    pending_sending_messages: VecDeque<PlainTextMessage>,
    pending_substream_open: bool,
    outgoing_message: Option<u64>,
    errors: VecDeque<ErrorMessage>,
}

/// Event coming from behavior to notify about
/// the new incoming message from user to be sent
#[derive(Debug, Clone)]
pub enum InEvent {
    SendMessage(PlainTextMessage),
}

impl ProtocolsHandler for PrivateChatHandler {
    type InEvent = InEvent;
    type OutEvent = Event;
    type Error = Error;
    type InboundProtocol = PrivateChatProtocol;
    type OutboundProtocol = PrivateChatProtocol;
    type OutboundOpenInfo = ();
    type InboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<PrivateChatProtocol, ()> {
        SubstreamProtocol::new(PrivateChatProtocol::new(self.local_metadata.clone()), ())
    }

    fn inject_fully_negotiated_inbound(
        &mut self,
        protocol: (HandshakeMetadata, Framed<NegotiatedSubstream, LengthCodec>),
        _: (),
    ) {
        log::debug!("Injected fully negotiated inbound");
        self.pending_substream_open = false;
        let (metadata, framed_socket) = protocol;
        log::debug!("Received peer metadata: {:?}", metadata);
        self.framed_socket = Some(framed_socket);
        self.pending_metadata = Some(metadata);
    }

    fn inject_fully_negotiated_outbound(
        &mut self,
        protocol: (HandshakeMetadata, Framed<NegotiatedSubstream, LengthCodec>),
        _: (),
    ) {
        log::debug!("Injected fully negotiated outbound");
        self.pending_substream_open = false;
        let (metadata, framed_socket) = protocol;
        log::debug!("Received peer metadata: {:?}", metadata);
        self.framed_socket = Some(framed_socket);
        self.pending_metadata = Some(metadata);
    }

    fn inject_event(&mut self, event: InEvent) {
        match event {
            InEvent::SendMessage(message) => self.pending_sending_messages.push_back(message),
        }
    }

    fn inject_dial_upgrade_error(&mut self, _info: (), error: ProtocolsHandlerUpgrErr<Error>) {
        log::error!("Error upgrading connection: {}", error);
        self.framed_socket = None;
        self.errors.push_back(ErrorMessage::FailedToDial {
            cause: error.to_string(),
        });
    }

    fn connection_keep_alive(&self) -> KeepAlive {
        KeepAlive::Yes
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<ProtocolsHandlerEvent<PrivateChatProtocol, (), Self::OutEvent, Self::Error>> {
        if let Some(e) = self.errors.pop_front() {
            return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error { error: e }));
        }
        if let Some(metadata) = self.pending_metadata.take() {
            return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::ReceivedMetadata {
                name: metadata.name,
            }));
        }
        if !self.pending_sending_messages.is_empty()
            && self.framed_socket.is_none()
            && !self.pending_substream_open
        {
            log::debug!("Opening substream");
            self.pending_substream_open = true;
            return Poll::Ready(ProtocolsHandlerEvent::OutboundSubstreamRequest {
                protocol: SubstreamProtocol::new(
                    PrivateChatProtocol::new(self.local_metadata.clone()),
                    (),
                ),
            });
        }
        if let Some(framed_socket) = self.framed_socket.as_mut() {
            match framed_socket.poll_ready_unpin(cx) {
                Poll::Ready(_) => {
                    if let Some(message) = self.pending_sending_messages.pop_front() {
                        log::debug!("Sending message with timestamp: {}", message.timestamp);
                        self.outgoing_message = Some(message.timestamp);
                        let bytes = match serde_json::to_vec(&message) {
                            Ok(b) => b,
                            Err(e) => {
                                return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error {
                                    error: ErrorMessage::MessageValidation {
                                        timestamp: message.timestamp,
                                        cause: e.to_string(),
                                    },
                                }))
                            }
                        };
                        if let Err(e) = framed_socket.start_send_unpin(bytes.into()) {
                            return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error {
                                error: ErrorMessage::Network {
                                    cause: e.to_string(),
                                },
                            }));
                        }
                    }
                }
                Poll::Pending => (),
            }
            // poll for sent message
            match framed_socket.poll_flush_unpin(cx) {
                Poll::Ready(_) => {
                    if let Some(timestamp) = self.outgoing_message.take() {
                        log::debug!("Sent message with timestamp: {}", timestamp);
                        return Poll::Ready(ProtocolsHandlerEvent::Custom(
                            Event::SentPlainTextMessage { timestamp },
                        ));
                    }
                }
                _ => (),
            }
            match framed_socket.poll_next_unpin(cx) {
                Poll::Pending => (),
                Poll::Ready(Some(Ok(bytes))) => {
                    match serde_json::from_slice::<PlainTextMessage>(&bytes) {
                        Ok(message) => {
                            log::debug!("Received message: {:?}", message);
                            return Poll::Ready(ProtocolsHandlerEvent::Custom(
                                Event::ReceivedPlainTextMessage { message },
                            ));
                        }
                        Err(e) => {
                            return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error {
                                error: ErrorMessage::Other {
                                    cause: format!(
                                    "Failed to deserialize incoming message: {:02x?}. Reason: {}",
                                    bytes, e
                                ),
                                },
                            }))
                        }
                    }
                }
                Poll::Ready(Some(Err(e))) => {
                    log::error!("Error on the receiving stream: {}", e);
                    return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error {
                        error: ErrorMessage::Network {
                            cause: e.to_string(),
                        },
                    }));
                }
                Poll::Ready(None) => {
                    log::warn!("Stream is closed");
                }
            }
        }
        Poll::Pending
    }
}

impl PrivateChatHandler {
    /// Creates a new handler. `local_metadata` is required for the initial exchange with a peer.
    pub fn new(local_metadata: HandshakeMetadata) -> PrivateChatHandler {
        PrivateChatHandler {
            local_metadata,
            pending_metadata: None,
            pending_sending_messages: VecDeque::new(),
            outgoing_message: None,
            pending_substream_open: false,
            framed_socket: None,
            errors: VecDeque::new(),
        }
    }
}
