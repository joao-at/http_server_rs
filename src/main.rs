use std::fs::File;
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
    let read_file_result = read_file_to_bytes("resources/static/html/index.html");

    let response = format!(
        "\
        HTTP/1.1 200 OK\r\n\
        Content-Length: {}\r\n\
        Content-Type: text/html\r\n\
        \r\n\
        ",
        match &read_file_result {
            Ok(bytes) => bytes.len(),
            Err(_) => 0,
        }
    );

    println!("Response:\n{}", &response);
    stream.write_all(response.as_bytes())?;

    if let Ok(bytes) = read_file_result {
        stream.write_all(bytes.as_slice())?;
    }

    stream.flush()?;

    Ok(())
}

fn read_file_to_bytes(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // Accept connections 
    for stream in listener.incoming() {
        let _ =handle_client(stream?);
    }

    Ok(())
}