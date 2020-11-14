mod vector;

use crate::network::NetworkManager;
use crate::source::OverscaledTileId;
use crate::style_spec;
use eyre::Result;
use std::rc::Rc;
use vector::Vector;

#[derive(Debug)]
pub(crate) enum Source {
    Vector(Vector),
    Raster,
    RasterDEM,
    GeoJSON,
    Video,
    Image,
}

impl Source {
    pub fn new(nm: Rc<NetworkManager>, name: &str, source: &style_spec::Source) -> Source {
        match source {
            style_spec::Source::Vector(data) => Source::Vector(vector::Vector::new(nm, name, data)),
            style_spec::Source::Raster(_) => Source::Raster,
            style_spec::Source::RasterDEM(_) => Source::RasterDEM,
            style_spec::Source::GeoJSON(_) => Source::GeoJSON,
            style_spec::Source::Video(_) => Source::Video,
            style_spec::Source::Image(_) => Source::Image,
        }
    }

    pub async fn load(&mut self) -> Result<()> {
        if let Source::Vector(v) = self {
            v.load().await?;
        }

        Ok(())
    }

    pub fn has_tile(&self, tile_id: &OverscaledTileId) -> bool {
        if let Source::Vector(v) = self {
            return v.has_tile(tile_id);
        }

        true
    }

    pub fn tile_size(&self) -> u32 {
        if let Source::Vector(v) = self {
            return v.tile_size();
        }

        512
    }

    pub fn min_zoom(&self) -> f32 {
        if let Source::Vector(v) = self {
            return v.min_zoom();
        }

        0.0
    }

    pub fn max_zoom(&self) -> f32 {
        if let Source::Vector(v) = self {
            return v.max_zoom();
        }

        0.0
    }

    pub fn round_zoom(&self) -> bool {
        false
    }

    pub fn reparse_overscaled(&self) -> bool {
        false
    }

    pub fn render_world_copies(&self) -> bool {
        true
    }
}
