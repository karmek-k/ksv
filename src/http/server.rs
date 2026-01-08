use std::io::Write;
use std::io::{self, Read};
use std::net::{Shutdown, TcpListener, TcpStream};

use crate::config::Config;
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::http::status::Status;

use log::{debug, error, info};

/// Base HTTP server.
pub struct HttpServer {
    /// Server configuration
    config: Config,
}

impl HttpServer {
    /// Creates a new `HttpServer` using `config`.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Listens on the configured IP address and port and handles TCP requests.
    pub fn serve(&self) -> io::Result<()> {
        let listener = self.make_listener()?;

        info!("listening on {}:{}", self.config.address, self.config.port);
        for request in listener.incoming() {
            match self.handle_request(request?) {
                Ok(response) => info!("served: {}", response.status),
                Err(e) => error!("error: {}", e),
            }
        }

        Ok(())
    }

    /// Creates a TCP listener from the config.
    fn make_listener(&self) -> io::Result<TcpListener> {
        info!("creating a TCP listener");
        TcpListener::bind((self.config.address, self.config.port))
    }

    /// Handles an incoming request.
    fn handle_request(&self, mut stream: TcpStream) -> io::Result<HttpResponse<'_>> {
        debug!("handling request");

        // TODO: improve this - handle the full request
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        let request = HttpRequest::from(&buffer);

        match request {
            Ok(request) => {
                debug!("{} {}", request.method, request.path);
                let response = write_response(request, stream)?;
                Ok(response)
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }
}

fn write_response<'a>(request: HttpRequest, mut stream: TcpStream) -> io::Result<HttpResponse<'a>> {
    let response = HttpResponse {
        status: Status::Ok,
        content_type: "text/plain",
        body: format!("{} {}", request.method, request.path),
    };

    stream.write_all(response.to_string().as_bytes())?;
    stream.flush()?;
    stream.shutdown(Shutdown::Both)?;

    Ok(response)
}
