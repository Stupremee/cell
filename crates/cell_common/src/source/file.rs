//! Source File specific implementations

use super::SourceDatabase;
use crate::intern_id_struct;
use codespan_reporting::files::Files;
use smol_str::SmolStr;
use std::{cmp::Ordering, ops::Range, string::String, sync::Arc};

intern_id_struct! {
    /// An id used to intern a [`File`].
    ///
    /// [`File`]: ./struct.File.html
    pub struct FileId;
}

/// A file that can be interned by a [`SourceDatabase`].
///
/// A file is composed of a name and a source string.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct File {
    name: Arc<SmolStr>,
    source: Arc<String>,
}

impl File {
    /// Creates a new `File`.
    pub fn new(name: impl Into<SmolStr>, source: impl Into<String>) -> Self {
        Self {
            name: Arc::new(name.into()),
            source: Arc::new(source.into()),
        }
    }

    /// Returns a reference to the `name` of this file.
    pub fn name(&self) -> Arc<SmolStr> {
        Arc::clone(&self.name)
    }

    /// Returns a reference to the `source` of this file.
    pub fn source(&self) -> Arc<String> {
        Arc::clone(&self.source)
    }
}

pub fn line_starts(db: &dyn SourceDatabase, file: FileId) -> Arc<Vec<usize>> {
    let starts = codespan_reporting::files::line_starts(&db.source(file)).collect();
    Arc::new(starts)
}

pub fn line_start(db: &dyn SourceDatabase, file: FileId, line_index: usize) -> Option<usize> {
    let len = db.line_starts(file).len();
    match line_index.cmp(&len) {
        Ordering::Less => db.line_starts(file).get(line_index).copied(),
        Ordering::Equal => Some(db.source(file).len()),
        Ordering::Greater => None,
    }
}

pub fn line_index(db: &dyn SourceDatabase, file: FileId, byte_index: usize) -> Option<usize> {
    match db.line_starts(file).binary_search(&byte_index) {
        Ok(line) => Some(line),
        Err(line) => Some(line - 1),
    }
}

pub fn line_range(
    db: &dyn SourceDatabase,
    file: FileId,
    line_index: usize,
) -> Option<Range<usize>> {
    let line = db.line_start(file, line_index)?;
    let next_line = db.line_start(file, line_index + 1)?;
    Some(line..next_line)
}

/// Helper struct because `Arc<String>` doesn't implement
/// `AsRef<str>`
pub struct StringRef {
    arc: Arc<String>,
}

impl AsRef<str> for StringRef {
    fn as_ref(&self) -> &str {
        self.arc.as_ref()
    }
}

/// A wrapper that can be used as a [`Files`] implementation,
/// which will use the [`SourceDatabase`] to do operations.
///
/// [`Files`]: https://docs.rs/codespan-reporting/0.9.5/codespan_reporting/files/trait.Files.html
/// [`SourceDatabase`]: ./trait.SourceDatabase.html
#[derive(Clone)]
pub struct FileCache<'db> {
    db: &'db dyn SourceDatabase,
}

impl<'db> FileCache<'db> {
    /// Creates a new `FileCache` from a source database.
    pub fn new(db: &'db dyn SourceDatabase) -> Self {
        Self { db }
    }
}

impl<'db> Files<'db> for FileCache<'db> {
    type FileId = FileId;
    type Name = Arc<SmolStr>;
    type Source = StringRef;

    fn name(&'db self, id: Self::FileId) -> Option<Self::Name> {
        Some(self.db.name(id))
    }

    fn source(&'db self, id: Self::FileId) -> Option<Self::Source> {
        let source = self.db.source(id);
        Some(StringRef { arc: source })
    }

    fn line_index(&'db self, id: Self::FileId, byte_index: usize) -> Option<usize> {
        self.db.line_index(id, byte_index)
    }

    fn line_range(&'db self, id: Self::FileId, line_index: usize) -> Option<Range<usize>> {
        self.db.line_range(id, line_index)
    }
}
