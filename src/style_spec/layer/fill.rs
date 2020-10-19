use super::super::{
    types::{Color, Type},
    Expression,
};
use super::{Anchor, Visibility};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct FillLayerPaint {
    #[serde(rename = "fill-antialias", default = "default_antialias")]
    pub antialias: bool,
    #[serde(rename = "fill-color", default = "default_color")]
    pub color: Expression,
    #[serde(rename = "fill-opacity", default = "default_opacity")]
    pub opacity: Expression,
    #[serde(rename = "fill-outline-color")]
    pub outline_color: Option<Expression>,
    #[serde(rename = "fill-pattern")]
    pub pattern: Option<Expression>,
    #[serde(rename = "fill-translate", default = "default_translate")]
    pub translate: Expression,
    #[serde(rename = "fill-translate-anchor", default = "default_translate_anchor")]
    pub translate_anchor: Anchor,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct FillLayerLayout {
    #[serde(rename = "fill-sort-key")]
    pub sort_key: Option<f32>,
    #[serde(default, rename = "visibility")]
    pub visibility: Visibility,
}

fn default_antialias() -> bool {
    true
}

fn default_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#000000").unwrap()))
}

fn default_opacity() -> Expression {
    Expression::Type(Type::Number(1.0))
}

fn default_translate() -> Expression {
    Expression::Type(Type::Array(vec![0.0, 0.0]))
}

fn default_translate_anchor() -> Anchor {
    Anchor::Map
}
