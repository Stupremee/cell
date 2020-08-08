//! Abstractions for source code files.

mod file;
mod span;

pub use file::{File, FileCache, FileId};
pub use span::{Index, Span, Spanned};

use smol_str::SmolStr;
use std::{ops::Range, sync::Arc};

/// A database that is meant to handle every source related stuff,
/// like interning files.
#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: salsa::Database {
    /// Interns the given [`File`] and returns a [`FileId`]
    /// which can later be used to lookup a file using `lookup_intern_file`.
    #[salsa::interned]
    fn intern_file(&self, file: File) -> FileId;

    /// Looks up the given `FileId` and then returns a reference to the source of
    /// the File.
    fn source(&self, file: FileId) -> Arc<String>;

    /// Looks up the given `FileId` and then returns a reference to the name of
    /// the File.
    fn name(&self, file: FileId) -> Arc<SmolStr>;

    /// Returns the index of the line at the given byte index in the given file.
    #[salsa::invoke(file::line_index)]
    fn line_index(&self, file: FileId, byte_index: usize) -> Option<usize>;

    /// Returns the byte range of the given line in the given file.
    #[salsa::invoke(file::line_range)]
    fn line_range(&self, file: FileId, line_index: usize) -> Option<Range<usize>>;

    /// Returns the indices of every line start in the file.
    #[salsa::invoke(file::line_starts)]
    fn line_starts(&self, file: FileId) -> Arc<Vec<usize>>;

    /// Returns the start index of the line in the file.
    #[salsa::invoke(file::line_start)]
    fn line_start(&self, file: FileId, line_index: usize) -> Option<usize>;
}

fn source(db: &dyn SourceDatabase, file: FileId) -> Arc<String> {
    let file = db.lookup_intern_file(file);
    file.source()
}

fn name(db: &dyn SourceDatabase, file: FileId) -> Arc<SmolStr> {
    let file = db.lookup_intern_file(file);
    file.name()
}
