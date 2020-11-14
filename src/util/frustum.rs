use nalgebra::{Matrix4, Vector4};

#[derive(Debug)]
pub(crate) struct Frustum {
    points: Vec<Vector4<f64>>,
    planes: Vec<Vector4<f64>>,
}

impl Frustum {
    pub fn new(inv_proj: &Vec<f64>, world_size: f64, zoom: f64) -> Self {
        let inv_proj = Matrix4::from_vec(inv_proj.clone());
        let clip_space_corners = [
            Vector4::new(-1.0, 1.0, -1.0, 1.0),
            Vector4::new(1.0, 1.0, -1.0, 1.0),
            Vector4::new(1.0, -1.0, -1.0, 1.0),
            Vector4::new(-1.0, -1.0, -1.0, 1.0),
            Vector4::new(-1.0, 1.0, 1.0, 1.0),
            Vector4::new(1.0, 1.0, 1.0, 1.0),
            Vector4::new(1.0, -1.0, 1.0, 1.0),
            Vector4::new(-1.0, -1.0, 1.0, 1.0),
        ];

        let scale = 2f64.powf(zoom);

        let points: Vec<Vector4<f64>> = clip_space_corners
            .iter()
            .map(|v| inv_proj * v)
            .map(|v| v.scale(1.0 / v[3] / world_size * scale))
            .collect();

        const FRUSTUM_PLANE_POINT_INDICES: &[&[usize]] = &[
            &[0, 1, 2], // near
            &[6, 5, 4], // far
            &[0, 3, 7], // left
            &[2, 1, 5], // right
            &[3, 2, 6], // bottom
            &[0, 4, 5], // top
        ];

        let planes: Vec<Vector4<f64>> = FRUSTUM_PLANE_POINT_INDICES
            .iter()
            .map(|p| {
                let a = points[p[0]] - points[p[1]];
                let a = a.remove_row(3);
                let b = points[p[2]] - points[p[1]];
                let b = b.remove_row(3);
                let n = a.cross(&b).normalize();
                let d = -n.dot(&points[p[1]].remove_row(3));
                n.insert_row(3, d)
            })
            .collect();

        Self { points, planes }
    }

    pub fn points(&self) -> &Vec<Vector4<f64>> {
        &self.points
    }

    pub fn planes(&self) -> &Vec<Vector4<f64>> {
        &self.planes
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

        let points = vec![
            Vector4::new(
                0.9745623465173241,
                0.986704851421945,
                88050.65437506074,
                0.0014904875087505462,
            ),
            Vector4::new(
                1.0254376534826761,
                0.986704851421945,
                88050.65437506074,
                0.0014904875087505462,
            ),
            Vector4::new(
                1.0254376534826761,
                1.013295148578055,
                88050.65437506074,
                0.0014904875087505462,
            ),
            Vector4::new(
                0.9745623465173241,
                1.013295148578055,
                88050.65437506074,
                0.0014904875087505462,
            ),
            Vector4::new(
                -0.926902251312708,
                -0.007107504787660734,
                -892.4052808279938,
                0.0014904875087505462,
            ),
            Vector4::new(
                2.926902251312682,
                -0.007107504787660734,
                -892.4052808279938,
                0.0014904875087505462,
            ),
            Vector4::new(
                2.926902251312682,
                2.0071075047876454,
                -892.4052808279938,
                0.0014904875087505462,
            ),
            Vector4::new(
                -0.926902251312708,
                2.0071075047876454,
                -892.4052808279938,
                0.0014904875087505462,
            ),
        ];
        assert_eq!(points, *f.points());

        let planes = vec![
            Vector4::new(0.0, 0.0, -1.0, 88050.65437506074),
            Vector4::new(0.0, -0.0, 1.0, 892.4052808279938),
            Vector4::new(
                0.9999999997714809,
                0.0,
                -0.00002137844824264058,
                0.9078240109932523,
            ),
            Vector4::new(
                -0.9999999997714809,
                0.0,
                -0.00002137844824264029,
                2.9078240105361886,
            ),
            Vector4::new(
                -0.0,
                -0.9999999999375756,
                -0.000011173579591173352,
                1.9971361432294379,
            ),
            Vector4::new(
                0.0,
                0.9999999999375757,
                -0.000011173579591173525,
                -0.0028638566456980962,
            ),
        ];

        assert_eq!(planes, *f.planes());
    }
}
