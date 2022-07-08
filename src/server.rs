use std::io::{self, Read};
use std::error::Error;
use std::net::{TcpListener, TcpStream};

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
        
        for req in listener.incoming() {
           self.print_body(&mut req?); 
        }

        Ok(())
    }

    fn make_listener(&self) -> io::Result<TcpListener> {
        TcpListener::bind((self.config.address, self.config.port))
    }

    fn print_body(&self, req: &mut TcpStream) {
        let mut body = String::new();

        if req.read_to_string(&mut body).is_ok() {
            println!("{}", body);
        };
    }
}