mod config;
mod server;

use server::HttpServer;
use config::Config;

fn main() {
    println!("ksv web server");

    HttpServer::new(Config::default())
        .serve()
        .unwrap();
}
