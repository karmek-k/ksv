use std::io;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};

use crate::config::Config;
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

        // TODO: change this to something more meaningful
        let response = HttpResponse {
            status: Status::Ok,
            content_type: "text/plain",
            body: String::from("Today will be a good day!"),
        };

        stream.write_all(response.to_string().as_bytes())?;
        stream.flush()?;
        stream.shutdown(Shutdown::Write)?;

        Ok(response)
    }
}
