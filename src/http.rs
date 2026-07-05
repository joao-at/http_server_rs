use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: RequestMethod,
    pub uri: String,
    pub resource_type: String,
}

#[derive(Debug)]
pub enum RequestMethod {
    ErrorParsing,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

#[derive(Debug)]
pub struct Response {
    pub status: ResponseStatus,
    pub content: Option<Content>,
}

#[derive(Debug)]
pub enum ResponseStatus {
    Ok,
    //Created,
    //Accepted,
    BadRequest,
    //Unauthorized,
    //Forbidden,
    NotFound,
    //ImATeapot,
    NotImplemented,
}

#[derive(Debug)]
pub struct Content {
    pub content_type: String,
    pub bytes: Vec<u8>,
}

impl ResponseStatus {
    fn get_representations(&self) -> (i32, &str){
        match self {
            ResponseStatus::Ok => (200, "OK"),
            //ResponseStatus::Created => (201, "Created"),
            //ResponseStatus::Accepted => (202, "Accepted"),
            ResponseStatus::BadRequest => (400, "Bad Request"),
            //ResponseStatus::Unauthorized => (401, "Unauthorized"),
            //ResponseStatus::Forbidden => (403, "Forbidden"),
            ResponseStatus::NotFound => (404, "Not Found"),
            //ResponseStatus::ImATeapot => (418, "I'm a Teapot"),
            ResponseStatus::NotImplemented => (501, "Not Implemented"),
        }
    }
}

impl Request {

    pub fn new(method: RequestMethod, mut uri: String) -> Request {

        if uri.contains("..") { // Prevents .. URI attacks
            return Request::error()
        }

        if uri.is_empty() { uri = "index".to_string() }

        let r_type;
        if uri.ends_with(".css") {
            uri = format!("resources/static/css/{}", uri);
            r_type = "text/css".to_string();
        } else {
            uri = format!("resources/static/html/{}.html", uri);
            r_type = "text/html".to_string();
        }

        Self{
            method,
            uri,
            resource_type: r_type,
        }
    }

    pub fn error() -> Request {
        Self{
            method: RequestMethod::ErrorParsing,
            uri: "".to_string(),
            resource_type: "".to_string(),
        }
    }

    pub fn create_response(&self) -> Response {
        match self.method {
            RequestMethod::ErrorParsing => Response{
                status: ResponseStatus::BadRequest,
                content: None,
            },
            RequestMethod::Get => {
                let content_result = Self::read_file_to_bytes(self.uri.as_str());

                match content_result {
                    Ok(content) => {
                        let mut headers: HashMap<String, String> = HashMap::new();

                        headers.insert("Content-Length".to_string(), content.len().to_string());
                        headers.insert("Content-Type:".to_string(), "text/html".to_string());

                        Response{
                            status: ResponseStatus::Ok,
                            content: Some( Content{
                                content_type: self.resource_type.clone(),
                                bytes: content,
                            }),
                        }
                    },
                    Err(_) => Response{
                        status: ResponseStatus::NotFound,
                        content: None,
                    }
                }
            },
            _ => Response{
                status: ResponseStatus::NotImplemented,
                content: None,
            }
        }
    }

    fn read_file_to_bytes(path: &str) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}

pub struct HttpStream {
    stream: TcpStream
}

impl HttpStream {
    pub fn new(stream: TcpStream) -> HttpStream {
        Self {
            stream
        }
    }
    pub fn get_request(&self) -> Request {
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

    pub fn send_response(&mut self, response: Response) -> std::io::Result<()> {

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
