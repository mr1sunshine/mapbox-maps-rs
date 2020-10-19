use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) enum Lookup {
    Has {
        property: String,
        object: Option<String>,
    },
    Get {
        property: String,
        object: Option<String>,
    },
}

impl<'de> Deserialize<'de> for Lookup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LookupVisitor;

        impl<'de> Visitor<'de> for LookupVisitor {
            type Value = Lookup;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum Lookup")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let first: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                match first {
                    "get" => {
                        let property: &str = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let object: Option<String> = if let Ok(Some(object)) = seq.next_element() {
                            Some(object)
                        } else {
                            None
                        };

                        Ok(Lookup::Get {
                            property: property.to_owned(),
                            object,
                        })
                    }
                    "has" => {
                        let property: &str = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let object: Option<String> = if let Ok(Some(object)) = seq.next_element() {
                            Some(object)
                        } else {
                            None
                        };

                        Ok(Lookup::Has {
                            property: property.to_owned(),
                            object,
                        })
                    }
                    _ => Err(de::Error::custom(
                        "Missing get command in the first element of array",
                    )),
                }
            }
        }

        deserializer.deserialize_seq(LookupVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_property() {
        let data = r#"["get", "x"]"#;

        let t = serde_json::from_str::<Lookup>(data).expect("Expected valid Lookup::Get enum");
        assert_eq!(
            t,
            Lookup::Get {
                property: "x".to_owned(),
                object: None
            }
        );
    }

    #[test]
    fn get_property_object() {
        let data = r#"["get", "x", "my_object"]"#;

        let t = serde_json::from_str::<Lookup>(data).expect("Expected valid Lookup::Get enum");
        assert_eq!(
            t,
            Lookup::Get {
                property: "x".to_owned(),
                object: Some("my_object".to_owned())
            }
        );
    }

    #[test]
    fn has_property() {
        let data = r#"["has", "x"]"#;

        let t = serde_json::from_str::<Lookup>(data).expect("Expected valid Lookup::Has enum");
        assert_eq!(
            t,
            Lookup::Has {
                property: "x".to_owned(),
                object: None
            }
        );
    }

    #[test]
    fn has_property_object() {
        let data = r#"["has", "x", "my_object"]"#;

        let t = serde_json::from_str::<Lookup>(data).expect("Expected valid Lookup::Has enum");
        assert_eq!(
            t,
            Lookup::Has {
                property: "x".to_owned(),
                object: Some("my_object".to_owned())
            }
        );
    }
}
