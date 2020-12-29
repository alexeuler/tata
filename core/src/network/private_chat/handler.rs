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

pub struct PrivateChatHandler {
    local_metadata: HandshakeMetadata,
    framed_socket: Option<Framed<NegotiatedSubstream, LengthCodec>>,
    pending_metadata: Option<HandshakeMetadata>,
    pending_sending_messages: VecDeque<PlainTextMessage>,
    pending_substream_open: bool,
    outgoing_message: Option<u64>,
    peer_id: Option<String>,
    errors: VecDeque<ErrorMessage>,
}

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
        log::debug!("Private chat: Injected fully negotiated inbound");
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
        log::debug!("Private chat: Injected fully negotiated outbound");
        self.pending_substream_open = false;
        let (metadata, framed_socket) = protocol;
        log::debug!("Peer Metadata: {:?}", metadata);
        self.framed_socket = Some(framed_socket);
        self.pending_metadata = Some(metadata);
    }

    fn inject_event(&mut self, event: InEvent) {
        match event {
            InEvent::SendMessage(message) => {
                self.peer_id = Some(message.from.clone());
                self.pending_sending_messages.push_back(message)
            }
        }
    }

    fn inject_dial_upgrade_error(&mut self, _info: (), error: ProtocolsHandlerUpgrErr<Error>) {
        log::error!("Error upgrading connection: {}", error);
        self.framed_socket = None;
        self.errors.push_back(ErrorMessage::FailedToDial {
            peer_id: self.peer_id(),
            reason: error.to_string(),
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
            return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error(e)));
        }
        if !self.pending_sending_messages.is_empty()
            && self.framed_socket.is_none()
            && !self.pending_substream_open
        {
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
                        self.outgoing_message = Some(message.timestamp);
                        let bytes = match serde_json::to_vec(&message) {
                            Ok(b) => b,
                            Err(e) => {
                                return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error(
                                    ErrorMessage::MessageValidation {
                                        timestamp: message.timestamp,
                                        reason: e.to_string(),
                                    },
                                )))
                            }
                        };
                        if let Err(e) = framed_socket.start_send_unpin(bytes.into()) {
                            return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error(
                                ErrorMessage::Network {
                                    peer_id: self.peer_id(),
                                    reason: e.to_string(),
                                },
                            )));
                        }
                    }
                }
                Poll::Pending => (),
            }
            // poll for sent message
            match framed_socket.poll_flush_unpin(cx) {
                Poll::Ready(_) => {
                    if let Some(timestamp) = self.outgoing_message.take() {
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
                            return Poll::Ready(ProtocolsHandlerEvent::Custom(
                                Event::ReceivedPlainTextMessage(message),
                            ));
                        }
                        Err(e) => {
                            return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error(
                                ErrorMessage::Other {
                                    reason: format!(
                                        "Failed to deserialize incoming message: {:02x?}",
                                        bytes
                                    ),
                                },
                            )))
                        }
                    }
                }
                Poll::Ready(Some(Err(e))) => {
                    return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error(
                        ErrorMessage::Network {
                            peer_id: self.peer_id(),
                            reason: e.to_string(),
                        },
                    )));
                }
                Poll::Ready(None) => {
                    log::debug!("Stream is closed");
                }
            }
        }
        Poll::Pending
    }
}

impl PrivateChatHandler {
    pub fn new(local_metadata: HandshakeMetadata) -> PrivateChatHandler {
        PrivateChatHandler {
            local_metadata,
            pending_metadata: None,
            pending_sending_messages: VecDeque::new(),
            outgoing_message: None,
            pending_substream_open: false,
            peer_id: None,
            framed_socket: None,
            errors: VecDeque::new(),
        }
    }

    fn peer_id(&self) -> String {
        self.peer_id
            .clone()
            .unwrap_or("Unknown peer_id".to_string())
    }
}
