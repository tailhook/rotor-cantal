use std::cmp::{max, min};
use std::sync::Arc;
use std::time::Duration;

use rotor::{Notifier, Time};

use datasets::{self, Dataset};
use peers::PeersState;
use query::RemoteQuery;



#[derive(Debug)]
pub struct State {
    pub min_retry: Duration,
    pub peers_interval: Option<Duration>,
    pub remote_query_task: Option<(Duration, Arc<Box<[u8]>>)>,
    pub peers: Option<Arc<PeersState>>,
    pub remote_query: Option<Arc<RemoteQuery>>,
    peers_attempt: Option<Time>,
    remote_query_attempt: Option<Time>,
    scheduler_notifier: Notifier,
    listeners: Vec<Notifier>,
}

pub trait PrivateState {
    fn new(notifier: Notifier) -> State;
    fn add_listener(&mut self, listener: Notifier);
    fn wakeup_listeners(&self);
    fn next_request(&mut self, now: Time) -> Result<Box<Dataset>, Time>;
    fn changed(&self);
}

impl PrivateState for State {
    fn new(notifier: Notifier) -> State {
        State {
            min_retry: Duration::from_millis(50),
            peers_interval: None,
            peers_attempt: None,
            peers: None,
            remote_query_task: None,
            remote_query_attempt: None,
            remote_query: None,
            scheduler_notifier: notifier,
            listeners: Vec::new(),
        }
    }
    fn add_listener(&mut self, listener: Notifier) {
        self.listeners.push(listener);
    }
    fn wakeup_listeners(&self) {
        for i in &self.listeners {
            i.wakeup().expect("wakeup cantal listener");
        }
    }
    fn next_request(&mut self, now: Time) -> Result<Box<Dataset>, Time> {
        // TODO(tailhook) select minimal time rather than prioritizing
        // the peers over the query
        let mut sleep = now + Duration::new(120, 0);
        if let Some(ivl) = self.peers_interval {
            let mut next;
            if let Some(last) = self.peers.as_ref().map(|x| x.received) {
                next = last + ivl;
            } else {
                next = now;
            }
            if let Some(attempt) = self.peers_attempt {
                next = max(attempt + self.min_retry, next);
            }
            if next <= now {
                self.peers_attempt = Some(now);
                return Ok(Box::new(datasets::Peers));
            } else {
                sleep = min(next, sleep);
            }
        }
        if let Some((ivl, ref data)) = self.remote_query_task {
            let mut next;
            if let Some(last) = self.remote_query.as_ref().map(|x| x.received) {
                next = last + ivl;
            } else {
                next = now;
            }
            if let Some(attempt) = self.remote_query_attempt {
                next = max(attempt + self.min_retry, next);
            }
            if next <= now {
                self.remote_query_attempt = Some(now);
                return Ok(Box::new(datasets::RemoteQuery(data.clone())));
            } else {
                sleep = min(next, sleep);
            }
        }
        return Err(sleep)
    }
    fn changed(&self) {
        self.scheduler_notifier.wakeup().expect("wakeup state machine")
    }
}
