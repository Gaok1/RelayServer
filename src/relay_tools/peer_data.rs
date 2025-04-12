// ============================
use std::net::SocketAddrV4;
use crate::relay_tools::RelayMap::now_ms;

pub type PeerId = u64;

#[derive(Debug)]
pub struct PeerData {
    pub peer_id: PeerId,
    pub peer_addr: SocketAddrV4,
    pub discovery_time: u128,
    pub waiting_punch: bool,
    stream: Option<std::net::TcpStream>,
}

impl  PeerData {
    fn clone_no_stream(&self) -> Self {
        Self {
            peer_id: self.peer_id,
            peer_addr: self.peer_addr,
            discovery_time: self.discovery_time,
            waiting_punch: self.waiting_punch,
            stream: None, // Não clona o stream
        }
    }

    pub fn new(peer_id: PeerId, peer_addr: SocketAddrV4) -> Self {
        Self {
            peer_id,
            peer_addr,
            discovery_time: now_ms(),
            waiting_punch: false,
            stream: None,
        }
    }
}
