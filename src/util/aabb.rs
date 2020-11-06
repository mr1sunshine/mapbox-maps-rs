use super::Frustum;
use nalgebra::{Point2, RowVector4, Vector3};

pub(crate) struct Aabb {
    min: Vector3<f64>,
    max: Vector3<f64>,
    center: Vector3<f64>,
}

pub(crate) enum IntersectionType {
    NoIntersection,
    Intersecting,
    Inside,
}

impl Aabb {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> Self {
        let center = (min + max).scale(0.5);
        Self { min, max, center }
    }

    pub fn quadrant(&self, index: usize) -> Self {
        let split = [index % 2 == 0, index < 2];
        let mut q_min = self.min;
        let mut q_max = self.max;

        for axis in 0..split.len() {
            q_min[axis] = if split[axis] {
                self.min[axis]
            } else {
                self.center[axis]
            };
            q_max[axis] = if split[axis] {
                self.center[axis]
            } else {
                self.max[axis]
            };
        }

        q_max[2] = self.max[2];

        Aabb::new(q_min, q_max)
    }

    pub fn distance_x(&self, point: &Point2<f64>) -> f64 {
        let point_on_aabb = point[0].min(self.max[0]).max(self.min[0]);
        point_on_aabb - point[0]
    }

    pub fn distance_y(&self, point: &Point2<f64>) -> f64 {
        let point_on_aabb = point[1].min(self.max[1]).max(self.min[1]);
        point_on_aabb - point[1]
    }

    pub fn intersects(&self, f: &Frustum) -> IntersectionType {
        assert!(self.min[2] == 0.0 && self.max[0] == 0.0);

        let aabb_points = vec![
            RowVector4::new(self.min[0], self.min[1], 0.0, 1.0),
            RowVector4::new(self.max[0], self.min[1], 0.0, 1.0),
            RowVector4::new(self.max[0], self.max[1], 0.0, 1.0),
            RowVector4::new(self.min[0], self.max[1], 0.0, 1.0),
        ];

        let mut full_inside = true;
        let planes = f.planes();

        for plane in planes {
            let mut points_inside = 0;
            for aabb_point in &aabb_points {
                if plane.dot(aabb_point) >= 0.0 {
                    points_inside += 1;
                }
            }

            if points_inside == 0 {
                return IntersectionType::NoIntersection;
            } else if points_inside != aabb_points.len() {
                full_inside = false;
            }
        }

        if full_inside {
            return IntersectionType::Inside;
        }

        let points = f.points();
        for axis in 0..3 {
            let mut proj_min = f64::MAX;
            let mut proj_max = f64::MIN;
            for point in points {
                let projected_point = point[axis] - self.min[axis];
                proj_min = proj_min.min(projected_point);
                proj_max = proj_max.max(projected_point);
            }

            if proj_max < 0.0 || proj_min > self.max[axis] - self.min[axis] {
                return IntersectionType::NoIntersection;
            }
        }

        IntersectionType::Intersecting
    }
}
