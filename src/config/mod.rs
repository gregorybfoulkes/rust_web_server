use std::net::SocketAddr;

pub struct Config {
    pub addr: SocketAddr,
}

impl Default for Config {
    /// Creates a new `Config` instance with the default values.
    ///
    /// The default values are:
    ///
    /// * `addr`: `127.0.0.1:3000`
    fn default() -> Self {
        Config {
            addr: SocketAddr::from(([127, 0, 0, 1], 3001)),
        }
    }
}
