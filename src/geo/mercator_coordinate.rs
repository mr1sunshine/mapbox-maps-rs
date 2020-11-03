use super::LngLat;
use num::traits::Float;

const PI: f64 = std::f64::consts::PI;
const EARTH_CIRCUMFRENCE: f64 = 2.0 * PI * super::EARTH_RADIUS; // meters

fn circumfrence_at_latitude<T: Float>(lat: T) -> T {
    T::from(EARTH_CIRCUMFRENCE).unwrap()
        * (lat * T::from(PI).unwrap() / T::from(180.0).unwrap()).cos()
}

pub(crate) fn mercator_x_from_lng<T: Float>(lng: T) -> T {
    (T::from(180.0).unwrap() + lng) / T::from(360.0).unwrap()
}

pub(crate) fn mercator_y_from_lat<T: Float>(lat: T) -> T {
    (T::from(180.0).unwrap()
        - (T::from(180.0).unwrap() / T::from(PI).unwrap()
            * (T::from(PI).unwrap() / T::from(4.0).unwrap()
                + lat * T::from(PI).unwrap() / T::from(360.0).unwrap())
            .tan()
            .ln()))
        / T::from(360.0).unwrap()
}

pub(crate) fn mercator_z_from_altitude<T: Float>(altitude: T, lat: T) -> T {
    altitude / circumfrence_at_latitude(lat)
}

fn lng_from_mercator_x<T: Float>(x: T) -> T {
    x * T::from(360.0).unwrap() - T::from(180.0).unwrap()
}

fn lat_from_mercator_y<T: Float>(y: T) -> T {
    let y2 = T::from(180.0).unwrap() - y * T::from(360.0).unwrap();
    T::from(360.0).unwrap() / T::from(PI).unwrap()
        * (y2 * T::from(PI).unwrap() / T::from(180.0).unwrap())
            .exp()
            .atan()
        - T::from(90.0).unwrap()
}

pub(crate) struct MercatorCoordinate<T: Float> {
    x: T,
    y: T,
    z: T,
}

impl<T: Float> MercatorCoordinate<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn from_lng_lat(ll: &LngLat<T>, altitude: T) -> Self {
        MercatorCoordinate::new(
            mercator_x_from_lng(ll.lng()),
            mercator_y_from_lat(ll.lat()),
            mercator_z_from_altitude(altitude, ll.lat()),
        )
    }

    pub fn to_lng_lat(&self) -> LngLat<T> {
        LngLat::new(lng_from_mercator_x(self.x), lat_from_mercator_y(self.y))
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn z(&self) -> T {
        self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn from_lng_lat() {
        let ll = LngLat::new(0.0, 0.0);
        let coord = MercatorCoordinate::from_lng_lat(&ll, 0.0);
        assert_approx_eq!(coord.x(), 0.5);
        assert_approx_eq!(coord.y(), 0.5);
        assert_approx_eq!(coord.z(), 0.0);
    }

    #[test]
    fn to_lng_lat() {
        let coord = MercatorCoordinate::new(0.5, 0.5, 0.0);
        let ll = coord.to_lng_lat();
        assert_approx_eq!(ll.lng(), 0.0);
        assert_approx_eq!(ll.lat(), 0.0);
    }
}
