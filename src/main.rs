use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use relay_tools::server::Server;

mod relay_tools;

/// Função auxiliar que envia uma requisição HTTP via TcpStream e retorna a resposta como String.
/// Essa função simula uma máquina externa, sem compartilhar o estado interno do servidor.
fn send_http_request(request: &str) -> String {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Falha ao conectar ao servidor");
    stream
        .write_all(request.as_bytes())
        .expect("Falha ao enviar a requisição");
    stream
        .shutdown(std::net::Shutdown::Write)
        .expect("Falha ao fechar o stream de escrita");
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader
        .read_to_string(&mut response)
        .expect("Falha ao ler a resposta");
    response
}

#[tokio::main]
async fn main() {
    Server::new().start_http_server().await;
}
