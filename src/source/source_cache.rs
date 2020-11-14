use super::sources::Source;
use super::tile_cache::TileCache;
use crate::geo::Transform;
use crate::network::NetworkManager;
use crate::style_spec;
use eyre::Result;
use std::rc::Rc;

pub(crate) struct SourceCache {
    source: Source,
    tile_cache: TileCache,
}

impl SourceCache {
    pub async fn new(
        nm: Rc<NetworkManager>,
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
        Ok(())
    }
}
