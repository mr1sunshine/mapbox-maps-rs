pub(crate) fn wrap(n: f32, min: f32, max: f32) -> f32 {
    let d = max - min;
    let w = ((n - min) % d + d) % d + min;
    if (w - min).abs() < f32::EPSILON {
        max
    } else {
        w
    }
}
