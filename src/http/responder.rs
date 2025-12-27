use std::io::Error;
use std::io::Write;
use std::net::TcpStream;

use crate::http::response::HttpResponse;

// TODO: remove this? why not just make a single responder function
/// Responsible for writing a `HttpResponse` to a `TcpStream`.
pub struct Responder<'a> {
    /// Response that should be written
    pub response: HttpResponse<'a>,
}

impl<'a> Responder<'a> {
    /// Creates a new `Responder` with the default response.
    pub fn new() -> Self {
        Responder {
            response: Default::default(),
        }
    }

    /// Writes a `HttpResponse` to a `TcpStream`.
    pub fn respond(&self, stream: &mut TcpStream) -> Result<(), Error> {
        stream.write_all(self.response.to_string().as_bytes())?;
        stream.flush()?;

        Ok(())
    }
}
