use super::SourceControl;
use crate::{network::NetworkManager, source::OverscaledTileId};
use crate::{source::tile::Tile, style_spec};
use async_trait::async_trait;
use eyre::Result;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct RasterDEM {
    nm: Arc<NetworkManager>,
    name: String,
    options: style_spec::RasterDEM,
}

impl RasterDEM {
    pub fn new(nm: Arc<NetworkManager>, name: &str, options: &style_spec::RasterDEM) -> Self {
        Self {
            nm,
            name: name.to_owned(),
            options: options.clone(),
        }
    }
}

#[async_trait]
impl SourceControl for RasterDEM {
    async fn load(&mut self) -> Result<()> {
        todo!()
    }

    fn has_tile(&self, _tile_id: &OverscaledTileId) -> bool {
        todo!()
    }

    async fn load_tile(&self, _tile: &mut Tile) -> Result<()> {
        todo!()
    }

    fn tile_size(&self) -> u32 {
        todo!()
    }

    fn min_zoom(&self) -> f32 {
        todo!()
    }

    fn max_zoom(&self) -> f32 {
        todo!()
    }

    fn round_zoom(&self) -> bool {
        todo!()
    }

    fn reparse_overscaled(&self) -> bool {
        todo!()
    }

    fn render_world_copies(&self) -> bool {
        todo!()
    }
}
