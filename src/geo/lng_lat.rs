use crate::util::wrap;
use num::traits::Float;
use std::fmt::{self, Display, Formatter};

const PI: f64 = std::f64::consts::PI;

#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) struct LngLat<T: Float> {
    lng: T,
    lat: T,
}

impl<T: Float> LngLat<T> {
    pub fn new(lng: T, lat: T) -> Self {
        assert!(
            lat <= T::from(90.0).unwrap() && lat >= -T::from(90.0).unwrap(),
            "Invalid LngLat latitude value: must be between -90 and 90"
        );
        Self { lng, lat }
    }

    pub fn wrap(&self) -> LngLat<T> {
        LngLat::new(
            wrap(self.lng, -T::from(180.0).unwrap(), T::from(180.0).unwrap()) as T,
            self.lat,
        )
    }

    pub fn to_array(&self) -> [T; 2] {
        [self.lng, self.lat]
    }

    pub fn lng(&self) -> T {
        self.lng
    }

    pub fn lat(&self) -> T {
        self.lat
    }

    pub fn distance_to(&self, other: &LngLat<T>) -> T {
        let rad: T = T::from(PI).unwrap() / T::from(180.0).unwrap();
        let lat1 = self.lat * rad;
        let lat2 = other.lat * rad;
        let a = lat1.sin() * lat2.sin()
            + lat1.cos() * lat2.cos() * ((other.lng - self.lng) * rad).cos();
        T::from(super::EARTH_RADIUS).unwrap() * a.min(T::from(1.0).unwrap()).acos()
    }
}

impl<T: Float + std::fmt::Display> Display for LngLat<T> {
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
