use super::{
    mercator_x_from_lng, mercator_y_from_lat, mercator_z_from_altitude, LngLat, LngLatBounds,
    MercatorCoordinate,
};
use crate::geo::edge_insets::{EdgeInsets, PaddingOptions};
use crate::source::OverscaledTileId;
use crate::util::wrap;
use nalgebra::{clamp, Matrix, Matrix4, Point2, Vector3};

const PI: f64 = std::f64::consts::PI as f64;

#[derive(Default, Debug)]
pub(crate) struct Transform {
    pub tile_size: u32,
    pub tile_zoom: u32,
    pub lng_range: [f64; 2],
    pub lat_range: [f64; 2],
    pub max_validate_latitude: f64,
    pub scale: f64,
    pub width: f64,
    pub height: f64,
    pub angle: f64,
    // pub rotation_matrix: Matrix2<f64>,
    pub zoom_fraction: f64,
    pub pixels_to_gl_units: [f64; 2],
    pub camera_to_center_distance: f64,
    pub mercator_matrix: Matrix4<f64>,
    pub proj_matrix: Matrix4<f64>,
    pub inv_proj_matrix: Matrix4<f64>,
    pub aligned_proj_matrix: Matrix4<f64>,
    pub pixel_matrix: Matrix4<f64>,
    pub pixel_matrix_inverse: Matrix4<f64>,
    pub gl_coord_matrix: Matrix4<f64>,
    pub label_plane_matrix: Matrix4<f64>,
    fov: f64,
    pitch: f64,
    zoom: f64,
    unmodified: bool,
    render_world_copies: bool,
    min_zoom: f64,
    max_zoom: f64,
    min_pitch: f64,
    max_pitch: f64,
    center: LngLat<f64>,
    edge_insets: EdgeInsets<f64>,
    constraining: bool,
}

impl Transform {
    pub fn new(
        min_zoom: f32,
        max_zoom: f32,
        min_pitch: f32,
        max_pitch: f32,
        render_world_copies: bool,
    ) -> Self {
        let mut transform: Self = Default::default();

        transform.tile_size = 512;
        transform.max_validate_latitude = 85.051129;
        transform.render_world_copies = render_world_copies;
        transform.min_zoom = min_zoom as f64;
        transform.max_zoom = max_zoom as f64;
        transform.min_pitch = min_pitch as f64;
        transform.max_pitch = max_pitch as f64;

        transform.set_max_bounds(None);

        transform.width = 0.0;
        transform.height = 0.0;
        transform.center = LngLat::new(0.0, 0.0);
        transform.set_zoom(1.39);
        transform.angle = 0.0;
        transform.fov = 0.6435011087932844;
        transform.pitch = 0.0;
        transform.unmodified = true;

        transform
    }

    pub fn set_max_bounds(&mut self, bounds: Option<LngLatBounds<f64>>) {
        match bounds {
            Some(bounds) => {
                self.lng_range = [bounds.get_west(), bounds.get_east()];
                self.lat_range = [bounds.get_south(), bounds.get_north()];
                self.constrain();
            }
            None => self.lat_range = [-self.max_validate_latitude, self.max_validate_latitude],
        }
    }

    pub fn bearing(&self) -> f64 {
        -self.angle / PI * 180.0
    }

    pub fn set_bearing(&mut self, bearing: f64) {
        let b = -wrap(bearing, -180.0, 180.0) * PI / 180.0;
        if (self.bearing() - b).abs() < f64::EPSILON {
            return;
        }
        self.unmodified = false;
        self.angle = b;
        self.calc_matrices();

        // self.rotation_matrix = Matrix2::<f64>::identity();
        // self.rotation_matrix *= Matrix4::new_rotation(self.angle);
    }

    pub fn pitch(&self) -> f64 {
        self.pitch / PI * 180.0
    }

    pub fn set_pitch(&mut self, pitch: f64) {
        let p = clamp(pitch, self.min_pitch, self.max_pitch) / 180.0 * PI;
        if (self.pitch() - p).abs() < f64::EPSILON {
            return;
        }

        self.unmodified = false;
        self.pitch = p;
        self.calc_matrices();
    }

    pub fn fov(&self) -> f64 {
        self.fov / PI * 180.0
    }

    pub fn set_fov(&mut self, fov: f64) {
        let f = 0.01f64.max(fov.min(60.0));
        if (self.fov() - f).abs() < f64::EPSILON {
            return;
        }

        self.unmodified = false;
        self.fov = fov / 180.0 * PI;
        self.calc_matrices();
    }

    pub fn zoom(&self) -> f64 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        let z = self.max_zoom.min(self.min_zoom.max(zoom));
        if (self.zoom() - z).abs() < f64::EPSILON {
            return;
        }

        self.unmodified = false;
        self.zoom = z;
        self.scale = Transform::zoom_scale(z);
        self.tile_zoom = z.floor() as u32;
        self.zoom_fraction = z - self.tile_zoom as f64;
        self.constrain();
        self.calc_matrices();
    }

    pub fn center(&self) -> &LngLat<f64> {
        &self.center
    }

    pub fn set_center(&mut self, center: LngLat<f64>) {
        if self.center == center {
            return;
        }

        self.unmodified = false;
        self.center = center;
        self.constrain();
        self.calc_matrices();
    }

    pub fn padding(&self) -> PaddingOptions<f64> {
        self.edge_insets.padding_options()
    }

    pub fn set_padding(&mut self, padding: PaddingOptions<f64>) {
        let current = self.edge_insets.padding_options();
        if current == padding {
            return;
        }

        self.unmodified = false;
        self.edge_insets.interpolate(current, padding, 1.0);
        self.calc_matrices();
    }

    fn scale_zoom(scale: f64) -> f64 {
        scale.ln() / 2.0f64.ln()
    }

    fn zoom_scale(zoom: f64) -> f64 {
        2.0f64.powf(zoom)
    }

    fn covering_zoom_level(&self, round_zoom: bool, tile_size: u32) -> f64 {
        let tmp = self.zoom + Transform::scale_zoom(self.tile_size as f64 / tile_size as f64);
        let z = if round_zoom { tmp.round() } else { tmp.floor() };
        z.max(0.0)
    }

    pub fn center_point(&self) -> Point2<f64> {
        self.edge_insets.center(self.width, self.height)
    }

    pub fn size(&self) -> Point2<f64> {
        Point2::new(self.width, self.height)
    }

    pub fn center_offset(&self) -> Point2<f64> {
        (self.center_point() - self.size() / 2.0).into()
    }

    pub fn point(&self) -> Point2<f64> {
        self.project(&self.center)
    }

    pub fn world_size(&self) -> f64 {
        self.tile_size as f64 * self.scale
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width as f64;
        self.height = height as f64;
        self.pixels_to_gl_units = [2.0 / self.width, -2.0 / self.height];
        self.constrain();
        self.calc_matrices();
    }

    pub fn project(&self, lng_lat: &LngLat<f64>) -> Point2<f64> {
        let lat = clamp(
            lng_lat.lat(),
            -self.max_validate_latitude,
            self.max_validate_latitude,
        );
        let world_size = self.world_size();
        Point2::new(
            mercator_x_from_lng(lng_lat.lng()) * world_size,
            mercator_y_from_lat(lat) * world_size,
        )
    }

    pub fn unproject(&self, point: &Point2<f64>) -> LngLat<f64> {
        MercatorCoordinate::new(
            point.x / self.world_size(),
            point.y / self.world_size(),
            0.0,
        )
        .to_lng_lat()
    }

    fn calc_matrices(&mut self) {
        if self.height == 0.0 {
            return;
        };
        let half_fov = self.fov / 2.0;
        let offset = self.center_offset();
        self.camera_to_center_distance = 0.5 / half_fov.tan() * self.height as f64;

        let ground_angle = PI / 2.0 + self.pitch();
        let fov_above_center = self.fov * (0.5 + offset.y as f64 / self.height as f64);

        let top_half_surface_distance = fov_above_center.sin() * self.camera_to_center_distance
            / clamp(PI - ground_angle - fov_above_center, 0.01, PI - 0.01).sin();
        let point = self.point();
        let (x, y) = (point.x, point.y);

        let furthest_distance = (PI / 2.0 - self.pitch).cos() * top_half_surface_distance
            + self.camera_to_center_distance;
        let far_z = furthest_distance * 1.01;

        let near_z = self.height / 50.0;

        let mut m = Matrix4::new_perspective(
            self.width / self.height,
            self.fov as f64,
            near_z,
            far_z as f64,
        );
        m[8] = -offset.x * 2.0 / self.width;
        m[9] = offset.y * 2.0 / self.height;

        m *= Matrix::new_nonuniform_scaling(&Vector3::new(1.0, -1.0, 1.0));
        m *= Matrix::new_translation(&Vector3::new(0.0, 0.0, -self.camera_to_center_distance));
        m *= Matrix::from_axis_angle(&Vector3::x_axis(), self.pitch);
        m *= Matrix::from_axis_angle(&Vector3::z_axis(), self.angle);
        m *= Matrix::new_translation(&Vector3::new(-x, -y, 0.0));

        self.mercator_matrix = m * Matrix::new_nonuniform_scaling(&Vector3::new(
            self.world_size(),
            self.world_size(),
            self.world_size(),
        ));

        m *= Matrix::new_nonuniform_scaling(&Vector3::new(
            1.0,
            1.0,
            mercator_z_from_altitude(1.0, self.center.lat()) * self.world_size(),
        ));

        self.proj_matrix = m;
        self.inv_proj_matrix = self.proj_matrix.try_inverse().unwrap();

        let x_shift = (self.width % 2.0) / 2.0;
        let y_shift = (self.height % 2.0) / 2.0;
        let angle_cos = self.angle.cos();
        let angle_sin = self.angle.sin();
        let dx = x - x.round() + angle_cos as f64 * x_shift + angle_sin as f64 * y_shift;
        let dy = y - y.round() + angle_cos as f64 * y_shift + angle_sin as f64 * x_shift;
        let mut aligned_m = m;
        aligned_m *= Matrix::new_translation(&Vector3::new(
            if dx > 0.5 { dx - 1.0 } else { dx },
            if dy > 0.5 { dy - 1.0 } else { dy },
            0.0,
        ));
        self.aligned_proj_matrix = aligned_m;

        let mut m = Matrix4::<f64>::identity();
        m *= Matrix::new_nonuniform_scaling(&Vector3::new(
            self.width / 2.0,
            -self.height / 2.0,
            1.0,
        ));
        m *= Matrix::new_translation(&Vector3::new(1.0, -1.0, 0.0));
        self.label_plane_matrix = m;

        let mut m = Matrix4::<f64>::identity();
        m *= Matrix::new_nonuniform_scaling(&Vector3::new(1.0, -1.0, 1.0));
        m *= Matrix::new_translation(&Vector3::new(-1.0, -1.0, 0.0));
        m *=
            Matrix::new_nonuniform_scaling(&Vector3::new(2.0 / self.width, 2.0 / self.height, 1.0));
        self.gl_coord_matrix = m;

        self.pixel_matrix = self.label_plane_matrix * self.proj_matrix;
        self.pixel_matrix_inverse = self.pixel_matrix.try_inverse().unwrap();
    }

    fn constrain(&mut self) {
        if self.width == 0.0 || self.height == 0.0 || !self.constraining {
            return;
        }

        self.constraining = true;
        let mut min_y = -90.0;
        let mut max_y = 90.0;
        let mut min_x = -180.0;
        let mut max_x = 180.0;
        let mut sy = 0.0;
        let mut sx = 0.0;
        let mut x2 = 0.0;
        let mut y2 = 0.0;
        let size = self.size();
        let unmodified = self.unmodified;

        if !self.lat_range.is_empty() {
            min_y = mercator_y_from_lat(self.lat_range[1]) * self.world_size();
            max_y = mercator_y_from_lat(self.lat_range[0]) * self.world_size();
            sy = max_y
                - if min_y < size.y {
                    size.y / (max_y - min_y)
                } else {
                    0.0
                };
        }

        if !self.lng_range.is_empty() {
            min_x = mercator_x_from_lng(self.lng_range[0]) * self.world_size();
            max_x = mercator_x_from_lng(self.lng_range[1]) * self.world_size();
            sx = max_x
                - if min_x < size.x {
                    size.x / (max_x - min_x)
                } else {
                    0.0
                };
        }

        let point = self.point();

        let s = sy.max(sx);

        if s != 0.0 {
            self.set_center(self.unproject(&Point2::new(
                if sx != 0.0 {
                    (max_x + min_x) / 2.0
                } else {
                    point.x
                },
                if sy != 0.0 {
                    (max_y + min_y) / 2.0
                } else {
                    point.y
                },
            )));

            self.set_zoom(self.zoom() + Transform::scale_zoom(s));
            self.unmodified = unmodified;
            self.constraining = false;
        }

        if !self.lat_range.is_empty() {
            let y = point.y;
            let h2 = size.y / 2.0;

            if y - h2 < min_y {
                y2 = min_y + h2;
            }

            if y + h2 > max_y {
                y2 = max_y - h2;
            }
        }

        if !self.lng_range.is_empty() {
            let x = point.x;
            let w2 = size.x / 2.0;

            if x - w2 < min_x {
                x2 = min_x + w2;
            }

            if x + w2 > max_x {
                x2 = max_x - w2;
            }
        }

        if x2 != 0.0 || y2 != 0.0 {
            self.set_center(self.unproject(&Point2::new(
                if x2 != 0.0 { x2 } else { point.x },
                if y2 != 0.0 { y2 } else { point.y },
            )));
        }

        self.unmodified = unmodified;
        self.constraining = false;
    }

    pub fn covering_tiles(
        &self,
        tile_size: u32,
        min_zoom: Option<f32>,
        max_zoom: Option<f32>,
        round_zoom: bool,
        reparse_overscaled: bool,
        render_world_copies: bool,
    ) -> Vec<OverscaledTileId> {
        let mut z = self.covering_zoom_level(round_zoom, tile_size);
        let actual_zoom = z;

        if let Some(min_zoom) = min_zoom {
            if z < min_zoom as f64 {
                return vec![];
            }
        }

        if let Some(max_zoom) = max_zoom {
            if z > max_zoom as f64 {
                z = max_zoom as f64;
            }
        }

        let center_coord = MercatorCoordinate::from_lng_lat(self.center(), 0.0);
        let num_tiles = 2u32.pow(tile_size);
        vec![]
    }
}
