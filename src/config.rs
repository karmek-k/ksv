use std::net::{IpAddr, Ipv4Addr};

/// Represents server configuration.
#[derive(Debug)]
pub struct Config {
    /// IP address to listen on
    pub address: IpAddr,
    /// TCP port number
    pub port: u16,
}

impl Default for Config {
    /// Provides a default Config.
    ///
    /// # Default values
    /// * `address`: 127.0.0.1
    /// * `port`: 8000
    ///
    /// # Example
    /// ```
    /// let config = Config::default();
    /// assert_eq!(config.port, 8000);
    /// ```
    fn default() -> Self {
        Self {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();

        assert_eq!(config.port, 8000);
        assert_eq!(config.address, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
}
