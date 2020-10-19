use super::super::{
    types::{Color, Type},
    Expression,
};
use super::{Anchor, Visibility};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct LineLayerPaint {
    #[serde(rename = "line-blur", default = "default_blur")]
    pub blur: Expression,
    #[serde(rename = "line-color", default = "default_color")]
    pub color: Expression,
    #[serde(rename = "line-dasharray")]
    pub dasharray: Option<Vec<Expression>>,
    #[serde(rename = "line-gap-width", default = "default_gap_width")]
    pub gap_width: Expression,
    #[serde(rename = "line-gradient")]
    pub gradient: Option<Expression>,
    #[serde(rename = "line-offset", default = "default_offset")]
    pub offset: Expression,
    #[serde(rename = "line-opacity", default = "default_opacity")]
    pub opacity: Expression,
    #[serde(rename = "line-pattern")]
    pub pattern: Option<Expression>,
    #[serde(rename = "line-translate", default = "default_translate")]
    pub translate: Expression,
    #[serde(rename = "line-translate-anchor", default = "default_translate_anchor")]
    pub translate_anchor: Anchor,
    #[serde(rename = "line-width", default = "default_width")]
    pub width: Expression,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum Cap {
    #[serde(rename = "butt")]
    Butt,
    #[serde(rename = "round")]
    Round,
    #[serde(rename = "square")]
    Square,
}

impl Default for Cap {
    fn default() -> Self {
        Cap::Butt
    }
}

#[derive(Deserialize, Debug)]
pub enum Join {
    #[serde(rename = "bevel")]
    Bevel,
    #[serde(rename = "round")]
    Round,
    #[serde(rename = "miter")]
    Miter,
}

impl Default for Join {
    fn default() -> Self {
        Join::Miter
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct LineLayerLayout {
    #[serde(rename = "line-cap", default = "default_line_cap")]
    pub cap: Expression,
    #[serde(default, rename = "line-join")]
    pub join: Join,
    #[serde(rename = "line-miter-limit", default = "default_miter_limit")]
    pub miter_limit: Expression,
    #[serde(rename = "line-round-limit", default = "default_round_limit")]
    pub round_limit: Expression,
    #[serde(rename = "line-sort-key")]
    pub sort_key: Option<f32>,
    #[serde(default, rename = "visibility")]
    pub visibility: Visibility,
}

fn default_blur() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#000000").unwrap()))
}

fn default_gap_width() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_offset() -> Expression {
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

fn default_width() -> Expression {
    Expression::Type(Type::Number(1.0))
}

fn default_line_cap() -> Expression {
    Expression::LineCap(Cap::Butt)
}

fn default_miter_limit() -> Expression {
    Expression::Type(Type::Number(2.0))
}

fn default_round_limit() -> Expression {
    Expression::Type(Type::Number(1.05))
}
