#![feature(async_await)]
use std::future::Future;
use tokio_sync::oneshot;
pub trait Client {
    type Future: Future<Output = Response>;
    fn call(&mut self) -> Self::Future;
}
pub trait Broadcast {
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
pub struct FastPaxos<'a, B> {
    broadcast: &'a mut B,
}
impl<'a,  B> FastPaxos<'a, B>
    where
        B: Broadcast,
{
    pub async fn propose(&mut self) -> () {
        let (tx, rx) = oneshot::channel();
        let request = Request::new(tx);
        self.broadcast.broadcast(request).await;
        rx.await;
    }
}