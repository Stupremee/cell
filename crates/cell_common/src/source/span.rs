//! Types for indexing a range in a source string.

use std::ops::Range;
use text_size::TextRange;

/// A index to a single byte in a string.
pub type Index = text_size::TextSize;

/// A type that points to a start and a end
/// in a source string.
#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
pub struct Span {
    range: TextRange,
}

impl Span {
    /// Creates a new `Span` from a start and an end index.
    pub fn new(start: impl Into<Index>, end: impl Into<Index>) -> Self {
        Self {
            range: TextRange::new(start.into(), end.into()),
        }
    }

    /// Returns the starting [`Index`] of `self`.
    ///
    /// [`Index`]: ./type.Index.html
    pub fn start(self) -> Index {
        self.range.start()
    }

    /// Returns the ending [`Index`] of `self`.
    ///
    /// [`Index`]: ./type.Index.html
    pub fn end(self) -> Index {
        self.range.start()
    }

    /// Returns the length of self
    pub fn len(self) -> Index {
        self.range.len()
    }

    /// Check if this span contains the given index.
    pub fn contains(self, index: Index) -> bool {
        self.range.contains(index)
    }

    /// Checks if this span contains the whole other range.
    pub fn contains_range(self, range: Span) -> bool {
        self.range.contains_range(range.range)
    }

    /// Returns the range covered by both spans, if it exists.
    /// If the ranges touch, but do not overlap, the output range is empty.
    pub fn intersect(self, other: Span) -> Option<Span> {
        Some(Span {
            range: self.range.intersect(other.range)?,
        })
    }
}

impl<T> From<Span> for Range<T>
where
    T: From<Index>,
{
    fn from(span: Span) -> Self {
        span.range.into()
    }
}
