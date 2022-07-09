use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

/// ksv server configuration.
/// 
/// This struct implements `Default` trait, by default the server
/// should listen on `127.0.0.1` on port `8000`, and the root path
/// should be the current working directory.
#[derive(Debug)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub root: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8000_u16,
            root: PathBuf::from("."),
        }
    }
}