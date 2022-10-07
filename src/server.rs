use std::error::Error;
use std::net::{TcpListener, TcpStream, IpAddr};
use std::io::{self, Read};

use crate::config::Config;
use crate::http::responder::Responder;
use crate::http::response::HttpResponse;
use crate::http::status::Status;

use log::{debug, info, error};

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
        
        for req in listener.incoming() {
            let mut stream = req?;

            let ip = self.get_remote_ip(&mut stream)?;
            debug!("received a TCP stream from {}", ip);

            // TODO: use this
            let _body = self.get_body(&mut stream)?;

            let responder = Responder::new(HttpResponse {
                status: Status::Ok,
                content_type: "text/plain",
                body: String::from("Today will be a good day!"),
            });

            match responder.respond(&mut stream) {
                Ok(_) => info!("successfully responded to {}", ip),
                Err(e) => error!("error while responding to {}: {}", ip, e)
            }
        }

        Ok(())
    }

    /// Creates a TCP listener from the config.
    fn make_listener(&self) -> io::Result<TcpListener> {
        TcpListener::bind((self.config.address, self.config.port))
    }

    /// Retrieves a TCP stream's body as a `String`.
    fn get_body(&self, stream: &mut TcpStream) -> io::Result<String>{
        let mut buffer = [0; 2048];

        match stream.read(&mut buffer) {
            Err(e) => Err(e),
            Ok(_) => Ok(String::from_utf8_lossy(&buffer).to_string())
        }
    }

    fn get_remote_ip(&self, stream: &mut TcpStream) -> io::Result<IpAddr> {
        let address = stream.peer_addr()?;

        Ok(address.ip())
    }
}