use super::super::{types::Type, Expression};
use super::Visibility;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct HeatmapLayerPaint {
    #[serde(rename = "heatmap-color", default = "default_color")]
    pub color: Expression,
    #[serde(rename = "heatmap-intensity", default = "default_intensity")]
    pub intensity: Expression,
    #[serde(rename = "heatmap-opacity", default = "default_opacity")]
    pub opacity: Expression,
    #[serde(rename = "heatmap-radius", default = "default_radius")]
    pub radius: Expression,
    #[serde(rename = "heatmap-weight", default = "default_weight")]
    pub weight: Expression,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct HeatmapLayerLayout {
    #[serde(default, rename = "visibility")]
    pub visibility: Visibility,
}

fn default_color() -> Expression {
    let data = r#"["interpolate",["linear"],["heatmap-density"],0,"rgba(0, 0, 255, 0)",0.1,"royalblue",0.3,"cyan",0.5,"lime",0.7,"yellow",1,"red"]"#;
    serde_json::from_str(data).expect("Expected valid default color for Heatmap layer")
}

fn default_intensity() -> Expression {
    Expression::Type(Type::Number(1.0))
}

fn default_opacity() -> Expression {
    Expression::Type(Type::Number(1.0))
}

fn default_radius() -> Expression {
    Expression::Type(Type::Number(30.0))
}

fn default_weight() -> Expression {
    Expression::Type(Type::Number(1.0))
}
