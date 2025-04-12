use std::str::FromStr;

use super::RelayFlags::RelayFlag;
use super::peer_data::PeerId;

#[derive(Debug)]
pub struct Req {
    pub flag: RelayFlag,
    pub content: Option<Vec<String>>,
    peer_id: PeerId,
}
impl FromStr for Req {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove espaços em branco no início e fim
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err("Entrada vazia".to_string());
        }

        // Divide a string pelo delimitador '|'
        let parts: Vec<&str> = trimmed.split('|').collect();
        // Esperamos pelo menos três partes: flag, peer_id e conteúdo
        if parts.len() < 3 {
            return Err("Formato inválido: esperado 'flag|peer_id|conteúdo'".to_string());
        }

        // Converte a flag (primeira parte)
        let flag = parts[0]
            .parse::<RelayFlag>()
            .map_err(|_| "Flag inválida".to_string())?;

        // Converte o peer_id (segunda parte)
        let peer_id = parts[1]
            .parse::<PeerId>()
            .map_err(|_| "PeerId inválido".to_string())?;

        // O conteúdo é formado pelas partes restantes
        let content: Vec<String> =  parts[2..].iter().map(|s| s.to_string()).collect();
        
        let content = match content.is_empty() {
            true => None,
            false => Some(content),
        };

        Ok(Req {
            flag,
            peer_id,
            content,
        })
    }
}

impl Req {
    pub fn get_id(&self) -> PeerId {
        self.peer_id
    }
}
