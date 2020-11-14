use super::Frustum;
use nalgebra::{Point2, Vector3, Vector4};

#[derive(Debug)]
pub(crate) struct Aabb {
    min: Vector3<f64>,
    max: Vector3<f64>,
    center: Vector3<f64>,
}

#[derive(Debug, PartialEq)]
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
        assert!(self.min[2] == 0.0 && self.max[2] == 0.0);

        let aabb_points = vec![
            Vector4::new(self.min[0], self.min[1], 0.0, 1.0),
            Vector4::new(self.max[0], self.min[1], 0.0, 1.0),
            Vector4::new(self.max[0], self.max[1], 0.0, 1.0),
            Vector4::new(self.min[0], self.max[1], 0.0, 1.0),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mat = vec![
            0.637767812655705,
            0.0,
            0.0,
            0.0,
            0.0,
            -0.3333333333333333,
            0.0,
            0.0,
            -12.370410580904272,
            -12.37041058090427,
            -1103941.9728413473,
            -0.0184379424489534,
            12.701391466012076,
            12.701391466012076,
            1103646.603751624,
            0.01893126532384179,
        ];

        let world_size = 1341.8428455509638;
        let zoom = 1.0;
        let f = Frustum::new(&mat, world_size, zoom);

        let aabb = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 0.0));

        assert_eq!(IntersectionType::Intersecting, aabb.intersects(&f));
    }
}
