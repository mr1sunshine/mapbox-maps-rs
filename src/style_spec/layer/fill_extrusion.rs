use super::super::{
    types::{Color, Type},
    Expression,
};
use super::{Anchor, Visibility};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct FillExtrusionLayerPaint {
    #[serde(rename = "fill-extrusion-base", default = "default_base")]
    pub base: Expression,
    #[serde(rename = "fill-extrusion-color", default = "default_color")]
    pub color: Expression,
    #[serde(rename = "fill-extrusion-height", default = "default_height")]
    pub height: Expression,
    #[serde(rename = "fill-extrusion-opacity", default = "default_opacity")]
    pub opacity: Expression,
    #[serde(rename = "fill-extrusion-pattern")]
    pub pattern: Option<Expression>,
    #[serde(rename = "fill-extrusion-translate", default = "default_translate")]
    pub translate: Expression,
    #[serde(
        rename = "fill-extrusion-translate-anchor",
        default = "default_translate_anchor"
    )]
    pub translate_anchor: Anchor,
    #[serde(
        rename = "fill-extrusion-vertical-gradient",
        default = "default_vertical_gradient"
    )]
    pub vertical_gradient: bool,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct FillExtrusionLayerLayout {
    #[serde(default, rename = "visibility")]
    pub visibility: Visibility,
}

fn default_base() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#000000").unwrap()))
}

fn default_height() -> Expression {
    Expression::Type(Type::Number(0.0))
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

fn default_vertical_gradient() -> bool {
    true
}
