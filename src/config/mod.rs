use std::net::SocketAddr;

pub struct Config {
    pub addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
        }
    }
}
