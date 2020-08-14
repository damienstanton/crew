pub use async_std::task;
use futures::task::AtomicWaker;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Weak},
    task::{Context, Poll},
};
use uuid::Uuid;

struct Work {
    waker: AtomicWaker,
}

impl Drop for Work {
    fn drop(&mut self) {
        self.waker.wake();
    }
}
#[allow(dead_code)]
pub(crate) struct Worker {
    work: Arc<Work>,
    pub(crate) id: String,
}

pub(crate) struct Crew {
    task: Arc<Work>,
}

impl Crew {
    pub(crate) fn new() -> Self {
        Self {
            task: Arc::new(Work {
                waker: AtomicWaker::new(),
            }),
        }
    }

    pub(crate) fn member(&self) -> Worker {
        Worker {
            work: self.task.clone(),
            id: Uuid::new_v4().to_string(),
        }
    }

    pub(crate) fn block(self) -> FutureTask {
        FutureTask {
            inner: Arc::downgrade(&self.task),
        }
    }
}
pub(crate) struct FutureTask {
    inner: Weak<Work>,
}

impl Future for FutureTask {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.upgrade() {
            Some(inner) => {
                inner.waker.register(cx.waker());
                Poll::Pending
            }
            None => return Poll::Ready(()),
        }
    }
}
