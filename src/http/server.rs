use std::net::{SocketAddr, TcpListener};
use crate::http::request::Request;
use crate::http::response::Response;
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
                listener: TcpListener::bind(format!("{ip}:{port}"))?,
            }
        )
    }

    pub fn run(&self) -> std::io::Result<()> {

        // Accept connections
        for stream in self.listener.incoming() {
            Self::handle_client( HttpStream::new(stream?) )?;
        }

        Ok(())
    }

    fn handle_client(mut stream: HttpStream) -> std::io::Result<()> {

        //println!("New connection from {}", stream.peer_addr()?);

        let request = stream.read_request();
        //println!("{:?}\n", request);
        let response = request.create_response();
        //println!("{:?}\n", response);
        stream.write_response(&response)?;

        //println!("Closing connection\n---\n");

        Self::log(&stream, &request, &response);

        Ok(())
    }

    fn log(stream: &HttpStream, request: &Request, response: &Response) {
        println!(
            " {:^20} | {:>15} | {:<7} \"{}\"",
            response.status,
            stream.peer_addr().unwrap_or(SocketAddr::from(([0, 0, 0, 0], 0))).ip(),
            request.method,
            request.uri,
        );
    }
}






