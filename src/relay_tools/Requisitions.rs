use std::str::FromStr;
use super::{peer_data::{PeerId}, RelayFlags::RelayFlag};

#[derive(Debug)]
pub struct Req {
    pub flag: RelayFlag,
    pub content: Vec<String>,
}

impl FromStr for Req {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() < 1 {
            return Err("requisição malformada".to_string());
        }

        let flag = parts[0].parse::<RelayFlag>()
            .map_err(|_| "flag inválida".to_string())?;

        let content = parts[1..].iter().map(|s| s.to_string()).collect();

        Ok(Req { flag, content })
    }
}

impl Req {
    pub fn to_string(&self) -> String {
        let mut result = self.flag.to_string();
        for c in &self.content {
            result.push('|');
            result.push_str(c);
        }
        result.push('\n');
        result
    }

    pub fn get_id(&self) -> Option<PeerId> {
        self.content.get(0)?.parse::<PeerId>().ok()
    }

    pub fn get_target_addr(&self) -> Option<(String, u16)> {
        if self.content.len() >= 3 {
            let ip = self.content[1].clone();
            let port = self.content[2].parse::<u16>().ok()?;
            Some((ip, port))
        } else {
            None
        }
    }
}