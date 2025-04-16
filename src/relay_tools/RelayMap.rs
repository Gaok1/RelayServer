use std::fmt::format;
use std::net::SocketAddrV4;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use super::peer_data::{PeerData, PublicKey};

pub const TIME_TO_LIVE: u128 = 600_000; // 10 min
pub const MAX_RELAY_COUNT: usize = 3_000;

pub struct RelayMap {
    pub inner: std::collections::HashMap<PublicKey, PeerData>,
}

impl RelayMap {
    pub fn new() -> Self {
        Self {
            inner: std::collections::HashMap::new(),
        }
    }

    pub fn bind_peer(
        &mut self,
        public_key: PublicKey,
        peer_addr: SocketAddrV4,
    ) -> Result<(), String> {
        if self.inner.len() >= MAX_RELAY_COUNT {
            return Err("Relay map está cheio".into());
        }

        let peer_data = PeerData::new(public_key.clone(), peer_addr);
        println!(
            "Binding peer : {}  to {}",
            peer_data.public_key, peer_data.peer_addr
        );
        self.inner.insert(public_key, peer_data);

        Ok(())
    }

    pub fn get_mut(&mut self, id: &PublicKey) -> Option<&mut PeerData> {
        self.inner.get_mut(id)
    }

    pub fn get(&self, id: &PublicKey) -> Option<&PeerData> {
        self.inner.get(id)
    }

    pub fn garbage_collect(&mut self) {
        let now = now_ms();
        self.inner
            .retain(|_, data| now - data.discovery_time <= TIME_TO_LIVE);
    }

    pub fn has_peer(&self, p_key: &PublicKey) -> bool {
        self.inner.contains_key(p_key)
    }

    pub fn get_peerData_mut(&mut self, p_key: &PublicKey) -> &mut PeerData {
        self.inner.get_mut(p_key).unwrap()
    }

    pub fn reset_peer_time(&mut self, p_key: &PublicKey) {
        if let Some(peer_data) = self.inner.get_mut(p_key) {
            peer_data.discovery_time = now_ms();
        }
    }
}

pub fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
