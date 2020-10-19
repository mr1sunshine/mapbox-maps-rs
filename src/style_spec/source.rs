use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub(crate) enum Source {
    #[serde(rename = "vector")]
    Vector(Vector),
    #[serde(rename = "raster")]
    Raster(Raster),
    #[serde(rename = "raster-dem")]
    RasterDEM(RasterDEM),
    #[serde(rename = "geojson")]
    GeoJSON(GeoJSON),
    #[serde(rename = "video")]
    Video(Video),
    #[serde(rename = "image")]
    Image(Image),
}

#[derive(Deserialize, Debug)]
pub enum Scheme {
    #[serde(rename = "xyz")]
    XYZ,
    #[serde(rename = "tms")]
    TMS,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Vector {
    pub attribution: Option<String>,
    #[serde(default = "default_bounds")]
    pub bounds: Vec<f64>,
    #[serde(default = "default_maxzoom")]
    pub maxzoom: f32,
    #[serde(default = "default_minzoom")]
    pub minzoom: f32,
    #[serde(rename = "promoteId")]
    pub promote_id: Option<Value>,
    #[serde(default = "default_scheme")]
    pub scheme: Scheme,
    pub tiles: Option<Vec<String>>,
    pub url: Option<String>,
}

fn default_bounds() -> Vec<f64> {
    vec![-180.0, -85.051129, 180.0, 85.051129]
}

fn default_maxzoom() -> f32 {
    22.0
}

fn default_minzoom() -> f32 {
    0.0
}

fn default_scheme() -> Scheme {
    Scheme::XYZ
}

#[derive(Deserialize, Debug)]
pub(crate) struct Raster {
    pub attribution: Option<String>,
    #[serde(default = "default_bounds")]
    pub bounds: Vec<f64>,
    #[serde(default = "default_maxzoom")]
    pub maxzoom: f32,
    #[serde(default = "default_minzoom")]
    pub minzoom: f32,
    #[serde(default = "default_scheme")]
    pub scheme: Scheme,
    #[serde(rename = "tileSize", default = "default_tile_size")]
    pub tile_size: f32,
    pub tiles: Option<Vec<String>>,
    pub url: Option<String>,
}

fn default_tile_size() -> f32 {
    512.0
}

#[derive(Deserialize, Debug)]
pub(crate) struct RasterDEM {
    pub attribution: Option<String>,
    #[serde(default = "default_bounds")]
    pub bounds: Vec<f64>,
    #[serde(default = "default_encoding")]
    pub encoding: Encoding,
    #[serde(default = "default_maxzoom")]
    pub maxzoom: f32,
    #[serde(default = "default_minzoom")]
    pub minzoom: f32,
    #[serde(rename = "tileSize", default = "default_tile_size")]
    pub tile_size: f32,
    pub tiles: Option<Vec<String>>,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum Encoding {
    #[serde(rename = "terrarium")]
    Terrarium,
    #[serde(rename = "mapbox")]
    Mapbox,
}

fn default_encoding() -> Encoding {
    Encoding::Mapbox
}

#[derive(Deserialize, Debug)]
pub(crate) struct GeoJSON {
    pub attribution: Option<String>,
    #[serde(default = "default_buffer")]
    pub buffer: f32,
    #[serde(default = "default_cluster")]
    pub cluster: bool,
    #[serde(rename = "clusterMaxZoom")]
    pub cluster_max_zoom: Option<f32>,
    #[serde(rename = "clusterProperties")]
    pub cluster_properties: Option<Value>,
    pub data: Option<Value>,
    #[serde(rename = "generateId", default = "default_generate_id")]
    pub generate_id: bool,
    #[serde(rename = "lineMetrics", default = "default_line_metrics")]
    pub line_metrics: bool,
    #[serde(default = "default_geojson_maxzoom")]
    pub maxzoom: f32,
    #[serde(rename = "promoteId")]
    pub promote_id: Option<Value>,
    #[serde(default = "default_tolerance")]
    pub tolerance: f32,
}

fn default_buffer() -> f32 {
    128.0
}

fn default_cluster() -> bool {
    false
}

fn default_generate_id() -> bool {
    false
}

fn default_line_metrics() -> bool {
    false
}

fn default_geojson_maxzoom() -> f32 {
    18.0
}

fn default_tolerance() -> f32 {
    0.375
}

#[derive(Deserialize, Debug)]
pub(crate) struct Image {
    url: String,
    coordinates: Vec<Vec<f32>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Video {
    urls: Vec<String>,
    coordinates: Vec<Vec<f32>>,
}
