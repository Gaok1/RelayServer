use std::net::SocketAddrV4;

use super::peer_data::{PeerData, PeerId};

pub const TIME_TO_LIVE : u128 = 600_000; // 10 minutes
pub const MAX_RELAY_COUNT : u64 = 3000;




pub struct RelayMap {
    pub relay_map : std::collections::HashMap<PeerId, PeerData>,
}

impl RelayMap {
    pub fn new() -> Self {
        RelayMap {
            relay_map: std::collections::HashMap::new(),
        }
    }

    pub fn bind_peer(&mut self, peer_id: PeerId, peer_addr: SocketAddrV4)-> Result<(), String> {
        let peer_data = PeerData {
            peer_id: peer_id.to_string(),
            peer_addr,
            waiting_punch: false,
            discovery_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis(),
        };
        if(self.relay_map.len() >= MAX_RELAY_COUNT as usize) {
            if(self.relay_map.contains_key(&peer_id)) {
                return Err("Peer already exists".to_string());
            } else {
                return Err("Relay map is full".to_string());
            }
        }
        
        self.relay_map.insert(peer_id, peer_data);
        Ok(())

    }


    pub fn garbage_collect(&mut self) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.relay_map.retain(|_, peer_data| {
            current_time - peer_data.discovery_time < TIME_TO_LIVE
        });
    }
}