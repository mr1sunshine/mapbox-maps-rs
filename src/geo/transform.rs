use super::{
    mercator_x_from_lng, mercator_y_from_lat, mercator_z_from_altitude, LngLat, LngLatBounds,
    MercatorCoordinate,
};
use crate::geo::edge_insets::{EdgeInsets, PaddingOptions};
use crate::util::wrap;
use nalgebra::{clamp, Matrix, Matrix4, Point2, Vector3};

const PI: f32 = std::f64::consts::PI as f32;

#[derive(Default)]
pub(crate) struct Transform {
    pub tile_size: u32,
    pub tile_zoom: u32,
    pub lng_range: [f32; 2],
    pub lat_range: [f32; 2],
    pub max_validate_latitude: f32,
    pub scale: f32,
    pub width: f32,
    pub height: f32,
    pub angle: f32,
    // pub rotation_matrix: Matrix2<f32>,
    pub zoom_fraction: f32,
    // pub pixels_to_gl_units: [f32; 2],
    pub camera_to_center_distance: f32,
    pub mercator_matrix: Matrix4<f32>,
    pub proj_matrix: Matrix4<f32>,
    pub inv_proj_matrix: Matrix4<f32>,
    pub aligned_proj_matrix: Matrix4<f32>,
    pub pixel_matrix: Matrix4<f32>,
    pub pixel_matrix_inverse: Matrix4<f32>,
    pub gl_coord_matrix: Matrix4<f32>,
    pub label_plane_matrix: Matrix4<f32>,
    fov: f32,
    pitch: f32,
    zoom: f32,
    unmodified: bool,
    render_world_copies: bool,
    min_zoom: f32,
    max_zoom: f32,
    min_pitch: f32,
    max_pitch: f32,
    center: LngLat,
    edge_insets: EdgeInsets,
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
        transform.max_validate_latitude = 85.051_13;
        transform.render_world_copies = render_world_copies;
        transform.min_zoom = min_zoom;
        transform.max_zoom = max_zoom;
        transform.min_pitch = min_pitch;
        transform.max_pitch = max_pitch;

        transform.set_max_bounds(None);

        transform.width = 0.0;
        transform.height = 0.0;
        transform.set_center(LngLat::new(0.0, 0.0));
        transform.set_zoom(0.0);
        transform.angle = 0.0;
        transform.set_fov(0.643_501_1);
        transform.set_pitch(0.0);
        transform.unmodified = true;

        transform
    }

    pub fn set_max_bounds(&mut self, bounds: Option<LngLatBounds>) {
        match bounds {
            Some(bounds) => {
                self.lng_range = [bounds.get_west(), bounds.get_east()];
                self.lat_range = [bounds.get_south(), bounds.get_north()];
                self.constrain();
            }
            None => self.lat_range = [-self.max_validate_latitude, self.max_validate_latitude],
        }
    }

    pub fn bearing(&self) -> f32 {
        -self.angle / PI * 180.0
    }

    pub fn set_bearing(&mut self, bearing: f32) {
        let b = -wrap(bearing, -180.0, 180.0) * PI / 180.0;
        if (self.bearing() - b).abs() < f32::EPSILON {
            return;
        }
        self.unmodified = false;
        self.angle = b;
        self.calc_matrices();

        // self.rotation_matrix = Matrix2::<f32>::identity();
        // self.rotation_matrix *= Matrix4::new_rotation(self.angle);
    }

    pub fn pitch(&self) -> f32 {
        self.pitch / PI * 180.0
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        let p = clamp(pitch, self.min_pitch, self.max_pitch) / 180.0 * PI;
        if (self.pitch() - p).abs() < f32::EPSILON {
            return;
        }

        self.unmodified = false;
        self.pitch = p;
        self.calc_matrices();
    }

    pub fn fov(&self) -> f32 {
        self.fov / PI * 180.0
    }

    pub fn set_fov(&mut self, fov: f32) {
        let f = 0.01f32.max(fov.min(60.0));
        if (self.fov() - f).abs() < f32::EPSILON {
            return;
        }

        self.unmodified = false;
        self.fov = fov / 180.0 * PI;
        self.calc_matrices();
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        let z = self.max_zoom.min(self.min_zoom.max(zoom));
        if (self.zoom() - z).abs() < f32::EPSILON {
            return;
        }

        self.unmodified = false;
        self.zoom = z;
        self.scale = Transform::zoom_scale(z);
        self.tile_zoom = z.floor() as u32;
        self.zoom_fraction = z - self.tile_zoom as f32;
        self.constrain();
        self.calc_matrices();
    }

    pub fn center(&self) -> &LngLat {
        &self.center
    }

    pub fn set_center(&mut self, center: LngLat) {
        if self.center == center {
            return;
        }

        self.unmodified = false;
        self.center = center;
        self.constrain();
        self.calc_matrices();
    }

    pub fn padding(&self) -> PaddingOptions {
        self.edge_insets.padding_options()
    }

    pub fn set_padding(&mut self, padding: PaddingOptions) {
        let current = self.edge_insets.padding_options();
        if current == padding {
            return;
        }

        self.unmodified = false;
        self.edge_insets.interpolate(current, padding, 1.0);
        self.calc_matrices();
    }

    fn scale_zoom(scale: f32) -> f32 {
        scale.ln() / 2.0f32.ln()
    }

    fn zoom_scale(zoom: f32) -> f32 {
        2.0f32.powf(zoom)
    }

    fn covering_zoom_level(&self, round_zoom: bool, tile_size: u32) -> f32 {
        let tmp = self.zoom + Transform::scale_zoom(self.tile_size as f32 / tile_size as f32);
        let z = if round_zoom { tmp.round() } else { tmp.floor() };
        z.max(0.0)
    }

    pub fn center_point(&self) -> Point2<f32> {
        self.edge_insets.center(self.width, self.height)
    }

    pub fn size(&self) -> Point2<f32> {
        Point2::new(self.width, self.height)
    }

    pub fn center_offset(&self) -> Point2<f32> {
        (self.center_point() - self.size() / 2.0).into()
    }

    pub fn point(&self) -> Point2<f32> {
        self.project(&self.center)
    }

    pub fn world_size(&self) -> f32 {
        self.tile_size as f32 * self.scale
    }

    pub fn project(&self, lng_lat: &LngLat) -> Point2<f32> {
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

    pub fn unproject(&self, point: &Point2<f32>) -> LngLat {
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
        self.camera_to_center_distance = 0.5 / half_fov.tan() * self.height;

        let ground_angle = PI / 2.0 + self.pitch();
        let fov_above_center = self.fov * (0.5 + offset.y / self.height);

        let top_half_surface_distance = fov_above_center.sin() * self.camera_to_center_distance
            / clamp(PI - ground_angle - fov_above_center, 0.01, PI - 0.01).sin();
        let point = self.point();
        let (x, y) = (point.x, point.y);

        let furthest_distance = (PI / 2.0 - self.pitch).cos() * top_half_surface_distance
            + self.camera_to_center_distance;
        let far_z = furthest_distance * 1.01;

        let near_z = self.height / 50.0;

        let mut m = Matrix4::new_perspective(self.width / self.height, self.fov, near_z, far_z);

        m[8] = -offset.x * 2.0 / self.width;
        m[9] = offset.y * 2.0 / self.height;

        m *= Matrix::from_scaled_axis(Vector3::new(1.0, -1.0, 1.0));
        m.append_translation_mut(&Vector3::new(0.0, 0.0, -self.camera_to_center_distance));
        m *= Matrix4::from_scaled_axis(Vector3::x() * self.pitch);
        m *= Matrix4::from_scaled_axis(Vector3::z() * self.angle);
        m.append_translation_mut(&Vector3::new(-x, -y, 0.0));

        self.mercator_matrix = m * Matrix::from_scaled_axis(Vector3::new(
            self.world_size(),
            self.world_size(),
            self.world_size(),
        ));

        m *= Matrix::from_scaled_axis(Vector3::new(
            1.0,
            1.0,
            mercator_z_from_altitude(1.0, self.center.lat()),
        ));

        self.proj_matrix = m;
        self.inv_proj_matrix = self.proj_matrix.try_inverse().unwrap();

        let x_shift = (self.width % 2.0) / 2.0;
        let y_shift = (self.height % 2.0) / 2.0;
        let angle_cos = self.angle.cos();
        let angle_sin = self.angle.sin();
        let dx = x - x.round() + angle_cos * x_shift + angle_sin * y_shift;
        let dy = y - y.round() + angle_cos * y_shift + angle_sin * x_shift;
        let mut aligned_m = m;
        aligned_m.append_translation_mut(&Vector3::new(
            if dx > 0.5 { dx - 1.0 } else { dx },
            if dy > 0.5 { dy - 1.0 } else { dy },
            0.0,
        ));
        self.aligned_proj_matrix = aligned_m;

        let mut m = Matrix4::<f32>::identity();
        m *= Matrix::from_scaled_axis(Vector3::new(self.width / 2.0, -self.height / 2.0, 1.0));
        m.append_translation_mut(&Vector3::new(1.0, -1.0, 0.0));
        self.label_plane_matrix = m;

        let mut m = Matrix4::<f32>::identity();
        m *= Matrix::from_scaled_axis(Vector3::new(1.0, -1.0, 1.0));
        m.append_translation_mut(&Vector3::new(-1.0, -1.0, 0.0));
        m *= Matrix::from_scaled_axis(Vector3::new(2.0 / self.width, 2.0 / self.height, 1.0));
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
}
