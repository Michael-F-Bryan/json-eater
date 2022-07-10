use std::io::Write;

use crate::{Path, Value, Visitor};

/// A [`Visitor`] implementation which will write each [`Path`]-[`Value`] pair
/// to a CSV file.
pub struct CsvWriter<W>(W);

impl<W> CsvWriter<W> {
    pub const fn new(writer: W) -> Self { CsvWriter(writer) }

    pub fn into_inner(self) -> W { self.0 }

    pub fn inner_mut(&mut self) -> &mut W { &mut self.0 }
}

impl<W: Write> Visitor<'_> for CsvWriter<W> {
    fn visit_any(&mut self, path: &Path<'_>, value: Value<'_>) {
        let _ = writeln!(self.0, "{}, {}", path, value);
    }
}
