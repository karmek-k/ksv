use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

pub struct Config {
    address: IpAddr,
    port: u16,
    root: PathBuf,
}

pub fn default() -> Config {
    Config { 
        address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port: 80_u16,
        root: PathBuf::from("~/ksv-root"),
    }
}