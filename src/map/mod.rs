mod config;

use crate::network::NetworkManager;
use crate::render::Painter;
use crate::style::Style;
pub use config::Config;
use eyre::Result;
use std::rc::Rc;

pub struct Map {
    nm: Rc<NetworkManager>,
    style: Option<Style>,
    painter: Painter,
}

impl Map {
    pub async fn new(config: Config) -> Result<Self> {
        let nm = NetworkManager::new(config.token())?;
        let painter = Painter::new(config.window()).await?;
        Ok(Self {
            nm: Rc::new(nm),
            style: None,
            painter,
        })
    }

    pub async fn load_style(&mut self, uri: &str) -> Result<()> {
        let style = Style::new(uri, self.nm.clone()).await?;
        self.style = Some(style);
        Ok(())
    }

    pub async fn render(&mut self) -> Result<()> {
        println!("Map rendered");

        Ok(())
    }
}
