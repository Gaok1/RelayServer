use std::{net::SocketAddrV4, sync::{Arc, RwLock}, time::Duration};
use axum::{
    Json, Router, extract::State, routing::post,
    http::Method,
};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, task};
use tower_http::cors::{Any, CorsLayer};

use super::RelayMap::RelayMap;
use super::peer_data::PeerId;

#[derive(Clone)]
pub struct Server {
    pub relay_map: Arc<RwLock<RelayMap>>,
}

#[derive(Deserialize)]
struct StoreRequest {
    sender_id: PeerId,
    p2p_addr: String, // <-- nome mais claro agora
}


#[derive(Deserialize)]
struct DiscoverRequest {
    target_id: PeerId,
}

#[derive(Deserialize)]
struct WaitingPunchRequest {
    sender_id: PeerId,
    target_id: PeerId,
}

#[derive(Serialize)]
struct RelayResponse {
    status: String,
    message: String,
}

impl Server {
    pub fn new() -> Self {
        Self {
            relay_map: Arc::new(RwLock::new(RelayMap::new())),
        }
    }

    pub async fn start_http_server(self) {
        let server = Arc::new(self);
        // Configura o CORS para permitir qualquer origem e os métodos GET e POST.
        use axum::http::header::CONTENT_TYPE;

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([CONTENT_TYPE]);


        let app = Router::new()
            .route("/store", post(store))
            .route("/discover", post(discover))
            .route("/waiting_punch", post(waiting_punch))
            .layer(cors)
            .with_state(server.clone());

        let gc_server = server.clone();
        task::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                gc_server.relay_map.write().unwrap().garbage_collect();
            }
        });

        println!("Servidor HTTP escutando na porta 8080");
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

async fn store(
    State(server): State<Arc<Server>>,
    Json(req): Json<StoreRequest>,
) -> Json<RelayResponse> {
    let addr: SocketAddrV4 = match req.p2p_addr.parse() {
        Ok(addr) => addr,
        Err(_) => {
            return Json(RelayResponse {
                status: "error".into(),
                message: "Endereço inválido".into(),
            });
        }
    };

    match server
        .relay_map
        .write()
        .unwrap()
        .bind_peer(req.sender_id, addr)
    {
        Ok(_) => Json(RelayResponse {
            status: "stored".into(),
            message: format!("{}", addr),
        }),
        Err(e) => Json(RelayResponse {
            status: "error".into(),
            message: e.to_string(),
        }),
    }
}


async fn discover(
    State(server): State<Arc<Server>>,
    Json(req): Json<DiscoverRequest>,
) -> Json<RelayResponse> {
    let map_guard = server.relay_map.read().unwrap();
    if let Some(peer_data) = map_guard.get(&req.target_id) {
        Json(RelayResponse {
            status: "present".into(),
            message: format!("{}", peer_data.peer_addr),
        })
    } else {
        Json(RelayResponse {
            status: "not_present".into(),
            message: "".into(),
        })
    }
}

async fn waiting_punch(
    State(server): State<Arc<Server>>,
    Json(req): Json<WaitingPunchRequest>,
) -> Json<RelayResponse> {
    // Atualiza o estado do peer remetente, marcando que ele está aguardando um hole punch com o peer alvo.
    {
        let mut map = server.relay_map.write().unwrap();
        match map.get_mut(&req.sender_id) {
            Some(data) => {
                data.waiting_punch = true;
                // Novo campo para indicar com qual peer está aguardando
                data.waiting_for = Some(req.target_id);
            },
            None => {
                return Json(RelayResponse {
                    status: "error".into(),
                    message: "Peer não registrado".into(),
                });
            }
        }
    } // Libera o lock após atualizar.

    // Checa se o peer alvo existe e se ele está aguardando um hole punch com o peer remetente.
    let map_reader = server.relay_map.read().unwrap();
    if let Some(target_peer) = map_reader.get(&req.target_id) {
        if target_peer.waiting_punch && target_peer.waiting_for == Some(req.sender_id) {
            return Json(RelayResponse {
                status: "punch".into(),
                message: req.target_id.to_string(),
            });
        }
    }

    // Se a condição não for satisfeita, retorna "not_punch"
    Json(RelayResponse {
        status: "not_punch".into(),
        message: "".into(),
    })
}
