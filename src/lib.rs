#![doc = include_str!("../README.md")]

mod csv;
mod paths;
mod seed;
mod value;
mod visitor;

pub use crate::{
    csv::CsvWriter,
    paths::{Path, Segment},
    value::Value,
    visitor::Visitor,
};

use crate::seed::Seed;
use serde::de::DeserializeSeed;
use std::io::Read;

pub fn eat_json<'a>(
    json: &'a [u8],
    visitor: impl Visitor<'a>,
) -> Result<(), serde_json::Error> {
    let mut seed = Seed::new(visitor);
    let mut de = serde_json::Deserializer::from_slice(json);
    seed.deserialize(&mut de)?;

    Ok(())
}

pub fn eat_json_read<'a>(
    reader: impl Read,
    visitor: impl Visitor<'a>,
) -> Result<(), serde_json::Error> {
    let mut seed = Seed::new(visitor);
    let mut de = serde_json::Deserializer::from_reader(reader);
    seed.deserialize(&mut de)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, Clone, PartialEq)]
    struct Visitor {
        items: Vec<(String, String)>,
    }

    impl crate::Visitor<'_> for Visitor {
        fn visit_any(&mut self, path: &Path, value: Value<'_>) {
            self.items.push((path.to_string(), value.to_string()));
        }
    }

    #[test]
    fn consume_example_json() {
        let json = r#"{
            "name": "John",
            "isAlive": true,
            "age": -27,
            "missing": null,
            "address": {
                "streetAddress": "21 2nd Street"
            },
            "numbers": [12.32]
         }"#;
        let mut visitor = Visitor::default();
        let expected = vec![
            ("name".to_string(), "John".to_string()),
            ("isAlive".to_string(), "true".to_string()),
            ("age".to_string(), "-27".to_string()),
            ("missing".to_string(), "".to_string()),
            (
                "address/streetAddress".to_string(),
                "21 2nd Street".to_string(),
            ),
            ("numbers/0".to_string(), "12.32".to_string()),
        ];

        eat_json(json.as_bytes(), &mut visitor).unwrap();

        assert_eq!(visitor.items, expected);
    }
}
