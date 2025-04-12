use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::{SocketAddr, SocketAddrV4, TcpListener, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use stunclient::StunClient;

use crate::relay_tools::RelayFlags::*;

use super::RelayMap::RelayMap;
use super::Requisitions::Req;
use super::peer_data::PeerId;

const LISTEN_ADDR: &str = "0.0.0.0:8080";

const TIME_OUT_WAITING_PUNCH: u64 = 30; // Timeout para espera do outro peer para punch em segundos

pub struct Server {
    relay_map: RwLock<RelayMap>,
}

impl Server {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            relay_map: RwLock::new(RelayMap::new()),
        })
    }

    /// Thread de GC (remove peers expirados periodicamente)
    pub fn start_garbage_collector(server: Arc<Self>) {
        loop {
            thread::sleep(Duration::from_secs(60));
            server.relay_map.write().unwrap().garbage_collect();
        }
    }

    

    /// Loop principal do servidor – aceita conexões e despacha para threads.
    pub fn listen(server: Arc<Self>) {
       // println!("Escutando no ip público: {}", Self::get_stun_address());
        let listener = TcpListener::bind(LISTEN_ADDR).expect("Falha ao abrir socket");
        println!("Servidor escutando em {}", LISTEN_ADDR);
        println!("Endereço do STUN: {}", get_stun_address());
        println!("Aguardando conexões...");

        for stream in listener.incoming() {
            let Ok(stream) = stream else {
                eprintln!("Falha ao aceitar conexão");
                continue;
            };
            println!("Conexão recebida de {}", stream.peer_addr().unwrap());
            // Clona o Arc do servidor para passar para a thread
            let server = Arc::clone(&server);
            thread::spawn(move || {
                if let Err(e) = Self::handle_client(stream, server) {
                    eprintln!("Erro ao tratar cliente: {e}");
                }
            });
        }
        println!("Servidor encerrado.");
    }

    fn handle_client(mut stream: std::net::TcpStream, server: Arc<Self>) -> std::io::Result<()> {
        let peer_socket = stream.peer_addr()?;
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
        
        let SocketAddr::V4(addr) = peer_socket else {
            return Err(Error::new(ErrorKind::InvalidInput, "Endereço inválido"));
        };
        let answer = Self::handle_request(buffer, addr, server.clone());
        stream.write_all(answer.as_bytes())?;
        stream.flush()?;
        println!("Resposta enviada para {}: {}", addr, answer.trim_end());
        stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }

    fn handle_request(req: String, addr: SocketAddrV4, server: Arc<Self>) -> String {
        // Tenta realizar o parse da requisição
        let parsed = match req.parse::<Req>() {
            Ok(r) => r,
            Err(e) => return format!("erro|{}\n", e),
        };

        // Identifica o id do remetente da requisição
        let sender_id = parsed.get_id();

        match parsed.flag {
            STORE => {
                Self::store(server, sender_id, addr)
            }
            DISCOVER => {
                Self::discover(server, parsed)
            }
            WAITING_PUNCH => {
                Self::waiting_punch(server, parsed, sender_id)
            },
            PUNCH_WAITING_TIMEOUT => {
                let Ok(mut map) = server.relay_map.write() else {
                    return format!("{}|{}\n", INTERNAL_ERROR, "Falha ao acessar o mapa de relay");
                };
                if(!map.has_peer(&sender_id)) {
                    return format!("{}|{}\n", INTERNAL_ERROR, "Peer não registrado");
                }
                let peer_data = map.get_mut(&sender_id).unwrap();
                peer_data.waiting_punch = false;
                println!("[PUNCH] {} timeout", sender_id);
                format!("{}|{}\n", PUNCH_WAITING_TIMEOUT, sender_id)
            }
            _ => format!("{}|{}\n", INTERNAL_ERROR, "Comando inválido"),
        }
    }


    fn store(server : Arc<Self>, sender_id: PeerId, addr: SocketAddrV4) -> String {
        // Operação STORE: registra o remetente com seu endereço
        match server.relay_map.write().unwrap().bind_peer(sender_id, addr) {
            Ok(_) => {
                println!("[STORE] Peer {} registrado em {}", sender_id, addr);
                format!("{}|{}|{}\n", STORED, addr, sender_id)
            }
            Err(e) => {
                eprintln!("[STORE] Erro: {}", e);
                format!("{}|{}\n", NOT_STORED, e)
            }
        }
    }

    fn discover(server : Arc<Self>, parsed: Req) -> String {
        // Operação DISCOVER: espera que o target_id seja informado como segundo argumento
        if parsed.content.is_none() {
            return "erro|DISCOVER requer target id\n".to_string();
        }
        let content = parsed.content.unwrap();
        
        let target_id = match content[0].parse::<PeerId>() {
            Ok(id) => id,
            Err(_) => return "erro|Target id inválido\n".to_string(),
        };

        let map_guard = server.relay_map.read().unwrap();
        if let Some(peer_data) = map_guard.get(&target_id) {
            println!(
                "[DISCOVER] {} => PRESENT {}",
                target_id, peer_data.peer_addr
            );
            format!(
                "{}|{}|{}|{}\n",
                PRESENT,
                target_id,
                peer_data.peer_addr.ip(),
                peer_data.peer_addr.port()
            )
        } else {
            println!("[DISCOVER] {} => NOT PRESENT", target_id);
            format!("{}\n", NOT_PRESENT)
        }
    }    

    fn waiting_punch(server : Arc<Self>, parsed: Req, sender_id: PeerId) -> String {
        // Operação WAITING_PUNCH: verifica se há target_id e se o peer requisitante já está registrado
        if parsed.content.is_none(){
            return format!("{}|{}\n", INVALID_REQUEST_FORMAT, "Esperado WAITING_PUNCH|ID|TARGET_ID");
        }
        let content = parsed.content.unwrap();
        let target_id: PeerId = match content[0].parse::<PeerId>() {
            Ok(id) => id,
            Err(_) => return format!("{}|{}\n", INVALID_REQUEST_FORMAT, "Target id inválido"),
        } ;

        let mut map = server.relay_map.write().unwrap();
        // Verifica se o remetente está registrado
        match map.get_mut(&sender_id) {
            Some(data) => data.waiting_punch = true,
            None => return format!("{}|{}\n", INTERNAL_ERROR, "Peer não registrado"),
        };
        drop(map);
        let now = std::time::SystemTime::now();
        while now.elapsed().unwrap().as_secs() < TIME_OUT_WAITING_PUNCH {
            thread::sleep(Duration::from_secs(1));
            let map_reader = server.relay_map.read().unwrap();
            if let Some(peer_data) = map_reader.get(&target_id) {
                if peer_data.waiting_punch {
                    return format!("{}|{}\n", PUNCH, target_id);
                }
            }
        }
        format!("{}\n", TIME_OUT_ERRO)
    }
    
}





/// Descobre o endereço público via STUN
fn get_stun_address() -> String {
    let Ok(socket) = UdpSocket::bind("0.0.0.0:8080") else {
        return "Erro ao criar socket UDP".into();
    };
    let stun_server = "stun.12voip.com:3478"
        .to_socket_addrs()
        .expect("Falha ao resolver endereço STUN")
        .next()
        .expect("Nenhum endereço encontrado");

    let client = StunClient::new(stun_server);
    match client.query_external_address(&socket) {
        Ok(public_addr) => format!("{}", public_addr),
        Err(e) => format!("Erro STUN: {}", e),
    }
}