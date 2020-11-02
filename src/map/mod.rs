mod config;

use crate::geo::Transform;
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
    transform: Transform,
}

impl Map {
    pub async fn new(config: Config) -> Result<Self> {
        let nm = NetworkManager::new(config.token())?;
        let painter = Painter::new(config.window()).await?;
        let transform = Transform::new(
            config.min_zoom(),
            config.max_zoom(),
            config.min_pitch(),
            config.max_zoom(),
            config.render_world_copies(),
        );
        Ok(Self {
            nm: Rc::new(nm),
            style: None,
            painter,
            transform,
        })
    }

    pub async fn load_style(&mut self, uri: &str) -> Result<()> {
        let style = Style::new(uri, self.nm.clone()).await?;
        self.style = Some(style);
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        println!("Map rendered");

        self.painter.render()?;

        Ok(())
    }
}
