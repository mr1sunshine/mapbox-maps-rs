use super::tile_id::OverscaledTileId;
use crate::util::unique_id;

#[derive(Debug)]
enum TileState {
    Loading,
    Loaded,
    Reloading,
    Unloaded,
    Errored,
    Expired,
}

#[derive(Debug)]
pub(crate) struct Tile {
    tile_id: OverscaledTileId,
    state: TileState,
    uid: usize,
    size: usize,
    vector_data: mvt::Tile<mvt::FeatureWithCoordinates>,
}

impl Tile {
    pub fn new(tile_id: OverscaledTileId, size: usize) -> Self {
        Self {
            tile_id,
            state: TileState::Loading,
            uid: unique_id(),
            size,
            vector_data: Default::default(),
        }
    }

    pub fn tile_id(&self) -> &OverscaledTileId {
        &self.tile_id
    }

    pub fn set_vector_data(&mut self, vector_data: mvt::Tile<mvt::FeatureWithCoordinates>) {
        self.vector_data = vector_data;
    }
}
