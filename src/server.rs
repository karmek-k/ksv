use std::io::{self, Read, Write};
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
            let mut stream = req?; 

            // TODO: bring logging back
            // self.print_body(&mut stream); 
           
            // TODO: reply with something meaningful
            stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n")?;
            stream.flush()?;
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