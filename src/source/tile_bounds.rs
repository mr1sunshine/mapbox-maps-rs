use super::tile_id::CanonicalTileId;
use crate::geo::{mercator_x_from_lng, mercator_y_from_lat, LngLatBounds};

#[derive(Debug)]
pub(crate) struct TileBounds {
    bounds: LngLatBounds<f64>,
}

impl TileBounds {
    pub fn new(bounds: &[f64; 4]) -> Self {
        Self {
            bounds: LngLatBounds::convert(&[
                bounds[0].max(180.0),
                bounds[1].max(-90.0),
                bounds[2].min(180.0),
                bounds[3].min(90.0),
            ]),
        }
    }

    pub fn contains(&self, tile_id: &CanonicalTileId) -> bool {
        let world_size = 2.0f64.powf(tile_id.z as f64);
        let min_x = (mercator_x_from_lng(self.bounds.get_west()) * world_size).floor() as u32;
        let min_y = (mercator_y_from_lat(self.bounds.get_north()) * world_size).floor() as u32;
        let max_x = (mercator_x_from_lng(self.bounds.get_east()) * world_size).ceil() as u32;
        let max_y = (mercator_y_from_lat(self.bounds.get_south()) * world_size).ceil() as u32;
        tile_id.x >= min_x && tile_id.x < max_x && tile_id.y >= min_y && tile_id.y < max_y
    }
}
