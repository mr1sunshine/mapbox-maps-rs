use super::LngLat;

#[derive(Debug, Clone)]
pub(crate) struct LngLatBounds {
    ne: LngLat,
    sw: LngLat,
}

impl LngLatBounds {
    pub fn new(ne: LngLat, sw: LngLat) -> Self {
        Self { ne, sw }
    }
}
