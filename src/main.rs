use std::fmt::format;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use relay_tools::RelayFlags::{DISCOVER, STORE, WAITING_PUNCH};
use relay_tools::server::Server;

mod relay_tools;

fn main() {
    // Inicia o servidor de relay
    let server = Server::new();

    // Thread de coleta de lixo para remover peers expirados
    let gc_server = Arc::clone(&server);
    thread::spawn(move || {
        Server::start_garbage_collector(gc_server);
    });

    // Bloqueia a thread principal ouvindo conexões TCP
    thread::spawn(|| {
        Server::listen(server);
    });
    

    //simular requisição store e discover
}

