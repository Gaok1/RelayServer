use std::net::SocketAddrV4;
use std::time::{SystemTime, UNIX_EPOCH};

use super::peer_data::{PeerData, PeerId};

pub const TIME_TO_LIVE: u128 = 600_000; // 10 min
pub const MAX_RELAY_COUNT: usize = 3_000;

pub struct RelayMap {
    pub inner: std::collections::HashMap<PeerId, PeerData>,
}

impl RelayMap {
    pub fn new() -> Self {
        Self {
            inner: std::collections::HashMap::new(),
        }
    }

    pub fn bind_peer(&mut self, peer_id: PeerId, peer_addr: SocketAddrV4) -> Result<(), String> {
        if self.inner.len() >= MAX_RELAY_COUNT {
            return Err("Relay map está cheio".into());
        }
        if self.inner.contains_key(&peer_id) {
            return Err("Peer já registrado".into());
        }

        let peer_data = PeerData::new(peer_id, peer_addr);
        self.inner.insert(peer_id, peer_data);
        Ok(())
    }

    pub fn get_mut(&mut self, id: &PeerId) -> Option<&mut PeerData> {
        self.inner.get_mut(id)
    }

    pub fn get(&self, id: &PeerId) -> Option<&PeerData> {
        self.inner.get(id)
    }

    pub fn garbage_collect(&mut self) {
        let now = now_ms();
        self.inner
            .retain(|_, data| now - data.discovery_time <= TIME_TO_LIVE);
    }

    pub fn has_peer(&self, id: &PeerId) -> bool {
        self.inner.contains_key(id)
    }

    pub fn get_peerData_mut(&mut self, id: &PeerId) ->&mut PeerData {
        self.inner.get_mut(id).unwrap()
    }
    
}

pub fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}