use super::super::{types::Type, Expression};
use super::Visibility;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct RasterLayerPaint {
    #[serde(rename = "raster-brightness-max", default = "default_brightness_max")]
    pub brightness_max: Expression,
    #[serde(rename = "raster-brightness-min", default = "default_brightness_min")]
    pub brightness_min: Expression,
    #[serde(rename = "raster-contrast", default = "default_contrast")]
    pub contrast: Expression,
    #[serde(rename = "raster-fade-duration", default = "default_fade_duration")]
    pub fade_duration: Expression,
    #[serde(rename = "raster-hue-rotate", default = "default_hue_rotate")]
    pub hue_rotate: Expression,
    #[serde(rename = "raster-opacity", default = "default_opacity")]
    pub opacity: Expression,
    #[serde(rename = "raster-resampling", default = "default_resampling")]
    pub resampling: Resampling,
    #[serde(rename = "raster-saturation", default = "default_saturation")]
    pub saturation: Expression,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct RasterLayerLayout {
    #[serde(default, rename = "visibility")]
    pub visibility: Visibility,
}

fn default_brightness_max() -> Expression {
    Expression::Type(Type::Number(1.0))
}

fn default_brightness_min() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_contrast() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_fade_duration() -> Expression {
    Expression::Type(Type::Number(300.0))
}

fn default_hue_rotate() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_opacity() -> Expression {
    Expression::Type(Type::Number(0.0))
}

#[derive(Deserialize, Debug)]
pub enum Resampling {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "nearest")]
    Nearest,
}

impl Default for Resampling {
    fn default() -> Self {
        Resampling::Linear
    }
}

fn default_resampling() -> Resampling {
    Resampling::Linear
}

fn default_saturation() -> Expression {
    Expression::Type(Type::Number(0.0))
}
