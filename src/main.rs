mod config;
mod server;
mod http;

use server::HttpServer;

fn main() {
    println!("ksv web server");

    HttpServer::new(Default::default())
        .serve()
        .expect("error while serving HTTP");
}
