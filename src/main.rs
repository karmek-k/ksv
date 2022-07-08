mod config;
mod server;

use server::HttpServer;
use config::Config;

fn main() {
    println!("ksv web server");

    let server = HttpServer::new(Config::default()); 

    if let Err(e) = server.serve() {
        panic!("{}", e);
    }
}
