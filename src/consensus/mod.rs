use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::{Duration, Instant},
};

use futures::{stream::FuturesUnordered, Future, FutureExt, Stream, StreamExt};
use rand::Rng;
use tokio_sync::oneshot;
use tokio_timer::Delay;

mod paxos;

use paxos::Paxos;

use crate::{
    error::{Error, Result},
    transport::{
        proto::{self, ConfigId, Endpoint, Phase2bMessage},
        Broadcast, Client, Request, Response,
    },
};

const BASE_DELAY: u64 = 1000;

#[derive(Debug)]
enum CancelPaxos {}

pub struct FastPaxos<'a, C, B> {
    broadcast: &'a mut B,
    my_addr: Endpoint,
    size: usize,
    decided: AtomicBool,
    /// Channel used to communicate with upstream about decision
    decision_tx: oneshot::Sender<Vec<Endpoint>>,
    /// Channel the paxos instance will use to communicate with us about decision
    // paxos_rx: oneshot::Receiver<Vec<Endpoint>>,
    paxos: Option<Paxos<'a, C>>,
    /// Latest configuration we accepted.
    config_id: ConfigId,
    votes_received: HashSet<Endpoint>, // should be a bitset?
    votes_per_proposal: HashMap<Vec<Endpoint>, AtomicUsize>,
}

impl<'a, C, B> FastPaxos<'a, C, B>
where
    C: Client,
    B: Broadcast,
{
    pub fn new(
        my_addr: Endpoint,
        size: usize,
        client: &'a mut C,
        broadcast: &'a mut B,
        decision_tx: oneshot::Sender<Vec<Endpoint>>,
        proposal_rx: oneshot::Receiver<Vec<Endpoint>>,
        config_id: ConfigId,
    ) -> FastPaxos<'a, C, B> {
        unimplemented!();

    }

    pub async fn propose(&mut self, proposal: Vec<Endpoint>) -> Result<()> {
        let mut paxos_delay = Delay::new(Instant::now() + self.get_random_delay()).fuse();

        async {
            paxos_delay.await;
            self.start_classic_round().await;
        }
            .await;

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

        // TODO: better error type
        Ok(())
    }

    pub async fn handle_message(&mut self, request: Request) -> Result<Vec<Endpoint>> {
        unimplemented!();

    }

    async fn handle_fast_round<'b>(
        &'b mut self,
        request: &'b proto::FastRoundPhase2bMessage,
    ) -> crate::Result<Vec<Endpoint>> {
        unimplemented!();

    }

    async fn on_decide(&mut self, hosts: Vec<Endpoint>) -> Result<Vec<Endpoint>> {
        unimplemented!();

    }

    async fn start_classic_round(&mut self) -> Result<()> {
        unimplemented!();

    }

    fn get_random_delay(&self) -> Duration {
        unimplemented!();

    }
}
