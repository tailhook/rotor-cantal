use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::marker::PhantomData;

use rotor::{Scope, Time};
use rotor_http::client::{Client, Request, Head, RecvMode, Requester};
use rotor_http::client::{Connection, Task};

use datasets::{Dataset};
use state::{State, PrivateState};

pub struct Generator<C> {
    state: Arc<Mutex<State>>,
    phantom: PhantomData<*const C>,
}

/// This struct is only public because it's used for associated type
pub struct RequestWrapper<C>(Box<Dataset>, Arc<Mutex<State>>,
                             PhantomData<*const C>);

impl<C> Generator<C> {
    fn next_request(self, scope: &mut Scope<C>) -> Task<Self> {
        let task = self.state.lock().expect("cantal state lock")
            .next_request(scope.now());
        match task {
            Ok(x) => {
                let lnk = self.state.clone();
                Task::Request(self, RequestWrapper(x, lnk, PhantomData))
            }
            Err(t) => Task::Sleep(self, t),
        }
    }
}

impl<C> Client for Generator<C> {
    type Requester = RequestWrapper<C>;
    type Seed = Arc<Mutex<State>>;
    fn create(seed: Self::Seed,
        _scope: &mut Scope<<Self::Requester as Requester>::Context>)
        -> Self
    {
        Generator { state: seed, phantom: PhantomData }
    }

    fn connection_idle(self,
        _connection: &Connection,
        scope: &mut Scope<<Self::Requester as Requester>::Context>)
        -> Task<Self>
    {
        self.next_request(scope)
    }

    fn wakeup(self,
        connection: &Connection,
        scope: &mut Scope<<Self::Requester as Requester>::Context>)
        -> Task<Self>
    {
        if connection.is_idle() {
            self.next_request(scope)
        } else {
            unreachable!();
        }
    }
    fn timeout(self,
        connection: &Connection,
        scope: &mut Scope<<Self::Requester as Requester>::Context>)
        -> Task<Self>
    {
        if connection.is_idle() {
            self.next_request(scope)
        } else {
            Task::Close
        }
    }
}

impl<C> Requester for RequestWrapper<C> {
    type Context = C;
    fn prepare_request(self, req: &mut Request) -> Option<Self> {
        self.0.write_request(req);
        Some(self)
    }
    fn headers_received(self, head: Head, _request: &mut Request,
        scope: &mut Scope<Self::Context>)
        -> Option<(Self, RecvMode, Time)>
    {
        // TODO(tailhook) check status code
        if head.code == 200 {
            Some((self, RecvMode::Buffered(1_048_576),
                scope.now() + Duration::new(10, 0)))
        } else {
            None
        }
    }
    fn response_received(self, data: &[u8], _request: &mut Request,
        scope: &mut Scope<Self::Context>)
    {
        let mut state = self.1.lock().expect("state locked");
        self.0.parse_response(data, &mut state, scope.now())
    }
    fn response_chunk(self, _chunk: &[u8], _request: &mut Request,
                      _scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        unreachable!();
    }
    fn response_end(self, _request: &mut Request,
        _scope: &mut Scope<Self::Context>)
    {
        unreachable!();
    }
    fn timeout(self, _request: &mut Request, _scope: &mut Scope<Self::Context>)
        -> Option<(Self, Time)>
    {
        None
    }
    fn wakeup(self, _request: &mut Request, _scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        unimplemented!();
    }
}
