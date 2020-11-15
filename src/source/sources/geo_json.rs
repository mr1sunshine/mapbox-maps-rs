use super::SourceControl;
use crate::style_spec;
use crate::{network::NetworkManager, source::OverscaledTileId};
use async_trait::async_trait;
use eyre::Result;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct GeoJSON {
    nm: Arc<NetworkManager>,
    name: String,
    options: style_spec::GeoJSON,
}

impl GeoJSON {
    pub fn new(nm: Arc<NetworkManager>, name: &str, options: &style_spec::GeoJSON) -> Self {
        Self {
            nm,
            name: name.to_owned(),
            options: options.clone(),
        }
    }
}

#[async_trait]
impl SourceControl for GeoJSON {
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
