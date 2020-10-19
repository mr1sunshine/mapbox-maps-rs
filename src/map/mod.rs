mod config;

pub use config::Config;
use crate::network::NetworkManager;
use eyre::Result;
use crate::style_spec::Style;

pub struct Map {
    _config: Config,
    nm: NetworkManager,
    style: Option<Style>
}

impl Map {
    pub fn new(config: Config) -> Result<Self> {
        let nm = NetworkManager::new(config.token())?;
        Ok(Self {
            _config: config,
            nm,
            style: None
        })
    }

    pub async fn load_style(&mut self, uri: &str) -> Result<()> {
        let style_str = self.nm.load_style(uri).await?;
        let style = serde_json::from_str::<Style>(&style_str)?;
        self.style = Some(style);

        println!("{:#?}", self.style);
        Ok(())
    }
}