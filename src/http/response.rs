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
    pub fn get_representations(&self) -> (i32, &str){
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
