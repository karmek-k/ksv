use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub root: PathBuf,
}

impl Config {
    pub fn default() -> Self {
        Self { 
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 80_u16,
            root: PathBuf::from("~/ksv-root"),
        }
    }
}