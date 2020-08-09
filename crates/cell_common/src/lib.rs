//! Structures and traits that are shared around the whole cell
//! compiler.
#![warn(rust_2018_idioms)]

pub mod error;
mod macros;
pub mod profiler;
pub mod source;
pub mod strings;

use source::FileId;

/// Diagnostic type alias with specific `FileId`.
pub type Diagnostic = codespan_reporting::diagnostic::Diagnostic<FileId>;
/// Label type alias with specific `FileId`.
pub type Label = codespan_reporting::diagnostic::Label<FileId>;
