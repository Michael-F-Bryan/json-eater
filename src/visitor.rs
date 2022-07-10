use crate::{Path, Value};

/// A set of callbacks that will be triggered as the JSON document is parsed.
pub trait Visitor<'de> {
    fn visit_any(&mut self, path: &Path<'de>, value: Value<'de>);

    fn visit_str(&mut self, path: &Path<'de>, value: &'de str) {
        self.visit_any(path, value.into());
    }

    fn visit_owned_string(&mut self, path: &Path<'de>, value: String) {
        self.visit_any(path, value.into());
    }

    fn visit_null(&mut self, path: &Path<'de>) {
        self.visit_any(path, Value::Null);
    }

    fn visit_bool(&mut self, path: &Path<'de>, value: bool) {
        self.visit_any(path, value.into());
    }

    fn visit_u64(&mut self, path: &Path<'de>, value: u64) {
        self.visit_any(path, value.into());
    }

    fn visit_i64(&mut self, path: &Path<'de>, value: i64) {
        self.visit_any(path, value.into());
    }

    fn visit_f32(&mut self, path: &Path<'de>, value: f32) {
        self.visit_any(path, value.into());
    }

    fn visit_f64(&mut self, path: &Path<'de>, value: f64) {
        self.visit_any(path, value.into());
    }
}

impl<'de, V: Visitor<'de>> Visitor<'de> for &'_ mut V {
    fn visit_any(&mut self, path: &Path<'de>, value: Value<'de>) {
        (*self).visit_any(path, value)
    }

    fn visit_str(&mut self, path: &Path<'de>, value: &'de str) {
        (*self).visit_str(path, value);
    }

    fn visit_owned_string(&mut self, path: &Path<'de>, value: String) {
        (*self).visit_owned_string(path, value)
    }

    fn visit_null(&mut self, path: &Path<'de>) { (*self).visit_null(path) }

    fn visit_bool(&mut self, path: &Path<'de>, value: bool) {
        (*self).visit_bool(path, value);
    }

    fn visit_u64(&mut self, path: &Path<'de>, value: u64) {
        (*self).visit_u64(path, value);
    }

    fn visit_i64(&mut self, path: &Path<'de>, value: i64) {
        (*self).visit_i64(path, value);
    }

    fn visit_f32(&mut self, path: &Path<'de>, value: f32) {
        (*self).visit_f32(path, value);
    }

    fn visit_f64(&mut self, path: &Path<'de>, value: f64) {
        (*self).visit_f64(path, value);
    }
}
