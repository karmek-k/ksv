mod config;
mod http;

use log::{error, info};
use simple_logger::SimpleLogger;

use http::server::HttpServer;

fn main() {
    SimpleLogger::new()
        .init()
        .expect("logger initialization failed");

    info!("web server is starting");

    let server = HttpServer::new(Default::default());

    if let Err(e) = server.serve() {
        error!("an error occured while serving: {}", e);
    }
}
