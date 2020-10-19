use super::Expression;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

#[derive(PartialEq, Debug)]
pub(crate) enum Type {
    Linear,
    Exponential { base: f64 },
    CubicBezier { x1: f64, y1: f64, x2: f64, y2: f64 },
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TypeVisitor;

        impl<'de> Visitor<'de> for TypeVisitor {
            type Value = Type;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum interpolate type")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let command: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                match command {
                    "linear" => {
                        // It seems that there are cases when we one more argument with "linear" type
                        while let Ok(Some(_value)) = seq.next_element::<Expression>() {}
                        Ok(Type::Linear)
                    },
                    "exponential" => {
                        let base: f64 = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        Ok(Type::Exponential { base })
                    }
                    "cubic-bezier" => {
                        let x1: f64 = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y1: f64 = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let x2: f64 = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y2: f64 = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        Ok(Type::CubicBezier { x1, y1, x2, y2 })
                    }
                    _ => Err(de::Error::custom(
                        "Missing supported command in the first element of array for interpolate type",
                    )),
                }
            }
        }

        deserializer.deserialize_seq(TypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolation_type_linear() {
        let data = r#"["linear"]"#;

        let t: Type = serde_json::from_str(data).expect("Expected valid linear interpolation type");
        assert_eq!(t, Type::Linear);
    }

    #[test]
    fn interpolation_type_exponential() {
        let data = r#"["exponential", 2]"#;

        let t: Type =
            serde_json::from_str(data).expect("Expected valid exponential interpolation type");
        assert_eq!(t, Type::Exponential { base: 2.0 });
    }

    #[test]
    fn interpolation_type_cubic_bezier() {
        let data = r#"["cubic-bezier", 0.42, 0, 0.58, 1]"#;

        let t: Type =
            serde_json::from_str(data).expect("Expected valid cubic-bezier interpolation type");
        assert_eq!(
            t,
            Type::CubicBezier {
                x1: 0.42,
                y1: 0.0,
                x2: 0.58,
                y2: 1.0
            }
        );
    }
}
