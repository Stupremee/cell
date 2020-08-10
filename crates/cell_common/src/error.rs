//! Contains all different error kinds / types and some utilities.

use crate::{
    source::{FileId, Locatable, Span},
    Diagnostic,
};
use derive_more::{Display, Error, From};

pub type ParseResult<T> = std::result::Result<T, Locatable<SyntaxError>>;

/// The central error enum which can be **any** error generated
/// by the compiler.
#[derive(Debug, Clone, From, Display, Error)]
pub enum Error {
    #[display(fmt = "syntax: {}", _0)]
    Syntax(SyntaxError),
}

impl IntoDiagnostic for Error {
    fn into_diagnostic(self, span: Span, file: FileId) -> Diagnostic {
        match self {
            Error::Syntax(err) => err.into_diagnostic(span, file),
        }
    }
}

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

/// Any error that can happen during parsing.
#[derive(Debug, Clone, Display, Error)]
pub enum SyntaxError {}

impl IntoDiagnostic for SyntaxError {
    fn into_diagnostic(self, span: Span, file: FileId) -> Diagnostic {
        todo!()
    }
}

impl<T: IntoDiagnostic> Into<Diagnostic> for Locatable<T> {
    fn into(self) -> Diagnostic {
        let (data, file, span) = self.destruct();
        data.into_diagnostic(span, file)
    }
}

/// A collection of errors and / or warnings.
#[derive(Default)]
pub struct ErrorHandler {
    errors: Vec<Locatable<Error>>,
    warnings: Vec<Diagnostic>,
}

impl ErrorHandler {
    /// Appends an error to the list of errors.
    pub fn error(&mut self, error: impl Into<Locatable<Error>>) {
        self.errors.push(error.into())
    }

    /// Appends a warning to the list of errors.
    pub fn warning(&mut self, warn: impl Into<Diagnostic>) {
        self.warnings.push(warn.into())
    }

    /// Takes all errors out of this error handler and returns them.
    pub fn take_errors(&mut self) -> Vec<Locatable<Error>> {
        std::mem::take(&mut self.errors)
    }

    /// Takes all warnings out of this error handler and returns them.
    pub fn take_warnings(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.warnings)
    }
}
