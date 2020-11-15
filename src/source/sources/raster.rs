use super::SourceControl;
use crate::style_spec;
use crate::{network::NetworkManager, source::OverscaledTileId};
use async_trait::async_trait;
use eyre::Result;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct Raster {
    nm: Arc<NetworkManager>,
    name: String,
    options: style_spec::Raster,
}

impl Raster {
    pub fn new(nm: Arc<NetworkManager>, name: &str, options: &style_spec::Raster) -> Self {
        Self {
            nm,
            name: name.to_owned(),
            options: options.clone(),
        }
    }
}

#[async_trait]
impl SourceControl for Raster {
    async fn load(&mut self) -> Result<()> {
        Ok(())
    }

    fn has_tile(&self, _tile_id: &OverscaledTileId) -> bool {
        false
    }

    fn tile_size(&self) -> u32 {
        0
    }

    fn min_zoom(&self) -> f32 {
        0.0
    }

    fn max_zoom(&self) -> f32 {
        0.0
    }

    fn round_zoom(&self) -> bool {
        false
    }

    fn reparse_overscaled(&self) -> bool {
        false
    }

    fn render_world_copies(&self) -> bool {
        false
    }
}
