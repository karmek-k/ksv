mod config;
mod server;

use server::HttpServer;
use config::Config;

fn main() {
    println!("ksv web server");

    if let Err(e) = HttpServer::new(Config::default()).serve() {
        panic!("{}", e.to_string());
    }
}
