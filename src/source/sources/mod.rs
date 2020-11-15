mod geo_json;
mod image;
mod raster;
mod raster_dem;
mod vector;
mod video;

use crate::network::NetworkManager;
use crate::source::OverscaledTileId;
use crate::style_spec;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use eyre::Result;
use geo_json::GeoJSON;
use image::Image;
use raster::Raster;
use raster_dem::RasterDEM;
use std::sync::Arc;
use vector::Vector;
use video::Video;

use super::tile::Tile;

#[async_trait]
#[enum_dispatch]
pub(crate) trait SourceControl {
    async fn load(&mut self) -> Result<()>;
    fn has_tile(&self, tile_id: &OverscaledTileId) -> bool;
    async fn load_tile(&self, tile: &mut Tile) -> Result<()>;
    fn tile_size(&self) -> u32;
    fn min_zoom(&self) -> f32;
    fn max_zoom(&self) -> f32;
    fn round_zoom(&self) -> bool;
    fn reparse_overscaled(&self) -> bool;
    fn render_world_copies(&self) -> bool;
}

#[enum_dispatch(SourceControl)]
#[derive(Debug)]
pub(crate) enum Source {
    Vector(Vector),
    Raster(Raster),
    RasterDEM(RasterDEM),
    GeoJSON(GeoJSON),
    Video(Video),
    Image(Image),
}

impl Source {
    pub fn new(nm: Arc<NetworkManager>, name: &str, source: &style_spec::Source) -> Source {
        match source {
            style_spec::Source::Vector(data) => Source::Vector(Vector::new(nm, name, data)),
            style_spec::Source::Raster(data) => Source::Raster(Raster::new(nm, name, data)),
            style_spec::Source::RasterDEM(data) => {
                Source::RasterDEM(RasterDEM::new(nm, name, data))
            }
            style_spec::Source::GeoJSON(data) => Source::GeoJSON(GeoJSON::new(nm, name, data)),
            style_spec::Source::Video(data) => Source::Video(Video::new(nm, name, data)),
            style_spec::Source::Image(data) => Source::Image(Image::new(nm, name, data)),
        }
    }
}
