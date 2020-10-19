use super::super::{
    types::{Color, Type},
    Expression,
};
use super::{Anchor, Visibility};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SymbolLayerPaint {
    #[serde(default = "default_icon_color")]
    pub icon_color: Expression,
    #[serde(default = "default_icon_opacity")]
    pub icon_opacity: Expression,
    #[serde(default = "default_icon_halo_color")]
    pub icon_halo_color: Expression,
    #[serde(default = "default_icon_halo_width")]
    pub icon_halo_width: Expression,
    #[serde(default = "default_icon_halo_blur")]
    pub icon_halo_blur: Expression,
    #[serde(default = "default_icon_translate")]
    pub icon_translate: Expression,
    #[serde(default = "default_icon_translate_anchor")]
    pub icon_translate_anchor: Anchor,
    #[serde(default = "default_text_opacity")]
    pub text_opacity: Expression,
    #[serde(default = "default_text_color")]
    pub text_color: Expression,
    #[serde(default = "default_text_halo_color")]
    pub text_halo_color: Expression,
    #[serde(default = "default_text_halo_width")]
    pub text_halo_width: Expression,
    #[serde(default = "default_text_halo_blur")]
    pub text_halo_blur: Expression,
    #[serde(default = "default_text_translate")]
    pub text_translate: Expression,
    #[serde(default = "default_text_translate_anchor")]
    pub text_translate_anchor: Anchor,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SymbolLayerLayout {
    #[serde(default = "default_icon_allow_overlap")]
    pub icon_allow_overlap: bool,
    #[serde(default = "default_icon_anchor")]
    pub icon_anchor: IconAnchor,
    #[serde(default = "default_icon_ignore_placement")]
    pub icon_ignore_placement: bool,
    pub icon_image: Option<Expression>,
    #[serde(default = "default_icon_keep_upright")]
    pub icon_keep_upright: bool,
    #[serde(default = "default_icon_offset")]
    pub icon_offset: Expression,
    #[serde(default = "default_icon_optional")]
    pub icon_optional: bool,
    #[serde(default = "default_icon_padding")]
    pub icon_padding: Expression,
    #[serde(default = "default_icon_pitch_alignment")]
    pub icon_pitch_alignment: Alignment,
    #[serde(default = "default_icon_rotate")]
    pub icon_rotate: Expression,
    #[serde(default = "default_icon_rotation_alignment")]
    pub icon_rotation_alignment: Alignment,
    #[serde(default = "default_icon_size")]
    pub icon_size: Expression,
    #[serde(default = "default_icon_text_fit")]
    pub icon_text_fit: IconTextFit,
    #[serde(default = "default_icon_text_fit_padding")]
    pub icon_text_fit_padding: Expression,

    #[serde(default = "default_symbol_avoid_edges")]
    pub symbol_avoid_edges: bool,
    #[serde(default = "default_symbol_placement")]
    pub symbol_placement: Expression,
    pub symbol_sort_key: Option<f32>,
    #[serde(default = "default_symbol_spacing")]
    pub symbol_spacing: Expression,
    #[serde(default = "default_symbol_z_order")]
    pub symbol_z_order: SymbolZOrder,

    #[serde(default = "default_text_allow_overlap")]
    pub text_allow_overlap: bool,
    #[serde(default = "default_text_anchor")]
    pub text_anchor: Expression,
    #[serde(default = "default_text_field")]
    pub text_field: Expression,
    #[serde(default = "default_text_font")]
    pub text_font: Expression,
    #[serde(default = "default_text_ignore_placement")]
    pub text_ignore_placement: bool,
    #[serde(default = "default_text_justify")]
    pub text_justify: Expression,
    #[serde(default = "default_text_keep_upright")]
    pub text_keep_upright: bool,
    #[serde(default = "default_text_letter_spacing")]
    pub text_letter_spacing: Expression,
    #[serde(default = "default_text_line_height")]
    pub text_line_height: Expression,
    #[serde(default = "default_text_max_angle")]
    pub text_max_angle: Expression,
    #[serde(default = "default_text_max_width")]
    pub text_max_width: Expression,
    #[serde(default = "default_text_offset")]
    pub text_offset: Expression,
    #[serde(default = "default_text_optional")]
    pub text_optional: bool,
    #[serde(default = "default_text_padding")]
    pub text_padding: Expression,
    #[serde(default = "default_text_pitch_alignment")]
    pub text_pitch_alignment: Alignment,
    #[serde(default = "default_text_radial_offset")]
    pub text_radial_offset: Expression,
    #[serde(default = "default_text_rotate")]
    pub text_rotate: Expression,
    #[serde(default = "default_text_rotation_alignment")]
    pub text_rotation_alignment: Alignment,
    #[serde(default = "default_text_size")]
    pub text_size: Expression,
    #[serde(default = "default_text_transform")]
    pub text_transform: TextTransform,
    pub text_variable_anchor: Option<Vec<TextVariableAnchor>>,
    pub text_writing_mode: Option<Vec<TextWritingMode>>,

    #[serde(default, rename = "visibility")]
    pub visibility: Visibility,
}

fn default_icon_allow_overlap() -> bool {
    false
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum IconAnchor {
    Center,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for IconAnchor {
    fn default() -> Self {
        IconAnchor::Center
    }
}

fn default_icon_anchor() -> IconAnchor {
    IconAnchor::Center
}

fn default_icon_ignore_placement() -> bool {
    false
}

fn default_icon_keep_upright() -> bool {
    false
}

fn default_icon_offset() -> Expression {
    Expression::Type(Type::Array(vec![0.0, 0.0]))
}

fn default_icon_optional() -> bool {
    false
}

fn default_icon_padding() -> Expression {
    Expression::Type(Type::Number(2.0))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Alignment {
    Map,
    Viewport,
    Auto,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Auto
    }
}

fn default_icon_pitch_alignment() -> Alignment {
    Alignment::Auto
}

fn default_icon_rotate() -> Expression {
    Expression::Type(Type::Number(2.0))
}

fn default_icon_rotation_alignment() -> Alignment {
    Alignment::Auto
}

fn default_icon_size() -> Expression {
    Expression::Type(Type::Number(1.0))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum IconTextFit {
    None,
    Width,
    Height,
    Both,
}

impl Default for IconTextFit {
    fn default() -> Self {
        IconTextFit::None
    }
}

fn default_icon_text_fit() -> IconTextFit {
    IconTextFit::None
}

fn default_icon_text_fit_padding() -> Expression {
    Expression::Type(Type::Array(vec![0.0, 0.0, 0.0, 0.0]))
}

fn default_symbol_avoid_edges() -> bool {
    false
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SymbolPlacement {
    Point,
    Line,
    LineCenter,
}

impl Default for SymbolPlacement {
    fn default() -> Self {
        SymbolPlacement::Point
    }
}

fn default_symbol_placement() -> Expression {
    Expression::SymbolPlacement(SymbolPlacement::Point)
}

fn default_symbol_spacing() -> Expression {
    Expression::Type(Type::Number(250.0))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum SymbolZOrder {
    Auto,
    ViewportY,
    Source,
}

impl Default for SymbolZOrder {
    fn default() -> Self {
        SymbolZOrder::Auto
    }
}

fn default_symbol_z_order() -> SymbolZOrder {
    SymbolZOrder::Auto
}

fn default_text_allow_overlap() -> bool {
    false
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TextAnchor {
    Center,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for TextAnchor {
    fn default() -> Self {
        TextAnchor::Center
    }
}

fn default_text_anchor() -> Expression {
    Expression::TextAnchor(TextAnchor::Center)
}

fn default_text_field() -> Expression {
    Expression::Type(Type::String("".to_owned()))
}

fn default_text_font() -> Expression {
    Expression::Type(Type::String(
        r#"["Open Sans Regular","Arial Unicode MS Regular"]"#.to_owned(),
    ))
}

fn default_text_ignore_placement() -> bool {
    false
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TextJustify {
    Auto,
    Left,
    Center,
    Right,
}

impl Default for TextJustify {
    fn default() -> Self {
        TextJustify::Center
    }
}

fn default_text_justify() -> Expression {
    Expression::TextJustify(TextJustify::Center)
}

fn default_text_keep_upright() -> bool {
    true
}

fn default_text_letter_spacing() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_text_line_height() -> Expression {
    Expression::Type(Type::Number(1.2))
}

fn default_text_max_angle() -> Expression {
    Expression::Type(Type::Number(45.0))
}

fn default_text_max_width() -> Expression {
    Expression::Type(Type::Number(10.0))
}

fn default_text_offset() -> Expression {
    Expression::Type(Type::Array(vec![0.0, 0.0]))
}

fn default_text_optional() -> bool {
    false
}

fn default_text_padding() -> Expression {
    Expression::Type(Type::Number(2.0))
}

fn default_text_pitch_alignment() -> Alignment {
    Alignment::Auto
}

fn default_text_radial_offset() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_text_rotation_alignment() -> Alignment {
    Alignment::Auto
}

fn default_text_size() -> Expression {
    Expression::Type(Type::Number(16.0))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TextTransform {
    None,
    UpperCase,
    LowerCase,
}

impl Default for TextTransform {
    fn default() -> Self {
        TextTransform::None
    }
}

fn default_text_transform() -> TextTransform {
    TextTransform::None
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TextVariableAnchor {
    Center,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for TextVariableAnchor {
    fn default() -> Self {
        TextVariableAnchor::Center
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TextWritingMode {
    Horizontal,
    Vertical,
}

impl Default for TextWritingMode {
    fn default() -> Self {
        TextWritingMode::Horizontal
    }
}

fn default_text_rotate() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_icon_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#000000").unwrap()))
}

fn default_icon_halo_blur() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_icon_halo_color() -> Expression {
    Expression::Type(Type::Color(
        Color::new_with_string("rgba(0, 0, 0, 0)").unwrap(),
    ))
}

fn default_icon_halo_width() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_icon_opacity() -> Expression {
    Expression::Type(Type::Number(1.0))
}

fn default_icon_translate() -> Expression {
    Expression::Type(Type::Array(vec![0.0, 0.0]))
}

fn default_icon_translate_anchor() -> Anchor {
    Anchor::Map
}

fn default_text_color() -> Expression {
    Expression::Type(Type::Color(Color::new_with_string("#000000").unwrap()))
}

fn default_text_halo_blur() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_text_halo_color() -> Expression {
    Expression::Type(Type::Color(
        Color::new_with_string("rgba(0, 0, 0, 0)").unwrap(),
    ))
}

fn default_text_halo_width() -> Expression {
    Expression::Type(Type::Number(0.0))
}

fn default_text_opacity() -> Expression {
    Expression::Type(Type::Array(vec![0.0, 0.0]))
}

fn default_text_translate() -> Expression {
    Expression::Type(Type::Array(vec![0.0, 0.0]))
}

fn default_text_translate_anchor() -> Anchor {
    Anchor::Map
}
