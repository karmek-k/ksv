use std::io::Read;

pub struct HttpRequest {
    pub method: String,
    pub location: String,
}

pub fn parse(stream: &mut dyn Read) -> HttpRequest {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();

    HttpRequest {
        method: String::new(),
        location: String::new(),
    }
}
