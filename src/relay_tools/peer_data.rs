use std::net::SocketAddrV4;

pub type PeerId = u64;
pub struct PeerData {
    pub peer_id : String,
    pub peer_addr : SocketAddrV4,
    pub discovery_time : u128, 
    pub waiting_punch : bool,
}
