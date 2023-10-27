use std::io::Error;
use std::io::Write;
use std::net::TcpStream;

use crate::http::response::HttpResponse;

pub struct Responder<'a> {
    pub response: HttpResponse<'a>,
}

impl<'a> Responder<'a> {
    /// Creates a new `Responder` that responds with a HTTP response
    /// to a TCP stream.
    pub fn new() -> Self {
        Responder {
            response: Default::default(),
        }
    }

    /// Write a `HttpResponse` to the responder's TCP stream.
    pub fn respond(&self, stream: &mut TcpStream) -> Result<(), Error> {
        stream.write_all(self.response.to_string().as_bytes())?;
        stream.flush()?;

        Ok(())
    }
}
