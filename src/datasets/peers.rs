use std::str::from_utf8;
use std::sync::{Arc};
use time::get_time;
use rustc_serialize::json::decode;

use rotor::Time;
use rotor_http::client::Request;
use rotor_http::client::Version::Http11;

use datasets::Dataset;
use state::{State, PrivateState};
use peers::{PeersState, PeerInfo};


#[derive(Debug)]
pub struct Peers;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct PeersResponse {
    pub peers: Vec<PeerInfo>,
}


impl Dataset for Peers {
    fn write_request(&self, req: &mut Request) {
        req.start("GET", "/all_peers.json", Http11);
        req.done_headers().unwrap();
        req.done();
    }
    fn parse_response(&self, data: &[u8], state: &mut State, tm: Time) {
        let parsed = from_utf8(data)
            .map_err(|e| error!("Error parsing utf-8 for /all_peers.json: {}",
                                e))
            .and_then(|x| decode(x)
            .map_err(|e| error!("Error parsing json of /all_peers.json: {}",
                                e)));
        match parsed {
            Ok::<PeersResponse,()>(x) => {
                state.peers = Some(Arc::new(PeersState {
                    received: tm,
                    timestamp: get_time(),
                    peers: x.peers,
                }));
                state.wakeup_listeners();
            }
            Err(()) => {}
        }
    }
}
