use std::sync::Arc;
use std::time::Duration;


use rotor::Notifier;
use state::PrivateState;
use {Schedule, PeersState};

impl Schedule {
    pub fn set_peers_interval(&self, interval: Duration) {
        let mut state = self.0.lock().expect("cantal lock");
        state.peers_interval = Some(interval);
        state.changed();
    }
    pub fn get_peers(&self) -> Option<Arc<PeersState>> {
        self.0.lock().expect("cantal lock").peers.clone()
    }
    pub fn add_listener(&self, notifier: Notifier) {
        let mut state = self.0.lock().expect("cantal lock");
        state.add_listener(notifier);
    }
}
