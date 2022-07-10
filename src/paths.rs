use std::{
    fmt::{self, Display, Formatter},
    sync::Arc,
};

use im::Vector;

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
pub struct Path(pub Vector<Segment>);

impl Path {
    pub(crate) fn push(&mut self, segment: Segment) {
        self.0.push_back(segment);
    }

    pub(crate) fn pop(&mut self) { self.0.pop_back(); }
}

impl Display for Path {
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
pub enum Segment {
    String(Arc<str>),
    Index(usize),
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Segment::String(s) => Display::fmt(s, f),
            Segment::Index(i) => Display::fmt(i, f),
        }
    }
}

impl From<usize> for Segment {
    fn from(i: usize) -> Self { Segment::Index(i) }
}
