mod interpolate_type;

use super::Expression;
use interpolate_type::Type;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) enum RampsScalesCurves {
    Interpolate {
        interpolate_type: Type,
        input: Box<Expression>,
        stops: Vec<(Expression, Expression)>,
    },
    InterpolateHcl {
        interpolate_type: Type,
        input: Box<Expression>,
        stops: Vec<(Expression, Expression)>,
    },
    InterpolateLab {
        interpolate_type: Type,
        input: Box<Expression>,
        stops: Vec<(Expression, Expression)>,
    },
    Step {
        input: Box<Expression>,
        stop: Box<Expression>,
        stops: Vec<(Expression, Expression)>,
    },
}

impl<'de> Deserialize<'de> for RampsScalesCurves {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RampsScalesCurvesVisitor;

        impl<'de> Visitor<'de> for RampsScalesCurvesVisitor {
            type Value = RampsScalesCurves;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum RampsScalesCurves")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let cmd: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                match cmd {
                    "interpolate" => {
                        let interpolate_type: Type = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let input: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let mut stops = Vec::new();
                        while let Ok(Some(stop_input)) = seq.next_element() {
                            let stop_output: Expression = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                            stops.push((stop_input, stop_output));
                        }

                        Ok(RampsScalesCurves::Interpolate {
                            interpolate_type,
                            input: Box::new(input),
                            stops,
                        })
                    }
                    "interpolate-hcl" => {
                        let interpolate_type: Type = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let input: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let mut stops = Vec::new();
                        while let Ok(Some(stop_input)) = seq.next_element() {
                            let stop_output: Expression = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                            stops.push((stop_input, stop_output));
                        }

                        Ok(RampsScalesCurves::InterpolateHcl {
                            interpolate_type,
                            input: Box::new(input),
                            stops,
                        })
                    }
                    "interpolate-lab" => {
                        let interpolate_type: Type = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let input: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let mut stops = Vec::new();
                        while let Ok(Some(stop_input)) = seq.next_element() {
                            let stop_output: Expression = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                            stops.push((stop_input, stop_output));
                        }

                        Ok(RampsScalesCurves::InterpolateLab {
                            interpolate_type,
                            input: Box::new(input),
                            stops,
                        })
                    }
                    "step" => {
                        let input: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let stop: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let mut stops = Vec::new();
                        while let Ok(Some(stop_input)) = seq.next_element() {
                            let stop_output: Expression = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                            stops.push((stop_input, stop_output));
                        }

                        Ok(RampsScalesCurves::Step {
                            input: Box::new(input),
                            stop: Box::new(stop),
                            stops,
                        })
                    }
                    _ => Err(de::Error::custom(
                        "Missing supported command in the first element of array for RampsScalesCurves type",
                    )),
                }
            }
        }

        deserializer.deserialize_seq(RampsScalesCurvesVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Type as VariableType;
    use super::super::{lookup::Lookup, types::Types};
    use super::*;

    #[test]
    fn interpolate_linear() {
        let data = r#"[
            "interpolate",
            ["linear"],
            ["number", ["get", "x"]],
            0,
            0,
            100,
            100
        ]"#;
        let t = serde_json::from_str::<RampsScalesCurves>(data)
            .expect("Expected valid linear interpolate");
        assert_eq!(
            t,
            RampsScalesCurves::Interpolate {
                interpolate_type: Type::Linear,
                input: Box::new(Expression::Types(Types::Number(vec![Expression::Lookup(
                    Lookup::Get {
                        property: "x".to_owned(),
                        object: None
                    }
                )]))),
                stops: vec![
                    (
                        Expression::Type(VariableType::Number(0.0)),
                        Expression::Type(VariableType::Number(0.0))
                    ),
                    (
                        Expression::Type(VariableType::Number(100.0)),
                        Expression::Type(VariableType::Number(100.0))
                    )
                ],
            }
        );
    }

    #[test]
    fn interpolate_cubic_bezier() {
        let data = r#"[
            "interpolate",
            ["cubic-bezier", 0.42, 0, 0.58, 1],
            ["number", ["get", "x"]],
            0,
            0,
            100,
            100
        ]"#;
        let t = serde_json::from_str::<RampsScalesCurves>(data)
            .expect("Expected valid cubic-bezier interpolate");
        assert_eq!(
            t,
            RampsScalesCurves::Interpolate {
                interpolate_type: Type::CubicBezier {
                    x1: 0.42,
                    y1: 0.0,
                    x2: 0.58,
                    y2: 1.0,
                },
                input: Box::new(Expression::Types(Types::Number(vec![Expression::Lookup(
                    Lookup::Get {
                        property: "x".to_owned(),
                        object: None
                    }
                )]))),
                stops: vec![
                    (
                        Expression::Type(VariableType::Number(0.0)),
                        Expression::Type(VariableType::Number(0.0))
                    ),
                    (
                        Expression::Type(VariableType::Number(100.0)),
                        Expression::Type(VariableType::Number(100.0))
                    )
                ],
            }
        );
    }

    #[test]
    fn step() {
        let data = r#"[
            "step",
            [
               "zoom"
            ],
            "butt",
            11,
            "round"
         ]"#;

        serde_json::from_str::<RampsScalesCurves>(data).expect("Expected valid Step struct");
    }
}
