use crate::util::wrap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) struct LngLat {
    lng: f32,
    lat: f32,
}

impl LngLat {
    pub fn new(lng: f32, lat: f32) -> Self {
        assert!(
            lat <= 90.0 && lat >= -90.0,
            "Invalid LngLat latitude value: must be between -90 and 90"
        );
        Self { lng, lat }
    }

    pub fn wrap(&self) -> LngLat {
        LngLat::new(wrap(self.lng, -180.0, 180.0), self.lat)
    }

    pub fn to_array(&self) -> [f32; 2] {
        [self.lng, self.lat]
    }

    pub fn lng(&self) -> f32 {
        self.lng
    }

    pub fn lat(&self) -> f32 {
        self.lat
    }

    pub fn distance_to(&self, other: &LngLat) -> f32 {
        let rad: f32 = std::f64::consts::PI as f32 / 180.0;
        let lat1 = self.lat * rad;
        let lat2 = other.lat * rad;
        let a = lat1.sin() * lat2.sin()
            + lat1.cos() * lat2.cos() * ((other.lng - self.lng) * rad).cos();
        super::EARTH_RADIUS * a.min(1.0).acos()
    }
}

impl Display for LngLat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "LngLat({}, {})", self.lng, self.lat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn lng_lat_new() {
        let ll = LngLat::new(286.0251, 40.7736);
        assert_approx_eq!(ll.lng(), 286.0251);
        assert_approx_eq!(ll.lat(), 40.7736);
    }

    #[test]
    fn lng_lat_wrap() {
        let ll = LngLat::new(286.0251, 40.7736);
        let wrapped = ll.wrap();
        assert_approx_eq!(wrapped.lng(), -73.974915);
    }

    #[test]
    fn lng_lat_to_array() {
        let ll = LngLat::new(286.0251, 40.7736);
        let arr = ll.to_array();
        assert_approx_eq!(arr[0], 286.0251);
        assert_approx_eq!(arr[1], 40.7736);
    }

    #[test]
    fn lng_lat_distance_to() {
        let new_york = LngLat::new(-74.0060, 40.7128);
        let los_angeles = LngLat::new(-118.2437, 34.0522);
        assert_approx_eq!(new_york.distance_to(&los_angeles), 3935752.3);
    }
}
