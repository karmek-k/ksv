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

            if let Ok(body) = self.get_body(&mut stream) {
                println!("{}", body);
            }

            // TODO: reply with something meaningful
            stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n")?;
            stream.flush()?;
        }

        Ok(())
    }

    fn make_listener(&self) -> io::Result<TcpListener> {
        TcpListener::bind((self.config.address, self.config.port))
    }

    fn get_body(&self, stream: &mut TcpStream) -> io::Result<String>{
        let mut buffer = [0; 2048];

        match stream.read(&mut buffer) {
            Err(e) => Err(e),
            Ok(_) => Ok(String::from_utf8_lossy(&buffer).to_string())
        }
    }
}