use super::super::{
    types::{Color, Type},
    Expression,
};
use super::{Anchor, Visibility};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct CircleLayerPaint {
    #[serde(rename = "circle-blur", default = "default_blur")]
    pub blur: Expression,
    #[serde(rename = "circle-color", default = "default_color")]
    pub color: Expression,
    #[serde(rename = "circle-opacity", default = "default_opacity")]
    pub opacity: Expression,
    #[serde(rename = "circle-pitch-alignment", default = "default_pitch_alignment")]
    pub pitch_alignment: Pitch,
    #[serde(rename = "circle-pitch-scale", default = "default_pitch_scale")]
    pub pitch_scale: Pitch,
    #[serde(rename = "circle-radius", default = "default_radius")]
    pub radius: Expression,
    #[serde(rename = "circle-stroke-color", default = "default_stroke_color")]
    pub stroke_color: Expression,
    #[serde(rename = "circle-stroke-opacity", default = "default_stroke_opacity")]
    pub stroke_opacity: Expression,
    #[serde(rename = "circle-stroke-width", default = "default_stroke_width")]
    pub stroke_width: Expression,
    #[serde(rename = "circle-translate", default = "default_translate")]
    pub translate: Expression,
    #[serde(
        rename = "circle-translate-anchor",
        default = "default_translate_anchor"
    )]
    pub translate_anchor: Anchor,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct CircleLayerLayout {
    #[serde(rename = "circle-sort-key")]
    pub sort_key: Option<f32>,
    #[serde(default, rename = "visibility")]
    pub visibility: Visibility,
}

#[derive(Deserialize, Debug)]
pub enum Pitch {
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "viewport")]
    Viewport,
}

impl Default for Pitch {
    fn default() -> Self {
        Pitch::Map
    }
}

fn default_blur() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#000000").unwrap()))
}

fn default_opacity() -> Expression {
    Expression::Type(Type::Number(1.0))
}

fn default_pitch_alignment() -> Pitch {
    Pitch::Viewport
}

fn default_pitch_scale() -> Pitch {
    Pitch::Map
}

fn default_radius() -> Expression {
    Expression::Type(Type::Number(5.0))
}

fn default_stroke_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#000000").unwrap()))
}

fn default_stroke_opacity() -> Expression {
    Expression::Type(Type::Number(1.0))
}

fn default_stroke_width() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_translate() -> Expression {
    Expression::Type(Type::Array(vec![0.0, 0.0]))
}

fn default_translate_anchor() -> Anchor {
    Anchor::Map
}
