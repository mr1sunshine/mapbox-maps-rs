mod config;

use crate::geo::Transform;
use crate::network::NetworkManager;
use crate::render::Painter;
use crate::style::Style;
pub use config::Config;
use eyre::Result;
use std::sync::Arc;

pub struct Map {
    nm: Arc<NetworkManager>,
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
            config.max_pitch(),
            config.render_world_copies(),
        );
        let mut map = Self {
            nm: Arc::new(nm),
            style: None,
            painter,
            transform,
        };

        // let window_size = config.window().inner_size();
        map.resize(2560.0, 1338.0);
        // map.resize(window_size.width as f32, window_size.height as f32);

        Ok(map)
    }

    pub async fn load_style(&mut self, uri: &str) -> Result<()> {
        let style = Style::new(uri, self.nm.clone()).await?;
        self.style = Some(style);
        Ok(())
    }

    pub async fn render(&mut self) -> Result<()> {
        let style = match &mut self.style {
            Some(style) => style,
            None => return Ok(()),
        };

        style.update_sources(&self.transform).await?;

        self.painter.render()?;

        println!("Map rendered");
        Ok(())
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.transform.resize(width, height);
    }
}
