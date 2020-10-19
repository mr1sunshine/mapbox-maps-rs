use super::Expression;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) enum Decision {
    Negation(Box<Expression>),
    NotEqual {
        left: Box<Expression>,
        right: Box<Expression>,
        collator: Option<Box<Expression>>,
    },
    Less {
        left: Box<Expression>,
        right: Box<Expression>,
        collator: Option<Box<Expression>>,
    },
    LessOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
        collator: Option<Box<Expression>>,
    },
    Equal {
        left: Box<Expression>,
        right: Box<Expression>,
        collator: Option<Box<Expression>>,
    },
    Greater {
        left: Box<Expression>,
        right: Box<Expression>,
        collator: Option<Box<Expression>>,
    },
    GreaterOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
        collator: Option<Box<Expression>>,
    },
    All {
        values: Vec<Expression>,
    },
    Case {
        arms: Vec<(Expression, Expression)>,
        fallback: Box<Expression>,
    },
    Coalesce {
        outputs: Vec<Expression>,
    },
    Match {
        input: Box<Expression>,
        labels: Vec<(Expression, Expression)>,
        fallback: Box<Expression>,
    },
}

impl<'de> Deserialize<'de> for Decision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DecisionVisitor;

        impl<'de> Visitor<'de> for DecisionVisitor {
            type Value = Decision;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum decision")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let cmd: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                match cmd {
                    "!" => {
                        let op: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        Ok(Decision::Negation(Box::new(op)))
                    }
                    "!=" => {
                        let left: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let right: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let collator: Option<Box<Expression>> = match seq.next_element() {
                            Ok(Some(e)) => Some(Box::new(e)),
                            _ => None,
                        };

                        Ok(Decision::NotEqual {
                            left: Box::new(left),
                            right: Box::new(right),
                            collator,
                        })
                    }
                    "<" => {
                        let left: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let right: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let collator: Option<Box<Expression>> = match seq.next_element() {
                            Ok(Some(e)) => Some(Box::new(e)),
                            _ => None,
                        };

                        Ok(Decision::Less {
                            left: Box::new(left),
                            right: Box::new(right),
                            collator,
                        })
                    }
                    "<=" => {
                        let left: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let right: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let collator: Option<Box<Expression>> = match seq.next_element() {
                            Ok(Some(e)) => Some(Box::new(e)),
                            _ => None,
                        };

                        Ok(Decision::LessOrEqual {
                            left: Box::new(left),
                            right: Box::new(right),
                            collator,
                        })
                    }
                    "==" => {
                        let left: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let right: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let collator: Option<Box<Expression>> = match seq.next_element() {
                            Ok(Some(e)) => Some(Box::new(e)),
                            _ => None,
                        };

                        Ok(Decision::Equal {
                            left: Box::new(left),
                            right: Box::new(right),
                            collator,
                        })
                    }
                    ">" => {
                        let left: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let right: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let collator: Option<Box<Expression>> = match seq.next_element() {
                            Ok(Some(e)) => Some(Box::new(e)),
                            _ => None,
                        };

                        Ok(Decision::Greater {
                            left: Box::new(left),
                            right: Box::new(right),
                            collator,
                        })
                    }
                    ">=" => {
                        let left: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let right: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let collator: Option<Box<Expression>> = match seq.next_element() {
                            Ok(Some(e)) => Some(Box::new(e)),
                            _ => None,
                        };

                        Ok(Decision::GreaterOrEqual {
                            left: Box::new(left),
                            right: Box::new(right),
                            collator,
                        })
                    }
                    "all" => {
                        let mut values = Vec::new();
                        while let Ok(Some(value)) = seq.next_element() {
                            values.push(value);
                        }
                        Ok(Decision::All { values })
                    }
                    "coalesce" => {
                        let mut outputs = Vec::new();
                        while let Ok(Some(value)) = seq.next_element() {
                            outputs.push(value);
                        }
                        Ok(Decision::Coalesce { outputs })
                    }
                    "case" => {
                        let mut arms = Vec::new();

                        while let Ok(Some(first)) = seq.next_element() {
                            if let Ok(second) = seq.next_element() {
                                match second {
                                    Some(second) => {
                                        arms.push((first, second));
                                    }
                                    None => {
                                        return Ok(Decision::Case {
                                            arms,
                                            fallback: Box::new(first),
                                        })
                                    }
                                }
                            }
                        }
                        Err(de::Error::custom(
                            "No fallback provide in the decision case",
                        ))
                    }
                    "match" => {
                        let input: Expression = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        let mut stops = Vec::new();

                        while let Ok(Some(first)) = seq.next_element() {
                            if let Ok(second) = seq.next_element() {
                                match second {
                                    Some(second) => {
                                        stops.push((first, second));
                                    }
                                    None => {
                                        return Ok(Decision::Match {
                                            input: Box::new(input),
                                            labels: stops,
                                            fallback: Box::new(first),
                                        })
                                    }
                                }
                            }
                        }
                        Err(de::Error::custom(
                            "Incorrect number of labels and fallback in the decision match",
                        ))
                    }
                    _ => Err(de::Error::custom(
                        "Missing command in the first element of array for decision enum",
                    )),
                }
            }
        }

        deserializer.deserialize_seq(DecisionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::super::lookup::Lookup;
    use super::super::Type as VariableType;
    use super::*;

    #[test]
    fn match_simple() {
        let data = r#"[
            "==",
            [
            "get",
            "class"
            ],
            "land"
        ]"#;

        let t: Decision = serde_json::from_str(data).expect("Expected valid == expression");
        // println!("{:#?}", t);
        assert_eq!(
            t,
            Decision::Equal {
                left: Box::new(Expression::Lookup(Lookup::Get {
                    property: "class".to_owned(),
                    object: None
                })),
                right: Box::new(Expression::Type(VariableType::String("land".to_owned()))),
                collator: None
            }
        );
    }

    #[test]
    fn decision_all() {
        let data = r#"[
            "all",
            [
               "==",
               [
                  "geometry-type"
               ],
               "LineString"
            ],
            [
               "==",
               [
                  "get",
                  "class"
               ],
               "land"
            ]
         ]"#;

        serde_json::from_str::<Decision>(data).expect("Expected valid all type expression");
    }

    #[test]
    fn decision_match() {
        let data = r#"[
            "match",
            [
               "get",
               "class"
            ],
            "shadow",
            "hsl(56, 59%, 22%)",
            "hsl(0, 0%, 100%)"
        ]"#;

        serde_json::from_str::<Decision>(data).expect("Expected valid Match expression");
        // println!("{:#?}", t);
        // assert_eq!(t, Type::Linear);
    }
}
