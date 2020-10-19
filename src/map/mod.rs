mod config;

pub use config::Config;
use crate::network::NetworkManager;
use eyre::Result;

pub struct Map {
    config: Config,
    nm: NetworkManager
}

impl Map {
    pub fn new(config: Config) -> Result<Self> {
        let nm = NetworkManager::new(config.token())?;
        Ok(Self {
            config,
            nm
        })
    }
}