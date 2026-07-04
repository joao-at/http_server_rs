use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {

    println!("New client connected!");
    
    // Get request
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    println!("Received {} bytes", bytes_read);
    println!("Data:\n{}", String::from_utf8_lossy(&buffer));

    // Answer request
    let response = "\
    HTTP/1.1 200 OK\r\n\
    Content-Length: 13\r\n\
    Content-Type: text/plain\r\n\
    \r\n\
    Hello, world!";

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // Accept connections 
    for stream in listener.incoming() {
        let _ =handle_client(stream?);
    }

    Ok(())
}