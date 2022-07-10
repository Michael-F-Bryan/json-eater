use std::fmt::{self, Display, Formatter};

#[derive(
    Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub enum Value<'a> {
    String(&'a str),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Boolean(bool),
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(s: &'a str) -> Self { Value::String(s) }
}

impl From<u64> for Value<'_> {
    fn from(v: u64) -> Self { Value::U64(v) }
}

impl From<i64> for Value<'_> {
    fn from(v: i64) -> Self { Value::I64(v) }
}

impl From<f32> for Value<'_> {
    fn from(v: f32) -> Self { Value::F32(v) }
}

impl From<f64> for Value<'_> {
    fn from(v: f64) -> Self { Value::F64(v) }
}

impl From<bool> for Value<'_> {
    fn from(v: bool) -> Self { Value::Boolean(v) }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(value) => Display::fmt(value, f),
            Value::U64(value) => Display::fmt(value, f),
            Value::I64(value) => Display::fmt(value, f),
            Value::F32(value) => Display::fmt(value, f),
            Value::F64(value) => Display::fmt(value, f),
            Value::Boolean(value) => Display::fmt(value, f),
        }
    }
}
