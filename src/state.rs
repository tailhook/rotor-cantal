use std::cmp::max;
use std::sync::Arc;
use std::time::Duration;

use rotor::{Notifier, Time};

use datasets::{Dataset, Peers};
use peers::PeersState;



#[derive(Debug)]
pub struct State {
    pub min_retry: Duration,
    pub peers_interval: Option<Duration>,
    pub peers: Option<Arc<PeersState>>,
    peers_attempt: Option<Time>,
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
                Ok(Box::new(Peers))
            } else {
                Err(next)
            }
        } else {
            // TODO(tailhook) this breaks abstractions a little bit
            Err(now + Duration::new(120, 0))
        }
    }
    fn changed(&self) {
        self.scheduler_notifier.wakeup().expect("wakeup state machine")
    }
}
