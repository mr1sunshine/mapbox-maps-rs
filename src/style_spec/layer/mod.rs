mod background;
mod circle;
mod fill;
mod fill_extrusion;
mod heatmap;
mod hillshade;
mod line;
mod raster;
mod symbol;

use background::*;
use circle::*;
use fill::*;
use fill_extrusion::*;
use heatmap::*;
use hillshade::*;
use line::*;
use raster::*;
use symbol::*;

use serde::Deserialize;
use serde_json::Value;

use super::{types::Anchor, Expression};

pub(crate) use line::Cap as LineCap;
pub(crate) use symbol::SymbolPlacement;
pub(crate) use symbol::TextAnchor;
pub(crate) use symbol::TextJustify;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub(crate) enum Layer {
    Fill(Box<LayerBase<FillLayerPaint, FillLayerLayout>>),
    Line(Box<LayerBase<LineLayerPaint, LineLayerLayout>>),
    Symbol(Box<LayerBase<SymbolLayerPaint, SymbolLayerLayout>>),
    Circle(Box<LayerBase<CircleLayerPaint, CircleLayerLayout>>),
    Heatmap(Box<LayerBase<HeatmapLayerPaint, HeatmapLayerLayout>>),
    FillExtrusion(Box<LayerBase<FillExtrusionLayerPaint, FillExtrusionLayerLayout>>),
    Raster(Box<LayerBase<RasterLayerPaint, RasterLayerLayout>>),
    Hillshade(Box<LayerBase<HillshadeLayerPaint, HillshadeLayerLayout>>),
    Background(Box<LayerBase<BackgroundLayerPaint, BackgroundLayerLayout>>),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Visibility {
    Visible,
    None,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct LayerBase<P, L> {
    pub filter: Option<Expression>,
    pub id: String,
    pub maxzoom: Option<f32>,
    pub metadata: Option<Value>,
    pub minzoom: Option<f32>,
    pub source: Option<String>,
    pub source_layer: Option<String>,
    pub paint: Option<P>,
    pub layout: Option<L>,
}