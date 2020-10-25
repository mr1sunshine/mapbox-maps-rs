mod lng_lat;
mod lng_lat_bounds;
mod mercator_coordinate;

/*
* Approximate radius of the earth in meters.
* Uses the WGS-84 approximation. The radius at the equator is ~6378137 and at the poles is ~6356752. https://en.wikipedia.org/wiki/World_Geodetic_System#WGS84
* 6371008.8 is one published "average radius" see https://en.wikipedia.org/wiki/Earth_radius#Mean_radius, or ftp://athena.fsv.cvut.cz/ZFG/grs80-Moritz.pdf p.4
*/
const EARTH_RADIUS: f32 = 6371008.8;

pub(crate) use lng_lat::LngLat;
pub(crate) use lng_lat_bounds::LngLatBounds;
pub(crate) use mercator_coordinate::{
    mercator_x_from_lng, mercator_y_from_lat, MercatorCoordinate,
};
