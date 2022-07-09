use std::io::{self, Read, Write};
use std::error::Error;
use std::net::{TcpListener, TcpStream};

use crate::config::Config;
use crate::http::{response::HttpResponse, status::Status};

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
        let listener = self.make_listener()?;
        
        for req in listener.incoming() {
            let mut stream = req?; 

            if let Ok(body) = self.get_body(&mut stream) {
                println!("{}", body);
            }

            let res = HttpResponse {
                status: Status::Ok,
                content_type: "text/plain",
                body: String::from("hello world!"),
            };
            stream.write_all(res.to_string().as_bytes())?;
            stream.flush()?;
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
}