use super::LngLat;
use num::traits::Float;

#[derive(Debug, Clone)]
pub(crate) struct LngLatBounds<T: Float> {
    ne: LngLat<T>,
    sw: LngLat<T>,
}

impl<T: Float> LngLatBounds<T> {
    pub fn new(ne: LngLat<T>, sw: LngLat<T>) -> Self {
        Self { ne, sw }
    }

    pub fn convert(input: &[T; 4]) -> Self {
        let sw = LngLat::new(input[0], input[1]);
        let ne = LngLat::new(input[2], input[3]);
        Self { ne, sw }
    }

    pub fn get_west(&self) -> T {
        self.sw.lng()
    }

    pub fn get_south(&self) -> T {
        self.ne.lat()
    }

    pub fn get_east(&self) -> T {
        self.ne.lng()
    }

    pub fn get_north(&self) -> T {
        self.ne.lat()
    }
}
