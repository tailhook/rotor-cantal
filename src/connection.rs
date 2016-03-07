use std::sync::{Arc, Mutex};
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

use rotor::{GenericScope, Void, Response};
use rotor_http::client::{Persistent};

use state::{State, PrivateState};
use {Fsm, Schedule};

/// Usually cantal should be on a localhost
pub fn connect_localhost<S: GenericScope, C>(scope: &mut S)
    -> Response<(Fsm<C>, Schedule), Void>
{
    connect_addr(scope,
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 22682)))
}

pub fn connect_addr<S, C>(scope: &mut S, addr: SocketAddr)
    -> Response<(Fsm<C>, Schedule), Void>
    where S: GenericScope
{
    let arc = Arc::new(Mutex::new(State::new(scope.notifier())));
    Persistent::connect(scope, addr, arc.clone()).wrap(|fsm| {
        (fsm, Schedule(arc))
    })
}
