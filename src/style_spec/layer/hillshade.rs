use super::super::{
    types::{Color, Type},
    Expression,
};
use super::{Anchor, Visibility};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct HillshadeLayerPaint {
    #[serde(rename = "hillshade-accent-color", default = "default_accent_color")]
    pub accent_color: Expression,
    #[serde(rename = "hillshade-exaggeration", default = "default_exaggeration")]
    pub exaggeration: Expression,
    #[serde(
        rename = "hillshade-highlight-color",
        default = "default_highlight_color"
    )]
    pub highlight_color: Expression,
    #[serde(
        rename = "hillshade-illumination-anchor",
        default = "default_illumination_anchor"
    )]
    pub illumination_anchor: Anchor,
    #[serde(
        rename = "hillshade-illumination-direction",
        default = "default_illumination_direction"
    )]
    pub illumination_direction: Expression,
    #[serde(rename = "hillshade-shadow-color", default = "default_shadow_color")]
    pub shadow_color: Expression,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct HillshadeLayerLayout {
    #[serde(default, rename = "visibility")]
    pub visibility: Visibility,
}

fn default_accent_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#000000").unwrap()))
}

fn default_exaggeration() -> Expression {
    Expression::Type(Type::Number(0.5))
}

fn default_highlight_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#FFFFFF").unwrap()))
}

fn default_illumination_anchor() -> Anchor {
    Anchor::Viewport
}

fn default_illumination_direction() -> Expression {
    Expression::Type(Type::Number(335.0))
}

fn default_shadow_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#000000").unwrap()))
}
