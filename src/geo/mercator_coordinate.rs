use super::LngLat;

const PI: f32 = std::f64::consts::PI as f32;
const EARTH_CIRCUMFRENCE: f32 = 2.0 * PI * super::EARTH_RADIUS; // meters

fn circumfrence_at_latitude(lat: f32) -> f32 {
    EARTH_CIRCUMFRENCE * (lat * PI / 180.0).cos()
}

pub(crate) fn mercator_x_from_lng(lng: f32) -> f32 {
    (180.0 + lng) / 360.0
}

pub(crate) fn mercator_y_from_lat(lat: f32) -> f32 {
    (180.0 - (180.0 / PI * (PI / 4.0 + lat * PI / 360.0).tan().ln())) / 360.0
}

fn mercator_z_from_altitude(altitude: f32, lat: f32) -> f32 {
    altitude / circumfrence_at_latitude(lat)
}

fn lng_from_mercator_x(x: f32) -> f32 {
    x * 360.0 - 180.0
}

fn lat_from_mercator_y(y: f32) -> f32 {
    let y2 = 180.0 - y * 360.0;
    360.0 / PI * (y2 * PI / 180.0).exp().atan() - 90.0
}

pub(crate) struct MercatorCoordinate {
    x: f32,
    y: f32,
    z: f32,
}

impl MercatorCoordinate {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_lng_lat(ll: &LngLat, altitude: f32) -> Self {
        MercatorCoordinate::new(
            mercator_x_from_lng(ll.lng()),
            mercator_y_from_lat(ll.lat()),
            mercator_z_from_altitude(altitude, ll.lat()),
        )
    }

    pub fn to_lng_lat(&self) -> LngLat {
        LngLat::new(lng_from_mercator_x(self.x), lat_from_mercator_y(self.y))
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
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
