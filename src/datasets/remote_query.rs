use std::io::Cursor;
use std::time::Instant;
use std::sync::{Arc};
use std::collections::HashMap;
use probor::{decode, Decoder, Config};

use rotor::Time;
use rotor_http::client::Request;
use rotor_http::client::Version::Http11;

use datasets::Dataset;
use state::{State, PrivateState};


#[derive(Debug)]
pub struct RemoteQuery(pub Arc<Box<[u8]>>);


impl Dataset for RemoteQuery {
    fn write_request(&self, req: &mut Request) {
        req.start("POST", "/remote/query_by_host.cbor", Http11);
        req.add_length(self.0.len() as u64).unwrap();
        req.done_headers().unwrap();
        req.write_body(&self.0);
        req.done();
    }
    fn parse_response(&self, data: &[u8], state: &mut State, tm: Time) {
        let mut dec = Decoder::new(Config::default(), Cursor::new(data));
        let parsed = decode(&mut dec)
            .map_err(|e| error!("Error parsing /query_by_host.cbor: {}", e));
        match parsed {
            Ok::<HashMap<String, ::Dataset>,()>(x) => {
                state.remote_query = Some(Arc::new(::RemoteQuery {
                    received: tm,
                    timestamp: Instant::now(),
                    items: x,
                }));
                state.wakeup_listeners();
            }
            Err(()) => {}
        }
    }
}
