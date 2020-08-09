//! Utilities for string interning.

use lasso::{Spur, ThreadedRodeo};
use std::sync::Arc;

/// The key for an interned string.
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct StringId {
    spur: Spur,
}

impl From<Spur> for StringId {
    fn from(spur: Spur) -> Self {
        Self { spur }
    }
}

/// A string interner that can intern strings quickly
/// without much memory overhead.
///
/// The `StringInterner` uses an `Arc<ThreadedRodeo>` and thus
/// can be cloned to create more references.
#[derive(Debug, Clone)]
pub struct StringInterner {
    rodeo: Arc<ThreadedRodeo>,
}

impl StringInterner {
    /// Creates a new string interner.
    pub fn new() -> Self {
        Self {
            rodeo: Arc::new(ThreadedRodeo::new()),
        }
    }

    /// Gets the key for the given val or interns it.
    pub fn get_or_intern(&self, val: impl AsRef<str>) -> StringId {
        self.rodeo.get_or_intern(val).into()
    }

    /// Tries to find the string for the given key. Returns `None` if
    /// the key is out of bounds.
    pub fn try_resolve<'s>(&'s self, key: &StringId) -> Option<&'s str> {
        self.rodeo.try_resolve(&key.spur)
    }

    /// Tries to find the string for the given key. Panics if
    /// the key is out of bounds.
    pub fn resolve<'s>(&'s self, key: &StringId) -> &'s str {
        self.rodeo.resolve(&key.spur)
    }
}
