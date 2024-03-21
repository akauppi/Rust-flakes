

/*
* Modifiers for a 'Stream'
*
* Usage:
*       use extra_stream as _;
*/

use futures_core::{
    future::Future,
    ready,
    stream::{self, Stream},
};
//use futures_util::fns::FnMut1;    // "module is private"

use std::pin::Pin;
use std::task::{Context, Poll};

use core::fmt;

#[must_use = "streams do nothing unless polled"]
pub struct Parcel<St, Fut, F>
    where St: Stream,
{
    #[pin]
    stream: St,
    f: F,
    #[pin]
    pending_fut: Option<Fut>,
    pending_item: Option<St::Item>,
}

impl<St, Fut, F> Parcel<St, Fut, F>
    where
        St: Stream,
        F: for<'a> FnMut1<&'a St::Item, Output = Fut>,
        Fut: Future<Output = bool>,
{
    pub fn new(stream: St, f: F) -> Self {
        Self { stream, f, pending_fut: None, pending_item: None }
    }

    delegate_access_inner!(stream, St, ());
}

impl<St, Fut, F> Stream for Packet<St, Fut, F>
    where
        St: Stream,
        F: for<'a> FnMut1<&'a St::Item, Output = Fut>,
        Fut: Future<Output = bool>,
{
    type Item = St::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<St::Item>> {
        let mut this = self.project();
        Poll::Ready(loop {
            if let Some(fut) = this.pending_fut.as_mut().as_pin_mut() {
                let res = ready!(fut.poll(cx));
                this.pending_fut.set(None);
                if res {
                    break this.pending_item.take();
                }
                *this.pending_item = None;
            } else if let Some(item) = ready!(this.stream.as_mut().poll_next(cx)) {
                this.pending_fut.set(Some(this.f.call_mut(&item)));
                *this.pending_item = Some(item);
            } else {
                break None;
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let pending_len = usize::from(self.pending_item.is_some());
        let (_, upper) = self.stream.size_hint();
        let upper = match upper {
            Some(x) => x.checked_add(pending_len),
            None => None,
        };
        (0, upper) // can't know a lower bound, due to the predicate
    }
}
