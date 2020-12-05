use crate::reactor::use_reactor;
use futures::{
    stream::Stream,
    task::{Context, Poll},
};
use primitives::Event;
use std::collections::VecDeque;
use std::pin::Pin;

pub struct EventStream {
    top: usize,
    buffer: VecDeque<Event>,
}

impl EventStream {
    pub fn new() -> Self {
        Self {
            top: 0,
            buffer: VecDeque::new(),
        }
    }
}

impl Stream for EventStream {
    type Item = Event;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let self_mut = unsafe { self.get_unchecked_mut() };
        if let Some(event) = self_mut.buffer.pop_front() {
            return Poll::Ready(Some(event));
        }
        use_reactor(|reactor| {
            if self_mut.top == reactor.top() {
                reactor.register(cx.waker().clone());
            } else {
                while let Some(event) = reactor.get_event(self_mut.top) {
                    self_mut.buffer.push_back(event.clone());
                    self_mut.top += 1;
                }
            }
        });
        if let Some(event) = self_mut.buffer.pop_front() {
            return Poll::Ready(Some(event));
        }
        Poll::Pending
    }
}
