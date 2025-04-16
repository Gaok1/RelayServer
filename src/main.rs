use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use relay_tools::server::Server;

mod relay_tools;



#[tokio::main]
async fn main() {

    Server::new().start_http_server().await;
    

}
