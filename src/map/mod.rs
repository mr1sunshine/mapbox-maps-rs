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

    pub async fn load_style(&self, uri: &str) -> Result<()> {
        let style_str = self.nm.load_style(uri).await?;
        println!("{}", style_str);
        Ok(())
    }
}