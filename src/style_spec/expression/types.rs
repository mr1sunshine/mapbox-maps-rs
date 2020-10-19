use super::Expression;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) enum Types {
    Literal(Box<Expression>),
    Number(Vec<Expression>),
    ToString(Box<Expression>),
}

impl<'de> Deserialize<'de> for Types {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TypesVisitor;

        impl<'de> Visitor<'de> for TypesVisitor {
            type Value = Types;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum Types")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let types: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                match types {
                    "literal" => {
                        let value: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        Ok(Types::Literal(Box::new(value)))
                    }
                    "number" => {
                        let mut values = Vec::new();
                        while let Ok(Some(value)) = seq.next_element() {
                            values.push(value);
                        }
                        Ok(Types::Number(values))
                    }
                    "to-string" => {
                        let value: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        Ok(Types::ToString(Box::new(value)))
                    }

                    _ => Err(de::Error::custom(
                        "Missing Types command in the first element of array",
                    )),
                }
            }
        }

        deserializer.deserialize_seq(TypesVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::super::lookup::Lookup;
    use super::*;

    #[test]
    fn types_to_string() {
        let data = r#"[
            "to-string",
            [
               "get",
               "reflen"
            ]
        ]"#;

        serde_json::from_str::<Types>(data).expect("Expected valid Types expression");
    }

    #[test]
    fn types_number() {
        let data = r#"["number", ["get", "x"]]"#;

        let t = serde_json::from_str::<Types>(data).expect("Expected valid Number type");
        assert_eq!(
            t,
            Types::Number(vec![Expression::Lookup(Lookup::Get {
                property: "x".to_owned(),
                object: None
            })])
        );
    }
}
