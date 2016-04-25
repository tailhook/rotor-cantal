use std::sync::Arc;
use std::time::Duration;

use rustc_serialize::json::Json;
use rotor::Notifier;

use state::PrivateState;
use {Schedule, PeersState, RemoteQuery};

impl Schedule {
    pub fn set_peers_interval(&self, interval: Duration) {
        let mut state = self.0.lock().expect("cantal lock");
        state.peers_interval = Some(interval);
        state.changed();
    }
    pub fn clear_peers_interval(&self) {
        let mut state = self.0.lock().expect("cantal lock");
        state.peers_interval = None;
    }
    pub fn get_peers(&self) -> Option<Arc<PeersState>> {
        self.0.lock().expect("cantal lock").peers.clone()
    }
    pub fn set_remote_query_json(&self, json: &Json, interval: Duration) {
        let mut state = self.0.lock().expect("cantal lock");
        state.remote_query_task = Some((interval,
            Arc::new(json.to_string().into_bytes().into_boxed_slice())));
        state.changed();
    }
    pub fn clear_remote_query(&self) {
        let mut state = self.0.lock().expect("cantal lock");
        state.remote_query_task = None;
    }
    pub fn get_remote_query(&self) -> Option<Arc<RemoteQuery>> {
        self.0.lock().expect("cantal lock").remote_query.clone()
    }
    pub fn add_listener(&self, notifier: Notifier) {
        let mut state = self.0.lock().expect("cantal lock");
        state.add_listener(notifier);
    }
}
