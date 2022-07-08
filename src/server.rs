use std::io;
use std::error::Error;
use std::net::TcpListener;

use crate::config::Config;

pub struct HttpServer {
    config: Config,
}

impl HttpServer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn serve(&self) -> Result<(), Box<dyn Error>> {
        let listener = self.make_listener()?;
        
        for req in listener.accept() {
            // TODO: finish
        }

        Ok(())
    }

    fn make_listener(&self) -> io::Result<TcpListener> {
        TcpListener::bind((self.config.address, self.config.port))
    }
}