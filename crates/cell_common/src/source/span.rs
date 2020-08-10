//! Types for indexing a range in a source string.

use crate::source::FileId;
use std::{
    fmt,
    ops::{Deref, Range},
};
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

impl Into<Span> for Range<usize> {
    fn into(self) -> Span {
        let start = self.start as u32;
        let end = self.end as u32;
        Span::new(start, end)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}..{:?}", self.start(), self.end())
    }
}

/// Represents any `T` that is located at a specific [`Span`].
///
/// [`Span`]: ./struct.Span.html
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Spanned<T> {
    span: Span,
    data: T,
}

impl<T> Spanned<T> {
    /// Creates a new `Spanned` object with the given `data` and `span`.
    pub fn new(data: T, span: Span) -> Self {
        Self { data, span }
    }

    /// Maps the data of this `Spanned` using the given mapper
    /// and returns a new `Spanned` with the mapped data.
    pub fn map<U, F: FnOnce(T) -> U>(self, map: F) -> Spanned<U> {
        let (data, span) = self.destruct();
        Spanned::new(map(data), span)
    }

    /// Returns the [`Span`] of this spanned data.
    ///
    /// [`Span`]: ./struct.Span.html
    pub fn span(&self) -> Span {
        self.span
    }

    /// Returns a reference to the data inside self.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Destructs this spanned data into it's `T` and [`Span`].
    ///
    /// [`Span`]: ./struct.Span.html
    pub fn destruct(self) -> (T, Span) {
        (self.data, self.span)
    }
}

/// Represents any `T` that is located at a specific [`Span`]
/// in a specific file.
///
/// [`Span`]: ./struct.Span.html
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Locatable<T> {
    span: Span,
    data: T,
    file: FileId,
}

impl<T> Locatable<T> {
    /// Creates a new `Spanned` object with the given `data` and `span`.
    pub fn new(data: T, file: FileId, span: Span) -> Self {
        Self { data, file, span }
    }

    /// Maps the data of this `Locatable` using the given mapper
    /// and returns a new `Locatable` with the mapped data.
    pub fn map<U, F: FnOnce(T) -> U>(self, map: F) -> Locatable<U> {
        let (data, file, span) = self.destruct();
        Locatable::new(map(data), file, span)
    }

    /// Returns the [`Span`] of this spanned data.
    ///
    /// [`Span`]: ./struct.Span.html
    pub fn span(&self) -> Span {
        self.span
    }

    /// Returns a reference to the data inside self.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Returns the [`FileId`] of this spanned data.
    ///
    /// [`FileId`]: ../source/struct.FileId.html
    pub fn file(&self) -> FileId {
        self.file
    }

    /// Destructs this spanned data into it's `T`, [`FileId`] and [`Span`].
    ///
    /// [`Span`]: ./struct.Span.html
    /// [`FileId`]: ../source/struct.FileId.html
    pub fn destruct(self) -> (T, FileId, Span) {
        (self.data, self.file, self.span)
    }
}
