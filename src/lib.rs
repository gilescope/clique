#![feature(async_await)]
mod error {
    use std::{error, fmt};
    pub type Result<T> = std::result::Result<T, Error>;
    type Source = Box<dyn error::Error + Send + Sync>;
    pub struct Error {
        kind: ErrorKind,
        source: Option<Source>,
    }
    pub(crate) enum ErrorKind {}
}
pub mod transport {
    pub mod proto {
        use bytes::Bytes;
        use std::collections::HashMap;
        use uuid::Uuid;
        pub type ConfigId = i64;
        pub type Endpoint = String;
        pub struct NodeId(Uuid);
        pub enum ResponseKind {}
        pub enum RequestKind {
            Consensus(Consensus),
        }
        pub enum Consensus {
            FastRoundPhase2bMessage(FastRoundPhase2bMessage),
            Phase1aMessage(Phase1aMessage),
            Phase1bMessage(Phase1bMessage),
            Phase2aMessage(Phase2aMessage),
            Phase2bMessage(Phase2bMessage),
        }
        pub struct FastRoundPhase2bMessage {
            pub sender: Endpoint,
            pub config_id: ConfigId,
            pub endpoints: Vec<Endpoint>,
        }
        pub struct Phase1aMessage {
            pub sender: Endpoint,
            pub config_id: ConfigId,
            pub rank: Rank,
        }
        pub struct Phase1bMessage {
            pub sender: Endpoint,
            pub config_id: ConfigId,
            pub rnd: Rank,
            pub vrnd: Rank,
            pub vval: Vec<Endpoint>,
        }
        pub struct Phase2aMessage {
            pub sender: Endpoint,
            pub config_id: ConfigId,
            pub rnd: Rank,
            pub vval: Vec<Endpoint>,
        }
        pub struct Phase2bMessage {
            pub sender: Endpoint,
            pub config_id: ConfigId,
            pub rnd: Rank,
            pub endpoints: Vec<Endpoint>,
        }
        pub struct Rank {
            pub round: u32,
            pub node_index: u32,
        }
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
    pub struct Request {
        kind: proto::RequestKind,
        res_tx: oneshot::Sender<crate::Result<Response>>,
    }
    pub struct Response {
        kind: proto::ResponseKind,
    }
    impl Request {
        pub fn new(
            res_tx: oneshot::Sender<crate::Result<Response>>,
            kind: proto::RequestKind,
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
        use crate::{
            error::Result,
            transport::{
                proto::{ConfigId, Endpoint, Phase1bMessage, Phase2aMessage, Rank},
                Client, Request, Response,
            },
        };
        pub struct Paxos<'a, C> {
            client: &'a mut C,
            size: usize,
            my_addr: Endpoint,
            rnd: Rank,
            vrnd: Rank,
            vval: Vec<Endpoint>,
            crnd: Rank,
            cval: Vec<Endpoint>,
            config_id: ConfigId,
            phase_1b_messages: Vec<Phase1bMessage>,
            phase_2a_messages: Vec<Phase2aMessage>,
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
        size: usize,
        decided: AtomicBool,
        decision_tx: oneshot::Sender<Vec<Endpoint>>,
        paxos: Option<Paxos<'a, C>>,
        config_id: ConfigId,
        votes_received: HashSet<Endpoint>,
        votes_per_proposal: HashMap<Vec<Endpoint>, AtomicUsize>,
    }
    impl<'a, C, B> FastPaxos<'a, C, B>
    where
        C: Client,
        B: Broadcast,
    {
        pub async fn propose(&mut self, proposal: Vec<Endpoint>) -> Result<()> {
            let mut paxos_delay = Delay::new(Instant::now() + self.get_random_delay()).fuse();
            async { unimplemented!() }.await;
            let (tx, rx) = oneshot::channel();
            let kind = proto::RequestKind::Consensus(proto::Consensus::FastRoundPhase2bMessage(
                proto::FastRoundPhase2bMessage {
                    sender: self.my_addr.clone(),
                    config_id: self.config_id,
                    endpoints: proposal,
                },
            ));
            let request = Request::new(tx, kind);
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
