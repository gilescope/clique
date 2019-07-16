#![feature(async_await)]
use std::future::Future;
use tokio_sync::oneshot;
pub trait Client {
    type Future: Future<Output = Response>;
    fn call(&mut self, req: Request) -> Self::Future;
}
pub trait Broadcast {
    type Error: std::error::Error;
    type Future: Future<Output = Vec<Response>>;
    fn broadcast(&mut self, req: Request) -> Self::Future;
}
pub struct Request {}
pub struct Response {}
impl Request {
    pub fn new(res_tx: oneshot::Sender<Response>) -> Self {
        unimplemented!()
    }
}

pub struct Paxos<'a, C> {
    client: &'a mut C,
}
pub struct FastPaxos<'a, C, B> {
    broadcast: &'a mut B,
    paxos: Option<Paxos<'a, C>>,
}
impl<'a, C, B> FastPaxos<'a, C, B>
    where
        C: Client,
        B: Broadcast,
{
    pub async fn propose(&mut self, proposal: Vec<String>) -> () {
        let (tx, rx) = oneshot::channel();
        let request = Request::new(tx);
        self.broadcast.broadcast(request).await;
        rx.await;
    }
}