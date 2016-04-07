use rotor::Time;
use time::Timespec;

/// Info about the peer
///
/// We currently include only a subset of data reported by cantal here.
/// Mostly things that are unlikely to change in future. This will be fixed
/// when cantal grows stable API.
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct PeerInfo {
    pub id: String,
    pub hostname: String,
    pub name: String,
    pub primary_addr: Option<String>,
    pub addresses: Vec<String>,
    /// Known to this host, unixtime in milliseconds
    pub known_since: u64,
    /// Last report directly to this node unixtime in milliseconds
    pub last_report_direct: Option<u64>,
}

#[derive(Debug)]
pub struct PeersState {
    pub timestamp: Timespec,
    pub received: Time,
    pub peers: Vec<PeerInfo>,
}

