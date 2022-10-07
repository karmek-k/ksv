use std::io::Error;
use std::net::TcpStream;
use std::io::Write;

use crate::http::response::HttpResponse;

pub struct Responder<'a> {
    response: HttpResponse<'a>
}

impl<'a> Responder<'a> {
    /// Creates a new `Responder` that responds with a HTTP response
    /// to a TCP stream.
    pub fn new(response: HttpResponse<'a>) -> Self {
        Responder { response }
    }

    /// Write a `HttpResponse` to the responder's TCP stream.
    pub fn respond(&self, stream: &mut TcpStream) -> Result<(), Error> {
        stream.write_all(self.response.to_string().as_bytes())?;
        stream.flush()?;

        Ok(())
    }
}