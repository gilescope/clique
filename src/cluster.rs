use crate::{
    error::{Error, Result},
    //membership::Membership,
    transport::{Client, Server},
};
use futures::{FutureExt, Stream, StreamExt};
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tokio_sync::watch;
use tokio_timer::Interval;

#[derive(Debug, Default, Clone)]
pub struct Event;

pub struct Cluster<S, C, T> {
    //membership: Membership,
    server: S,
    client: C,
    listen_target: T,
    event_tx: watch::Sender<Event>,
    handle: Handle,
}

#[derive(Clone)]
pub struct Handle {
    event_rx: watch::Receiver<Event>,
}

impl<S, C, T> Cluster<S, C, T>
where
    S: Server<T>,
    C: Client + Clone,
    T: Clone,
{
    pub fn new(server: S, client: C, listen_target: T) -> Self {
        unimplemented!();
    }

    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    pub async fn start(self) -> Result<()> {
        unimplemented!();

    }
}

impl Stream for Handle {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.event_rx).poll_next(cx)
    }
}
