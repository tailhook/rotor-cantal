#[macro_use] extern crate rotor;
extern crate rotor_cantal;
extern crate rotor_tools;
extern crate argparse;

use std::time::Duration;

use rotor::{Machine, EventSet, Scope, Response};
use rotor::void::{unreachable, Void};
use rotor_cantal::{Schedule, connect_localhost};
use rotor_tools::loop_ext::LoopExt;
use argparse::{ArgumentParser, StoreTrue};

pub struct Printer(bool);

rotor_compose!(enum Fsm/Seed<Context> {
    Print(Printer),
    Cantal(rotor_cantal::Fsm<Context>),
});

impl Machine for Printer {
    type Context = Context;
    type Seed = Void;
    fn create(seed: Self::Seed, _scope: &mut Scope<Context>)
        -> Response<Self, Void>
    {
        unreachable(seed)
    }
    fn ready(self, _events: EventSet, _scope: &mut Scope<Context>)
        -> Response<Self, Self::Seed>
    {
        unimplemented!();
    }
    fn spawned(self, _scope: &mut Scope<Context>) -> Response<Self, Self::Seed>
    {
        unimplemented!();
    }
    fn timeout(self, _scope: &mut Scope<Context>) -> Response<Self, Self::Seed>
    {
        unimplemented!();
    }
    fn wakeup(self, scope: &mut Scope<Context>) -> Response<Self, Self::Seed>
    {
        println!("{:#?}", scope.cantal.get_peers());
        if !self.0 {
            scope.shutdown_loop();
        }
        Response::ok(self)
    }
}

pub struct Context {
    cantal: Schedule,
}

fn main() {
    let mut monitor = false;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut monitor)
            .add_option(&["-m", "--monitor"], StoreTrue,
            "Poll the cantal every 10 seconds");
        ap.parse_args_or_exit();
    }

    let mut creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
    let schedule = creator.add_and_fetch(Fsm::Cantal, |scope| {
        connect_localhost(scope)
    }).unwrap();
    schedule.set_peers_interval(Duration::new(10, 0));
    let mut loop_inst = creator.instantiate(Context {
        cantal: schedule.clone(),
    });
    loop_inst.add_machine_with(|scope| {
        schedule.add_listener(scope.notifier());
        Response::ok(Fsm::Print(Printer(monitor)))
    }).unwrap();
    loop_inst.run().unwrap();
}
