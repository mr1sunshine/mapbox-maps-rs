use super::SourceControl;
use crate::network::NetworkManager;
use crate::source::tile::Tile;
use crate::source::tile_bounds::TileBounds;
use crate::source::OverscaledTileId;
use crate::style_spec;
use async_trait::async_trait;
use eyre::Result;
use std::convert::TryFrom;
use std::sync::Arc;
use tilejson::TileJson;

#[derive(Debug)]
pub(crate) struct Vector {
    nm: Arc<NetworkManager>,
    name: String,
    tilejson: Option<TileJson>,
    tile_bounds: Option<TileBounds>,
    options: style_spec::Vector,
}

impl Vector {
    pub fn new(nm: Arc<NetworkManager>, name: &str, options: &style_spec::Vector) -> Self {
        Self {
            nm,
            name: name.to_owned(),
            tilejson: None,
            tile_bounds: None,
            options: options.clone(),
        }
    }
}

#[async_trait]
impl SourceControl for Vector {
    async fn load(&mut self) -> Result<()> {
        self.tilejson = match &self.options.url {
            Some(url) => {
                let tilejson = self.nm.load_tilejson(&url).await?;
                let tilejson = serde_json::from_str::<TileJson>(&tilejson)?;
                self.tile_bounds = Some(TileBounds::new(
                    <&[f64; 4]>::try_from(&tilejson.bounds[0..4]).unwrap(),
                ));
                Some(tilejson)
            }
            None => None,
        };
        println!("Source \"{}\" loaded", self.name);
        Ok(())
    }

    fn has_tile(&self, tile_id: &OverscaledTileId) -> bool {
        let tile_bounds = match &self.tile_bounds {
            Some(tile_bounds) => tile_bounds,
            None => return false,
        };

        tile_bounds.contains(tile_id.canonical())
    }

    async fn load_tile(&self, tile: &mut Tile) -> Result<()> {
        let tilejson = match &self.tilejson {
            Some(t) => t,
            None => return Ok(()),
        };
        let url = tile
            .tile_id()
            .canonical()
            .url(&tilejson.tiles, Some(self.options.scheme.clone()));

        let vector_tile = self.nm.load_vector_tile(&url).await?;
        tile.set_vector_data(vector_tile);

        Ok(())
    }

    fn tile_size(&self) -> u32 {
        512
    }

    fn min_zoom(&self) -> f32 {
        match &self.tilejson {
            Some(t) => t.minzoom as f32,
            None => 22.0,
        }
    }

    fn max_zoom(&self) -> f32 {
        match &self.tilejson {
            Some(t) => t.maxzoom as f32,
            None => 22.0,
        }
    }

    fn round_zoom(&self) -> bool {
        false
    }

    fn reparse_overscaled(&self) -> bool {
        false
    }

    fn render_world_copies(&self) -> bool {
        true
    }
}
