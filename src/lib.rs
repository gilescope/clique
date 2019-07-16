#![feature(async_await)]
mod error {
    pub type Result<T> = std::result::Result<T, Error>;
    pub struct Error {}
}
pub mod transport {
    pub mod proto {
        pub type ConfigId = i64;
        pub type Endpoint = String;
        pub enum RequestKind {
            Consensus(),
        }
        pub enum Consensus {
            FastRoundPhase2bMessage(),
        }
        pub struct FastRoundPhase2bMessage {}
        pub struct Phase2bMessage {}
    }
    use std::future::Future;
    use tokio_sync::oneshot;
    pub trait Client {
        type Error: std::error::Error;
        type Future: Future<Output = Result<Response, Self::Error>>;
        fn call(&mut self, req: Request) -> Self::Future;
    }
    pub trait Broadcast {
        type Error: std::error::Error;
        type Future: Future<Output = Vec<Result<Response, Self::Error>>>;
        fn broadcast(&mut self, req: Request) -> Self::Future;
    }
    pub struct Request {}
    pub struct Response {}
    impl Request {
        pub fn new(
            res_tx: oneshot::Sender<crate::Result<Response>>,
          //  kind: proto::RequestKind,
        ) -> Self {
            unimplemented!()
        }
    }
}
mod consensus {
    use futures::{stream::FuturesUnordered, Future, FutureExt, Stream, StreamExt};
    use std::{
        collections::{HashMap, HashSet},
        sync::atomic::{AtomicBool, AtomicUsize, Ordering},
        time::{Duration, Instant},
    };
    use tokio_sync::oneshot;
    use tokio_timer::Delay;
    mod paxos {
        pub struct Paxos<'a, C> {
            client: &'a mut C,
        }
    }
    use crate::{
        error::{Error, Result},
        transport::{
            proto::{self, ConfigId, Endpoint, Phase2bMessage},
            Broadcast, Client, Request, Response,
        },
    };
    use paxos::Paxos;
    pub struct FastPaxos<'a, C, B> {
        broadcast: &'a mut B,
        my_addr: Endpoint,
        paxos: Option<Paxos<'a, C>>,
        config_id: ConfigId,
    }
    impl<'a, C, B> FastPaxos<'a, C, B>
    where
        C: Client,
        B: Broadcast,
    {
        pub async fn propose(&mut self, proposal: Vec<Endpoint>) -> Result<()> {
            let (tx, rx) = oneshot::channel();
            let request = Request::new(tx);
            self.broadcast.broadcast(request).await;
            rx.await;
            Ok(())
        }
        fn get_random_delay(&self) -> Duration {
            unimplemented!()
        }
    }
}
pub use self::error::{Error, Result};
