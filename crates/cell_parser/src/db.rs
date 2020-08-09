//! The salsa database for the whole frontend.

use crate::token::{self, Token};
use cell_common::source::{FileId, SourceDatabase};

#[salsa::query_group(ParseDatabaseStorage)]
pub trait ParseDatabase: SourceDatabase {
    /// Takes the source of the file and turns it into a sequence
    /// of tokens.
    fn lex(&self, file: FileId) -> Vec<Token>;
}

fn lex(db: &dyn ParseDatabase, file: FileId) -> Vec<Token> {
    let source = &*db.source(file);
    token::lex(source)
}
