#![feature(async_await)]
pub mod cluster {
    use crate::{
        error::{Error, Result},
        transport::{Client, Server},
    };
    use futures::{FutureExt, Stream, StreamExt};
    use std::{
        pin::Pin,
        task::{Context, Poll},
        time::{Duration, Instant},
    };
    use tokio_sync::watch;
    #[derive(Debug, Default, Clone)]
    pub struct Event;
    pub struct Cluster<S, C, T> {
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
}
mod error {
    use std::{error, fmt};
    pub type Result<T> = std::result::Result<T, Error>;
    type Source = Box<dyn error::Error + Send + Sync>;
    pub struct Error {
        kind: ErrorKind,
        source: Option<Source>,
    }
    #[derive(Debug)]
    pub(crate) enum ErrorKind {
        Start,
        Join,
        BrokenPipe,
        UnexpectedRequestType,
    }
    impl Error {
        pub(crate) fn new_broken_pipe(source: Option<Source>) -> Self {
            unimplemented!()
        }
    }
    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            unimplemented!()
        }
    }
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            unimplemented!()
        }
    }
}
pub mod transport {
    pub mod proto {
        use bytes::Bytes;
        use std::collections::HashMap;
        use uuid::Uuid;
        pub type ConfigId = i64;
        pub type RingNumber = i32;
        pub type Endpoint = String;
        #[derive(Debug)]
        pub struct NodeId(Uuid);
        #[derive(Debug)]
        pub enum RequestKind {
            PreJoin(PreJoinMessage),
            Join(JoinMessage),
            Probe,
            Consensus(Consensus),
        }
        #[derive(Debug)]
        pub enum ResponseKind {
            Join(JoinResponse),
            Response,
            Probe,
            Consensus(Consensus),
        }
        #[derive(Debug)]
        pub enum Consensus {
            FastRoundPhase2bMessage(FastRoundPhase2bMessage),
            Phase1aMessage(Phase1aMessage),
            Phase1bMessage(Phase1bMessage),
            Phase2aMessage(Phase2aMessage),
            Phase2bMessage(Phase2bMessage),
        }
        #[derive(Debug)]
        pub struct PreJoinMessage {
            pub sender: Endpoint,
            pub node_id: NodeId,
            pub ring_number: RingNumber,
            pub config_id: ConfigId,
        }
        #[derive(Debug)]
        pub struct JoinMessage {
            pub sender: Endpoint,
            pub node_id: NodeId,
            pub ring_number: Vec<RingNumber>,
            pub config_id: ConfigId,
        }
        #[derive(Debug)]
        pub struct JoinResponse {
            pub sender: Endpoint,
            pub status: JoinStatus,
            pub config_id: ConfigId,
            pub endpoints: Vec<Endpoint>,
            pub identifiers: Vec<NodeId>,
            pub cluster_metadata: HashMap<String, Metadata>,
        }
        #[derive(Debug)]
        pub enum JoinStatus {
            HostnameAlreadyInRing,
            NodeIdAlreadyInRing,
            SafeToJoin,
            ConfigChanged,
            MembershipRejected,
        }
        #[derive(Debug)]
        pub struct FastRoundPhase2bMessage {
            pub sender: Endpoint,
            pub config_id: ConfigId,
            pub endpoints: Vec<Endpoint>,
        }
        #[derive(Debug)]
        pub struct Phase1aMessage {
            pub sender: Endpoint,
            pub config_id: ConfigId,
            pub rank: Rank,
        }
        #[derive(Debug)]
        pub struct Phase1bMessage {
            pub sender: Endpoint,
            pub config_id: ConfigId,
            pub rnd: Rank,
            pub vrnd: Rank,
            pub vval: Vec<Endpoint>,
        }
        #[derive(Debug)]
        pub struct Phase2aMessage {
            pub sender: Endpoint,
            pub config_id: ConfigId,
            pub rnd: Rank,
            pub vval: Vec<Endpoint>,
        }
        #[derive(Debug)]
        pub struct Phase2bMessage {
            pub sender: Endpoint,
            pub config_id: ConfigId,
            pub rnd: Rank,
            pub endpoints: Vec<Endpoint>,
        }
        #[derive(Debug)]
        pub struct Metadata {
            pub metadata: HashMap<String, Bytes>,
        }
        #[derive(Debug)]
        pub struct Rank {
            pub round: u32,
            pub node_index: u32,
        }
    }
    use crate::Error;
    use futures::Stream;
    use std::future::Future;
    use tokio_sync::oneshot;
    pub trait Client {
        type Error: std::error::Error;
        type Future: Future<Output = Result<Response, Self::Error>>;
        fn call(&mut self, req: Request) -> Self::Future;
    }
    pub trait Server<T> {
        type Error: std::error::Error;
        type Stream: Stream<Item = Result<Request, Self::Error>> + Unpin;
        type Future: Future<Output = Result<Self::Stream, Self::Error>>;
        fn start(&mut self, target: T) -> Self::Future;
    }
    pub trait Broadcast {
        type Error: std::error::Error;
        type Future: Future<Output = Vec<Result<Response, Self::Error>>>;
        fn broadcast(&mut self, req: Request) -> Self::Future;
    }
    #[derive(Debug)]
    pub struct Request {
        kind: proto::RequestKind,
        res_tx: oneshot::Sender<crate::Result<Response>>,
    }
    #[derive(Debug)]
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
        async fn start_classic_round(&mut self) -> Result<()> {
            unimplemented!()
        }
        fn get_random_delay(&self) -> Duration {
            unimplemented!()
        }
    }
}
pub use self::error::{Error, Result};
