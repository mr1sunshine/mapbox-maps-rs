mod vector;

use crate::network::NetworkManager;
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
}
