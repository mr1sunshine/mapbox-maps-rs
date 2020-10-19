mod color;

use super::{Layer, Source};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

type Expression = Value;

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Style {
    pub bearing: Option<u32>,

    pub center: Option<Vec<f32>>,

    pub glyphs: Option<String>,

    pub layers: Vec<Layer>,

    pub light: Option<Light>,

    pub metadata: Value,

    pub name: Option<String>,

    pub pitch: Option<u32>,

    pub sources: HashMap<String, Source>,

    pub sprite: Option<String>,

    pub transition: Option<Transition>,

    pub version: u8,

    pub zoom: Option<f32>,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Transition {
    #[serde(default = "default_transition_duration")]
    pub duration: f32,
    #[serde(default = "default_transition_delay")]
    pub delay: f32,
}

fn default_transition_duration() -> f32 {
    0.0
}

fn default_transition_delay() -> f32 {
    300.0
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Light {
    #[serde(default)]
    anchor: Anchor,
    #[serde(default = "default_light_color")]
    color: Expression,
    #[serde(default = "default_light_intensity")]
    intensity: Expression,
    #[serde(default = "default_light_position")]
    position: Expression,
}

fn default_light_color() -> Expression {
    json!("#ffffff")
}

fn default_light_intensity() -> Expression {
    json!(0.5)
}

fn default_light_position() -> Expression {
    json!(["1.15", "210", "30"])
}

#[derive(Deserialize, Debug)]
pub enum Anchor {
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "viewport")]
    Viewport,
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::Viewport
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub(crate) enum Type {
    Color(color::Color),
    String(String),
    Formatted(String),
    ResolvedImage(String),
    Boolean(bool),
    Number(f64),
    Array(Vec<f64>),
    StringArray(Vec<String>),
}

pub(crate) use color::Color;
