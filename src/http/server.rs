use std::error::Error;
use std::io;
use std::net::TcpListener;

use crate::config::Config;
use crate::http::responder::Responder;
use crate::http::response::HttpResponse;
use crate::http::status::Status;

use log::{error, info};

/// This is the base HTTP server that powers ksv.
pub struct HttpServer {
    config: Config,
}

impl HttpServer {
    /// Creates a new `HttpServer` with given config.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Creates a `TcpListener`, listens on the host and port from the config
    /// and handles TCP requests.
    ///
    /// This method returns only if there is an error, otherwise
    /// it listens infinitely.
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

    // Retrieves a TCP stream's body as a `String`.
    // fn get_body(&self, stream: &mut TcpStream) -> io::Result<String> {
    //     let mut buffer = [0; 2048];

    //     match stream.read(&mut buffer) {
    //         Err(e) => Err(e),
    //         Ok(_) => Ok(String::from_utf8_lossy(&buffer).to_string()),
    //     }
    // }
}
