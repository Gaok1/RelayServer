// ============================
use crate::relay_tools::RelayMap::now_ms;
use std::net::SocketAddrV4;

pub type PublicKey = String;

#[derive(Debug)]
pub struct PeerData {
    pub public_key: PublicKey,
    pub peer_addr: SocketAddrV4,
    pub discovery_time: u128,
    pub waiting_punch: bool,
    pub waiting_for: Option<PublicKey>,
}

impl PeerData {
    fn clone_no_stream(&self) -> Self {
        Self {
            public_key: self.public_key.clone(),
            peer_addr: self.peer_addr,
            discovery_time: self.discovery_time,
            waiting_punch: self.waiting_punch,
            waiting_for: self.waiting_for.clone(),
        }
    }
    
    pub fn new(public_key: PublicKey, peer_addr: SocketAddrV4) -> Self {
        Self {
            public_key,
            peer_addr,
            discovery_time: now_ms(),
            waiting_punch: false,
            waiting_for: None,
        }
    }
}
