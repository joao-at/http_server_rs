mod http;

use std::net::{TcpListener, TcpStream};
use crate::http::HttpStream;

fn handle_client(stream: TcpStream) -> std::io::Result<()> {

    println!("New connection from {}", stream.peer_addr()?);
    let mut handler = HttpStream::new(stream);

    let request = handler.get_request();
    println!("{:?}\n", request);
    let response = request.create_response();
    println!("{:?}\n", response);
    handler.send_response(response)?;

    println!("Closing connection\n---\n");

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // Accept connections 
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}