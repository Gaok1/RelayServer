// ============================
use crate::relay_tools::RelayMap::now_ms;
use std::net::SocketAddrV4;

pub type PeerId = u64;

#[derive(Debug)]
pub struct PeerData {
    pub peer_id: PeerId,
    pub peer_addr: SocketAddrV4,
    pub discovery_time: u128,
    pub waiting_punch: bool,
    pub waiting_for: Option<PeerId>,
}

impl PeerData {
    fn clone_no_stream(&self) -> Self {
        Self {
            peer_id: self.peer_id,
            peer_addr: self.peer_addr,
            discovery_time: self.discovery_time,
            waiting_punch: self.waiting_punch,
            waiting_for: self.waiting_for,
        }
    }

    pub fn new(peer_id: PeerId, peer_addr: SocketAddrV4) -> Self {
        Self {
            peer_id,
            peer_addr,
            discovery_time: now_ms(),
            waiting_punch: false,
            waiting_for: None,
        }
    }
}
