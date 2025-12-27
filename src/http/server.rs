use std::error::Error;
use std::io;
use std::net::TcpListener;

use crate::config::Config;
use crate::http::responder::Responder;
use crate::http::response::HttpResponse;
use crate::http::status::Status;

use log::{error, info};

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

    /// Creates a `TcpListener`, listens on the configured IP address and port
    /// and handles TCP requests.
    pub fn serve(&self) -> Result<(), Box<dyn Error>> {
        info!("creating a TCP listener");

        let listener = self.make_listener()?;
        let mut responder = Responder::new();

        for req in listener.incoming() {
            let mut stream = req?;

            responder.response = HttpResponse {
                status: Status::Ok,
                content_type: "text/plain",
                body: String::from("Today will be a good day!"),
            };

            if let Err(e) = responder.respond(&mut stream) {
                error!("error: {}", e);
            }
        }

        Ok(())
    }

    /// Creates a TCP listener from the config.
    fn make_listener(&self) -> io::Result<TcpListener> {
        TcpListener::bind((self.config.address, self.config.port))
    }
}
