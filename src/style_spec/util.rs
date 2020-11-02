use super::Color;

pub(crate) fn number(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub(crate) fn color(from: &Color, to: &Color, t: f32) -> Color {
    Color::new_with_rgba(
        number(from.red(), to.red(), t),
        number(from.green(), to.green(), t),
        number(from.blue(), to.blue(), t),
        number(from.alpha(), to.alpha(), t),
    )
}

pub(crate) fn array(from: &[f32], to: &[f32], t: f32) -> Vec<f32> {
    from.iter()
        .enumerate()
        .map(|(i, e)| number(*e, to[i], t))
        .collect()
}
