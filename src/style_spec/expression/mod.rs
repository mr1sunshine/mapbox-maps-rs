mod decision;
mod feature_data;
mod lookup;
mod math;
mod ramps_scales_curves;
mod string;
mod types;
mod zoom;

use super::types::Type;
use serde::Deserialize;

use super::layer::LineCap;
use super::layer::SymbolPlacement;
use super::layer::TextAnchor;
use super::layer::TextJustify;

#[derive(PartialEq, Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Expression {
    RampsScalesCurves(ramps_scales_curves::RampsScalesCurves),
    Zoom(zoom::Zoom),
    FeatureData(feature_data::FeatureData),
    Decision(decision::Decision),
    Types(types::Types),
    String(string::String),
    Lookup(lookup::Lookup),
    Math(math::Math),

    LineCap(LineCap),
    SymbolPlacement(SymbolPlacement),
    TextAnchor(TextAnchor),
    TextJustify(TextJustify),

    Type(Type),
    // Other(serde_json::Value),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = r#"[
            "step",
            [
                "zoom"
            ],
            "point",
            11,
            "line"
        ]"#;

        serde_json::from_str::<Expression>(data).expect("Expected valid expression");
    }
}
