use super::reactor::use_reactor;
use futures::{
    stream::Stream,
    task::{Context, Poll},
};
use primitives::Event;
use std::collections::VecDeque;
use std::pin::Pin;

pub struct NetworkEventStream {
    reactor_cursor: Option<usize>,
    buffer: VecDeque<Event>,
}

impl NetworkEventStream {
    pub fn new() -> Self {
        Self {
            reactor_cursor: None,
            buffer: VecDeque::new(),
        }
    }
}

impl Stream for NetworkEventStream {
    type Item = Event;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let self_mut = unsafe { self.get_unchecked_mut() };
        if self_mut.reactor_cursor.is_none() {
            use_reactor(|reactor| {
                reactor.subscribe(cx.waker().clone());
                self_mut.reactor_cursor = Some(reactor.total());
            });
            return Poll::Pending;
        }
        if let Some(event) = self_mut.buffer.pop_front() {
            return Poll::Ready(Some(event));
        }
        use_reactor(|reactor| {
            let cursor = self_mut
                .reactor_cursor
                .as_mut()
                .expect("Reactor cursor in Some; qed");
            // We're in sync with reactor
            if *cursor == reactor.total() {
                reactor.subscribe(cx.waker().clone());
            } else {
                while let Some(event) = reactor.get_event(*cursor) {
                    self_mut.buffer.push_back(event.clone());
                    *cursor += 1;
                }
            }
        });
        if let Some(event) = self_mut.buffer.pop_front() {
            return Poll::Ready(Some(event));
        }
        Poll::Pending
    }
}
