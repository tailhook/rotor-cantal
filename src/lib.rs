//! The experimental API to [cantal] monitoring service
//!
//! The cantal doesn't have stable API yet, so anything here may change
//!
//! [cantal]: http://cantal.readthedocs.org

extern crate time;
extern crate rotor;
extern crate cbor;
extern crate rotor_http;
extern crate rustc_serialize;
#[macro_use] extern crate log;
#[macro_use] extern crate probor;

mod peers;
mod connection;
mod generator;
mod datasets;
mod state;
mod schedule;
mod query;
mod key;

use std::sync::{Arc, Mutex};

use rotor::mio::tcp::TcpStream;

use generator::Generator;
use state::State;

/// A state machine object, just add in to the loop
pub type Fsm<C> = rotor_http::client::Persistent<Generator<C>, TcpStream>;

#[derive(Clone, Debug)]
pub struct Schedule(Arc<Mutex<State>>);

pub use peers::{PeerInfo, PeersState};
pub use key::{Key, KeyVisitor};
pub use query::*;
pub use connection::{connect_localhost, connect_addr};
