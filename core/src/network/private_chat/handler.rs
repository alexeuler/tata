use futures::prelude::*;
use futures_timer::Delay;
use libp2p::core::upgrade;
use libp2p::swarm::{
    KeepAlive, NegotiatedSubstream, ProtocolsHandler, ProtocolsHandlerEvent,
    ProtocolsHandlerUpgrErr, SubstreamProtocol,
};
use primitives::{ErrorMessage, Event, PlainTextMessage, Timestamp};

use super::protocol::{HandshakeMetadata, PrivateChatProtocol};
use crate::error::{Error, Result};
use futures_codec::{Framed, LengthCodec};
use std::{
    collections::{HashMap, VecDeque},
    error::Error as StdError,
    time::Duration,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

const INITIAL_RETRY: Duration = Duration::from_secs(1);
const RETRY_EXP: u32 = 2;
const MAX_RETRY: Duration = Duration::from_secs(120);

pub struct PrivateChatHandler {
    local_metadata: HandshakeMetadata,
    stream: Option<Framed<NegotiatedSubstream, LengthCodec>>,
    pending_metadata: Option<HandshakeMetadata>,
    pending_sending_messages: VecDeque<PlainTextMessage>,
    outgoing_message: Option<u64>,
    errors: VecDeque<ErrorMessage>,
    retry: Option<Delay>,
    retry_value: Duration,
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
        protocol: (HandshakeMetadata, NegotiatedSubstream),
        _: (),
    ) {
        let (metadata, stream) = protocol;
        self.stream = Some(Framed::new(stream, LengthCodec {}));
        self.pending_metadata = Some(metadata);
    }

    fn inject_fully_negotiated_outbound(
        &mut self,
        protocol: (HandshakeMetadata, NegotiatedSubstream),
        _: (),
    ) {
        let (metadata, stream) = protocol;
        self.stream = Some(Framed::new(stream, LengthCodec {}));
        self.pending_metadata = Some(metadata);
    }

    fn inject_event(&mut self, event: InEvent) {
        match event {
            InEvent::SendMessage(message) => self.pending_sending_messages.push_back(message),
        }
    }

    fn inject_dial_upgrade_error(&mut self, _info: (), error: ProtocolsHandlerUpgrErr<Error>) {
        log::error!("Error upgrading connection: {}", error);
        self.stream = None;
        self.errors.push_front(ErrorMessage::Unreachable);
        self.retry = Some(Delay::new(self.retry_value));
        self.retry_value *= RETRY_EXP;
        // Stop trying
        if self.retry_value > MAX_RETRY {
            self.retry_value = INITIAL_RETRY;
            self.retry = None;
        }
    }

    fn connection_keep_alive(&self) -> KeepAlive {
        KeepAlive::Yes
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<ProtocolsHandlerEvent<PrivateChatProtocol, (), Self::OutEvent, Self::Error>> {
        if let Some(_) = self.errors.pop_back() {
            return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error(
                ErrorMessage::Unreachable,
            )));
        }
        if let Some(stream) = self.stream.as_mut() {
            match stream.poll_ready_unpin(cx) {
                Poll::Ready(_) => {
                    if let Some(timestamp) = self.outgoing_message.take() {
                        return Poll::Ready(ProtocolsHandlerEvent::Custom(
                            Event::SentPlainTextMessage(Timestamp { timestamp }),
                        ));
                    }
                    if let Some(message) = self.pending_sending_messages.pop_front() {
                        let bytes = serde_json::to_vec(&message).expect("Infallible; qed");
                        log::debug!("Sending message with timestamp `{}`", message.timestamp);
                        stream.start_send_unpin(bytes.into());
                    }
                }
                _ => (),
            }
            match stream.poll_next_unpin(cx) {
                Poll::Pending => (),
                Poll::Ready(Some(Ok(bytes))) => {
                    match serde_json::from_slice::<PlainTextMessage>(&bytes) {
                        Ok(message) => {
                            log::debug!("Received message: {:?}", message);
                            return Poll::Ready(ProtocolsHandlerEvent::Custom(
                                Event::ReceivedPlainTextMessage(message),
                            ));
                        }
                        Err(e) => {
                            log::error!("Error parsing bytes to json: {:?}", bytes);
                            return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error(
                                ErrorMessage::Parse,
                            )));
                        }
                    }
                }
                Poll::Ready(Some(Err(e))) => {
                    log::error!("Network error: {:?}", e);
                    return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Error(
                        ErrorMessage::Network,
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
            stream: None,
            errors: VecDeque::new(),
            retry: None,
            retry_value: INITIAL_RETRY,
        }
    }
}

enum State {
    Initialized,
    Open,
}

#[derive(Debug)]
enum PrivateChatError {
    Finished,
    Other(Box<dyn StdError + Send + 'static>),
}

impl std::fmt::Display for PrivateChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrivateChatError::Finished => write!(f, "PrivateChat error: Finished")?,
            PrivateChatError::Other(e) => write!(f, "PrivateChat error: {}", e)?,
        }
        Ok(())
    }
}

impl StdError for PrivateChatError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            PrivateChatError::Finished => None,
            PrivateChatError::Other(e) => Some(&**e),
        }
    }
}
