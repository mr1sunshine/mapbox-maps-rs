mod config;

use crate::network::NetworkManager;
use crate::style::Style;
pub use config::Config;
use eyre::Result;
use std::rc::Rc;

pub struct Map {
    _config: Config,
    nm: Rc<NetworkManager>,
    style: Option<Style>,
}

impl Map {
    pub fn new(config: Config) -> Result<Self> {
        let nm = NetworkManager::new(config.token())?;
        Ok(Self {
            _config: config,
            nm: Rc::new(nm),
            style: None,
        })
    }

    pub async fn load_style(&mut self, uri: &str) -> Result<()> {
        let style = Style::new(uri, self.nm.clone()).await?;
        self.style = Some(style);
        Ok(())
    }
}
