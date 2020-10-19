use super::Expression;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) enum Math {
    Plus { values: Vec<Expression> },
}

impl<'de> Deserialize<'de> for Math {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MathVisitor;

        impl<'de> Visitor<'de> for MathVisitor {
            type Value = Math;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum Math")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let cmd: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                match cmd {
                    "+" => {
                        let mut values = Vec::new();
                        while let Ok(Some(value)) = seq.next_element() {
                            values.push(value);
                        }
                        Ok(Math::Plus { values })
                    }
                    _ => Err(de::Error::custom(
                        "Missing command in the first element of array for Math enum",
                    )),
                }
            }
        }

        deserializer.deserialize_seq(MathVisitor)
    }
}

#[cfg(test)]
mod tests {}
