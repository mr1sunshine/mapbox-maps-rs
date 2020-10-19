use super::super::{
    types::{Color, Type},
    Expression,
};
use super::Visibility;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct BackgroundLayerPaint {
    #[serde(rename = "background-color", default = "default_background_color")]
    pub color: Expression,
    #[serde(rename = "background-opacity", default = "default_background_opacity")]
    pub opacity: Expression,
    #[serde(rename = "background-pattern")]
    pub pattern: Option<Expression>,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct BackgroundLayerLayout {
    #[serde(default, rename = "visibility")]
    pub visibility: Visibility,
}

fn default_background_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#000000").unwrap()))
}

fn default_background_opacity() -> Expression {
    Expression::Type(Type::Number(1.0))
}
