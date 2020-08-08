//! Contains all different error kinds / types and some utilities.

use crate::{
    source::{FileId, Span},
    Diagnostic,
};
use derive_more::{Display, Error, From};

/// The central error enum which can be **any** error generated
/// by the compiler.
#[derive(Debug, Clone, From, Display, Error)]
pub enum Error {}

/// Trait to represent anything that can be turned into a [`Diagnostic`]
/// using a file and a span.
///
/// [`Diagnostic`]: https://docs.rs/codespan-reporting/0.9.5/codespan_reporting/diagnostic/struct.Diagnostic.html
pub trait IntoDiagnostic {
    /// Turns `self` into a [`Diagnostic`].
    ///
    /// [`Diagnostic`]: https://docs.rs/codespan-reporting/0.9.5/codespan_reporting/diagnostic/struct.Diagnostic.html
    fn into_diagnostic(self, span: Span, file: FileId) -> Diagnostic;
}
