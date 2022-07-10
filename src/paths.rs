use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

use im::Vector;

/// The path to an element.
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Path<'a>(pub Vector<Segment<'a>>);

impl<'a> Path<'a> {
    pub fn segments(&self) -> impl Iterator<Item = &'_ Segment<'a>> + '_ {
        self.0.iter()
    }

    pub(crate) fn push(&mut self, segment: Segment<'a>) {
        self.0.push_back(segment);
    }

    pub(crate) fn pop(&mut self) { self.0.pop_back(); }
}

impl Display for Path<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, segment) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "/")?;
            }

            write!(f, "{segment}")?;
        }

        Ok(())
    }
}

/// A particular segment from a [`Path`], typically the name of a field or the
/// index into an array.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(untagged)]
pub enum Segment<'a> {
    String(Cow<'a, str>),
    Index(usize),
}

impl Display for Segment<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Segment::String(s) => Display::fmt(s, f),
            Segment::Index(i) => Display::fmt(i, f),
        }
    }
}

impl From<usize> for Segment<'_> {
    fn from(i: usize) -> Self { Segment::Index(i) }
}
