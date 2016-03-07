mod peers;

use std::fmt::Debug;
use state::State;
pub use self::peers::Peers;

use rotor::Time;
use rotor_http::client::Request;

pub trait Dataset: Debug {
    fn write_request(&self, req: &mut Request);
    fn parse_response(&self, data: &[u8], state: &mut State, tm: Time);
}
