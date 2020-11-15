use super::sources::SourceControl;
use super::tile_cache::TileCache;
use super::{sources::Source, tile::Tile, OverscaledTileId};
use crate::geo::Transform;
use crate::network::NetworkManager;
use crate::style_spec;
use eyre::Result;
use std::sync::Arc;

pub(crate) struct SourceCache {
    source: Source,
    tile_cache: TileCache,
}

impl SourceCache {
    pub async fn new(
        nm: Arc<NetworkManager>,
        name: &str,
        source: &style_spec::Source,
    ) -> Result<Self> {
        let mut source = Source::new(nm, name, source);
        let tile_cache = TileCache::new(0);

        source.load().await?;
        Ok(Self { source, tile_cache })
    }

    pub async fn update(&mut self, transform: &Transform) -> Result<()> {
        let ideal_tile_ids = transform.covering_tiles(
            self.source.tile_size(),
            Some(self.source.min_zoom()),
            Some(self.source.max_zoom()),
            self.source.round_zoom(),
            self.source.reparse_overscaled(),
            self.source.render_world_copies(),
        );
        for tile_id in ideal_tile_ids {
            self.add_tile(tile_id.clone()).await?;
        }
        Ok(())
    }

    async fn add_tile(&mut self, tile_id: OverscaledTileId) -> Result<()> {
        let size = (self.source.tile_size() * tile_id.overscaled_factor()) as usize;
        let mut tile = Tile::new(tile_id, size);
        self.source.load_tile(&mut tile).await?;
        Ok(())
    }
}
