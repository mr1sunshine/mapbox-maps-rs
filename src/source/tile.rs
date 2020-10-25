use super::tile_id::OverscaledTileId;
use crate::util::unique_id;

enum TileState {
    Loading,
    Loaded,
    Reloading,
    Unloaded,
    Errored,
    Expired,
}

pub(crate) struct Tile {
    tile_id: OverscaledTileId,
    state: TileState,
    uid: usize,
    size: usize,
}

impl Tile {
    pub fn new(tile_id: OverscaledTileId, size: usize) -> Self {
        Self {
            tile_id,
            state: TileState::Loading,
            uid: unique_id(),
            size,
        }
    }
}
