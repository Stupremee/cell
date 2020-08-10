//! The parser implementation.
//!
//! The parser is implemented using pratt parsing for
//! expressions and recursive decent for the rest.

use crate::token::Token;
use cell_common::{error::ErrorHandler, source::FileId};
use std::{iter::Peekable, vec::IntoIter};

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    errors: ErrorHandler,
    file: FileId,
}
