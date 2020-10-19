use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;
#[derive(PartialEq, Debug)]
pub(crate) enum FeatureData {
    Accumulated,
    FeatureState(String),
    GeometryType,
    Id,
    LineProgress,
    Properties,
}

impl<'de> Deserialize<'de> for FeatureData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureDataVisitor;

        impl<'de> Visitor<'de> for FeatureDataVisitor {
            type Value = FeatureData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Feature Data")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let feature_data: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                match feature_data {
                    "accumulated" => Ok(FeatureData::Accumulated),
                    "feature-state" => {
                        let feature_state: &str = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        Ok(FeatureData::FeatureState(feature_state.to_owned()))
                    }
                    "geometry-type" => Ok(FeatureData::GeometryType),
                    "id" => Ok(FeatureData::Id),
                    "line-progress" => Ok(FeatureData::LineProgress),
                    "properties" => Ok(FeatureData::Properties),
                    _ => Err(de::Error::custom(
                        "Missing feature-data command in the first element of array",
                    )),
                }
            }
        }

        deserializer.deserialize_seq(FeatureDataVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accumulated() {
        let data = r#"["accumulated"]"#;

        let t: FeatureData = serde_json::from_str(data).expect("Expected valid Match expression");
        assert_eq!(t, FeatureData::Accumulated);
    }

    #[test]
    fn feature_state() {
        let data = r#"["feature-state", "my_string"]"#;

        let t: FeatureData = serde_json::from_str(data).expect("Expected valid Match expression");
        assert_eq!(t, FeatureData::FeatureState("my_string".to_owned()));
    }

    #[test]
    fn geometry_type() {
        let data = r#"["geometry-type"]"#;

        let t: FeatureData = serde_json::from_str(data).expect("Expected valid Match expression");
        assert_eq!(t, FeatureData::GeometryType);
    }

    #[test]
    fn id() {
        let data = r#"["id"]"#;

        let t: FeatureData = serde_json::from_str(data).expect("Expected valid Match expression");
        assert_eq!(t, FeatureData::Id);
    }

    #[test]
    fn line_progress() {
        let data = r#"["line-progress"]"#;

        let t: FeatureData = serde_json::from_str(data).expect("Expected valid Match expression");
        assert_eq!(t, FeatureData::LineProgress);
    }

    #[test]
    fn properties() {
        let data = r#"["properties"]"#;

        let t: FeatureData = serde_json::from_str(data).expect("Expected valid Match expression");
        assert_eq!(t, FeatureData::Properties);
    }
}
