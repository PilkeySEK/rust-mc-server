use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};

use crate::net::client::Client;

mod net;

async fn handle_connection(mut stream: TcpStream, addr: SocketAddr) {
    let mut client = Client::new(&mut stream);
    client.process_client().await;
    println!("Client {} disconnected", addr);
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:25565";
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(_) => todo!(),
    };

    loop {
        let (stream, socket_addr) = match listener.accept().await {
            Ok(value) => value,
            Err(_) => todo!(),
        };
        tokio::spawn(handle_connection(stream, socket_addr));
    }
}
