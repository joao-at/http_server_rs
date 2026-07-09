use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::http::response::{Response, Content, ResponseStatus};

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