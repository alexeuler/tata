use futures::prelude::*;
use futures_timer::Delay;
use libp2p::swarm::{
    KeepAlive, NegotiatedSubstream, ProtocolsHandler, ProtocolsHandlerEvent,
    ProtocolsHandlerUpgrErr, SubstreamProtocol,
};
use primitives::{Event, Metadata};

use super::protocol::{recv_metadata, send_metadata, MetadataProtocol};
use crate::error::{Error, Result};
use std::{collections::VecDeque, error::Error as StdError, time::Duration};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct MetadataHandler {
    inbound: Option<Pin<Box<dyn Future<Output = Result<Metadata>> + Send>>>,
    outbound: Option<Pin<Box<dyn Future<Output = Result<()>> + Send>>>,
    timer: Delay,
    local_metadata: Metadata,
    errors: VecDeque<Error>,
    state: State,
}

impl ProtocolsHandler for MetadataHandler {
    type InEvent = ();
    type OutEvent = Event;
    type Error = MetadataError;
    type InboundProtocol = MetadataProtocol;
    type OutboundProtocol = MetadataProtocol;
    type OutboundOpenInfo = ();
    type InboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<MetadataProtocol, ()> {
        SubstreamProtocol::new(MetadataProtocol, ())
    }

    fn inject_fully_negotiated_inbound(&mut self, stream: NegotiatedSubstream, (): ()) {
        self.inbound = Some(recv_metadata(stream).boxed());
        self.state = State::Open;
    }

    fn inject_fully_negotiated_outbound(&mut self, stream: NegotiatedSubstream, (): ()) {
        self.outbound = Some(send_metadata(stream, self.local_metadata).boxed());
        self.state = State::Open;
    }

    fn inject_event(&mut self, _: ()) {}

    fn inject_dial_upgrade_error(&mut self, _info: (), error: ProtocolsHandlerUpgrErr<()>) {
        self.outbound = None; // Request a new substream on the next `poll`.
        self.errors.push_front(
            match error {
                // Note: This timeout only covers protocol negotiation.
                ProtocolsHandlerUpgrErr::Timeout => "Timeout upgrading protocol",
                e => &format!("{:?}", e),
            }
            .into(),
        )
    }

    fn connection_keep_alive(&self) -> KeepAlive {
        KeepAlive::No
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<ProtocolsHandlerEvent<MetadataProtocol, (), Self::OutEvent, Self::Error>> {
        match self.state {
            State::Initialized => return Poll::Pending,
            State::Open => {
                if self.inbound.is_none() && self.outbound.is_none() {
                    return Poll::Ready(ProtocolsHandlerEvent::Close(MetadataError::Finished));
                }
            }
        };
        match self.timer.boxed().poll_unpin(cx) {
            Poll::Pending => (),
            Poll::Ready(_) => {
                self.inbound = None;
                self.outbound = None;
                return Poll::Ready(ProtocolsHandlerEvent::Close(MetadataError::Finished));
            }
        }
        if let Some(fut) = self.inbound.as_mut() {
            match Pin::new(fut).poll(cx) {
                Poll::Pending => (),
                Poll::Ready(Err(e)) => {
                    log::error!("Error on inbound metadata: {}", e);
                    ()
                }
                Poll::Ready(Ok(metadata)) => {
                    self.inbound = None;
                    return Poll::Ready(ProtocolsHandlerEvent::Custom(Event::Metadata(metadata)));
                }
            }
        }
        if let Some(fut) = self.outbound.as_mut() {
            match Pin::new(fut).poll(cx) {
                Poll::Ready(_) => {
                    self.outbound = None;
                }
                Poll::Pending => (),
            }
        }
        Poll::Pending
    }
}

impl MetadataHandler {
    pub fn new(local_metadata: Metadata, timeout: Duration) -> MetadataHandler {
        MetadataHandler {
            inbound: None,
            outbound: None,
            timer: Delay::new(timeout),
            local_metadata,
            errors: VecDeque::new(),
            state: State::Initialized,
        }
    }
}

enum State {
    Initialized,
    Open,
}

#[derive(Debug)]
enum MetadataError {
    Finished,
    Other(Box<dyn StdError + Send + 'static>),
}

impl std::fmt::Display for MetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetadataError::Finished => write!(f, "Metadata error: Finished")?,
            MetadataError::Other(e) => write!(f, "Metadata error: {}", e)?,
        }
        Ok(())
    }
}

impl StdError for MetadataError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            MetadataError::Finished => None,
            MetadataError::Other(e) => Some(&**e),
        }
    }
}
