use super::Expression;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) enum String {
    Concat { values: Vec<Expression> },
}

impl<'de> Deserialize<'de> for String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringVisitor;

        impl<'de> Visitor<'de> for StringVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum String")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let first: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                match first {
                    "concat" => {
                        let mut values = Vec::new();
                        while let Ok(Some(value)) = seq.next_element() {
                            values.push(value);
                        }
                        Ok(String::Concat { values })
                    }
                    _ => Err(de::Error::custom(
                        "Missing command in the first element of array for String enum",
                    )),
                }
            }
        }

        deserializer.deserialize_seq(StringVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::super::lookup::Lookup;
    use super::super::types::Types;
    use super::super::Type;
    use super::*;

    #[test]
    fn string_concat() {
        let data = r#"[
            "concat",
            [
               "get",
               "shield"
            ],
            "-",
            [
               "to-string",
               [
                  "get",
                  "reflen"
               ]
            ]
         ]"#;

        let t = serde_json::from_str::<String>(data).expect("Expected String concat");
        assert_eq!(
            t,
            String::Concat {
                values: vec![
                    Expression::Lookup(Lookup::Get {
                        property: "shield".to_owned(),
                        object: None
                    }),
                    Expression::Type(Type::String("-".to_owned())),
                    Expression::Types(Types::ToString(Box::new(Expression::Lookup(Lookup::Get {
                        property: "reflen".to_owned(),
                        object: None
                    }))))
                ]
            }
        );
    }
}
