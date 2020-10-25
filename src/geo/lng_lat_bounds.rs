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

    pub fn convert(input: &[f32; 4]) -> Self {
        let sw = LngLat::new(input[0], input[1]);
        let ne = LngLat::new(input[2], input[3]);
        Self { ne, sw }
    }

    pub fn get_west(&self) -> f32 {
        self.sw.lng()
    }

    pub fn get_south(&self) -> f32 {
        self.ne.lat()
    }

    pub fn get_east(&self) -> f32 {
        self.ne.lng()
    }

    pub fn get_north(&self) -> f32 {
        self.ne.lat()
    }
}
