use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;
#[derive(Debug, PartialEq)]
pub(crate) struct Zoom {}

impl<'de> Deserialize<'de> for Zoom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ZoomVisitor;

        impl<'de> Visitor<'de> for ZoomVisitor {
            type Value = Zoom;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Zoom")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let zoom: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                if zoom != "zoom" {
                    return Err(de::Error::custom(
                        "Missing zoom command in the first element of array",
                    ));
                }

                Ok(Zoom {})
            }
        }

        deserializer.deserialize_seq(ZoomVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom() {
        let data = r#"["zoom"]"#;

        let t: Zoom = serde_json::from_str(data).expect("Expected valid zoom type");
        assert_eq!(t, Zoom {});
    }
}
