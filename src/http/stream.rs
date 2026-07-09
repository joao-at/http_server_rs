use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use crate::http::request::{Request, RequestMethod};
use crate::http::response::Response;

pub struct HttpStream {
    stream: TcpStream
}

impl HttpStream {
    pub fn new(stream: TcpStream) -> HttpStream {
        Self {
            stream
        }
    }
    pub fn read_request(&self) -> Request {
        let mut reader = BufReader::new(&self.stream);

        let mut first_line = String::new();
        if let Err(_) = reader.read_line(&mut first_line) {
            return Request::error()
        }
        // TODO: the connection currently closes after each request, but once keep-alive is implemented the rest of the request has to be read so the connection isn't filled with random stuff

        let tokens: Vec<&str> = first_line.split_whitespace().collect();

        // TODO: this is assuming the request is always correctly formated, it needs to check if a request is formated in an invalid way
        Request::new(
            match tokens[0] {
                "GET" => RequestMethod::Get,
                "HEAD" => RequestMethod::Head,
                "POST" => RequestMethod::Post,
                "PUT" => RequestMethod::Put,
                "DELETE" => RequestMethod::Delete,
                "CONNECT" => RequestMethod::Connect,
                "OPTIONS" => RequestMethod::Options,
                "TRACE" => RequestMethod::Trace,
                "PATCH" => RequestMethod::Patch,
                _ => RequestMethod::ErrorParsing
            },
            tokens[1][1..].to_string()
        )
    }

    pub fn write_response(&mut self, response: Response) -> std::io::Result<()> {

        // Send headers
        let status = response.status.get_representations();
        let mut headers = format!( "HTTP/1.1 {} {}\r\n", status.0, status.1 );


        if let Some(content) = &response.content {

            headers.push_str( format!("Content-Length: {}\r\n", content.bytes.len() ).as_str() );
            headers.push_str( format!("Content-Type: {}; charset=utf-8\r\n", content.content_type).as_str() );
        }

        headers.push_str( "\r\n");

        println!("{}", headers);

        self.stream.write_all(headers.as_bytes())?;

        // Send content
        if let Some(content) = &response.content {
            self.stream.write_all(content.bytes.as_slice())?;
        }

        self.stream.flush()?;

        Ok(())
    }
}