mod config;
mod server;
mod http;

use log::{info, error};
use simple_logger::SimpleLogger;

use server::HttpServer;

fn main() {
    SimpleLogger::new().init().expect("logger initialization failed");

    info!("web server is starting");

    let server = HttpServer::new(Default::default());

    if let Err(e) = server.serve() {
        error!("an error occured while serving: {}", e);
    }
}
