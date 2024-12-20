use core::{pin::Pin, task::{Context, Poll}};

pub struct IdleTask {

}

impl IdleTask {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Future for IdleTask {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}