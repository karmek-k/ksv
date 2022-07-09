mod config;
mod server;

use server::HttpServer;

fn main() {
    println!("ksv web server");

    HttpServer::new(Default::default())
        .serve()
        .unwrap();
}
