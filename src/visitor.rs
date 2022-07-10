use crate::{Path, Value};

pub trait Visitor {
    fn visit_any(&mut self, path: &Path, value: Value<'_>);

    fn visit_str(&mut self, path: &Path, value: &str) {
        self.visit_any(path, value.into());
    }

    fn visit_bool(&mut self, path: &Path, value: bool) {
        self.visit_any(path, value.into());
    }

    fn visit_u64(&mut self, path: &Path, value: u64) {
        self.visit_any(path, value.into());
    }

    fn visit_i64(&mut self, path: &Path, value: i64) {
        self.visit_any(path, value.into());
    }

    fn visit_f32(&mut self, path: &Path, value: f32) {
        self.visit_any(path, value.into());
    }

    fn visit_f64(&mut self, path: &Path, value: f64) {
        self.visit_any(path, value.into());
    }
}

impl<V: Visitor> Visitor for &'_ mut V {
    fn visit_any(&mut self, path: &Path, value: Value<'_>) {
        (*self).visit_any(path, value)
    }

    fn visit_str(&mut self, path: &Path, value: &str) {
        (*self).visit_str(path, value.into());
    }

    fn visit_bool(&mut self, path: &Path, value: bool) {
        (*self).visit_bool(path, value.into());
    }

    fn visit_u64(&mut self, path: &Path, value: u64) {
        (*self).visit_u64(path, value.into());
    }

    fn visit_i64(&mut self, path: &Path, value: i64) {
        (*self).visit_i64(path, value.into());
    }

    fn visit_f32(&mut self, path: &Path, value: f32) {
        (*self).visit_f32(path, value.into());
    }

    fn visit_f64(&mut self, path: &Path, value: f64) {
        (*self).visit_f64(path, value.into());
    }
}
