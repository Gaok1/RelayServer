use std::{
    borrow::Cow,
    io::{BufRead, BufReader, Read, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use relay_tools::{RelayFlags::*, RelayMap::RelayMap, Requisitions::Req, peer_data::PeerId};

const LISTEN_PORT: u16 = 8080;
const ADDR: &'static str = "0.0.0.0:8080";
struct Server {
    relay_map: RwLock<RelayMap>,
}

mod relay_tools;

impl Server {
    pub fn new() -> Arc<Self> {
        Arc::new(Server {
            relay_map: RwLock::new(RelayMap::new()),
        })
    }
}
// listener
impl Server {
    pub fn listen(server: Arc<Self>) {
        let listener = TcpListener::bind(ADDR).expect("falha ao abrir socket");
        loop {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else {
                    eprintln!("Falha ao aceitar conexão");
                    continue;
                };
                let server = Arc::clone(&server);

                thread::spawn(move || {
                    let addr = stream
                        .peer_addr()
                        .expect("Falha ao obter endereço do cliente");
                    let peer_id = addr.port() as u64; // Exemplo de ID de peer
                    let SocketAddr::V4(ip) = addr else {
                        return;
                    };
                    let mut reader = BufReader::new(stream.try_clone().unwrap());
                    let mut buffer = String::new();
                    reader
                        .read_line(&mut buffer)
                        .expect("Falha ao ler dados do cliente");
                    let answer = Self::handle_request(buffer, peer_id, ip, server.clone());
                    stream
                        .write_all(answer.as_bytes())
                        .expect("Falha ao enviar resposta");
                    println!("Resposta enviada para o peer {}: {}", peer_id, answer);
                });
            }
        }
    }

    fn handle_request(
        req: String,
        peer_id: u64,
        peer_addr: SocketAddrV4,
        server: Arc<Server>,
    ) -> String {
        let parsed = match req.parse::<Req>() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Erro ao parsear requisição: {}", e);
                return format!("erro|{}\n", e);
            }
        };

        match parsed.flag {
            STORE => {
                server
                    .relay_map
                    .write()
                    .unwrap()
                    .bind_peer(peer_id, peer_addr);
                println!("[STORE] Peer {} registrado com IP {}", peer_id, peer_addr);
                return format!("{}|{}|{}\n", STORED, peer_id, peer_addr);
            }

            DISCOVER => {
                let Some(target_id) = parsed.get_id() else {
                    return "DISCOVER malformado\n".to_string();
                };

                let map_guard = server.relay_map.read().unwrap();
                if let Some(peer_data) = map_guard.relay_map.get(&target_id) {
                    println!(
                        "[DISCOVER] Peer {} requisitou {} => {}",
                        peer_id, target_id, peer_data.peer_addr
                    );
                    return format!(
                        "{}|{}|{}|{}\n",
                        PRESENT,
                        target_id,
                        peer_data.peer_addr.ip(),
                        peer_data.peer_addr.port()
                    );
                } else {
                    println!(
                        "[DISCOVER] Peer {} requisitou {} => NOT PRESENT",
                        peer_id, target_id
                    );
                    return format!("{}\n", NOT_PRESENT);
                }
            }

            WAITING_PUNCH => {
                let Some(target_id) = parsed.get_id() else {
                    return "WAITING_PUNCH malformado\n".to_string();
                };

                let mut map_guard = server.relay_map.write().unwrap();
                if let Some(peer_data) = map_guard.relay_map.get_mut(&target_id) {
                    peer_data.waiting_punch = true;
                    println!(
                        "[WAITING_PUNCH] Peer {} requisitou {} => marcado como esperando punch",
                        peer_id, target_id
                    );
                    return format!("{}\n", WAITING_PUNCH);
                } else {
                    return format!("{}\n", NOT_PRESENT);
                }
            }

            _ => {
                eprintln!("Flag não reconhecida: {}", parsed.flag);
                return "flag desconhecida\n".to_string();
            }
        }
    }
}

// cordenar Hole Punch
impl Server {}

fn main() {
    let server = Server::new();
    let server_clone = Arc::clone(&server);
    thread::spawn(move || {
        Server::listen(server_clone);
    });
    println!("Testando");
}
