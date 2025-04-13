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
    let server = Server::new();

    // Thread de coleta de lixo para remover peers expirados
    let gc_server = Arc::clone(&server);
    thread::spawn(move || {
        Server::start_garbage_collector(gc_server);
    });
    thread::spawn(move || {
        Server::listen(server);
    });
    
    let mut tcp = TcpStream::connect("127.0.0.1:8080").unwrap();
    let message = format!("{STORE}|{}|\n", 2);

    tcp.write_all(message.as_bytes()).unwrap();
    tcp.shutdown(std::net::Shutdown::Write).unwrap();
    let mut reader = BufReader::new(tcp);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    
    loop {
        if buffer != "" {
            println!("Recebido: {}", buffer);
            buffer.clear();
        }
        reader.read_line(&mut buffer).unwrap();
    }
    //simular requisição store e discover
}
