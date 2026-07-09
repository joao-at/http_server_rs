use std::net::{TcpListener, TcpStream};
use crate::http::stream::HttpStream;


pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new() -> std::io::Result<Server> {
        Self::with_ip_and_port("127.0.0.1", 8080)
    }

    pub fn with_ip_and_port(ip: &str, port: u16) -> std::io::Result<Server> {
        Ok(
            Self {
                listener: TcpListener::bind(format!("{ip}:{port}"))?
            }
        )
    }

    pub fn run(&self) -> std::io::Result<()> {

        // Accept connections
        for stream in self.listener.incoming() {
            Self::handle_client(stream?)?;
        }

        Ok(())
    }

    fn handle_client(stream: TcpStream) -> std::io::Result<()> {

        println!("New connection from {}", stream.peer_addr()?);
        let mut handler = HttpStream::new(stream);

        let request = handler.read_request();
        println!("{:?}\n", request);
        let response = request.create_response();
        println!("{:?}\n", response);
        handler.write_response(response)?;

        println!("Closing connection\n---\n");

        Ok(())
    }
}






