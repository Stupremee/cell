//! The salsa database for the whole frontend.

use crate::token::{self, Token};
use cell_common::{
    source::{FileId, SourceDatabase},
    strings::StringInterner,
};
use cell_syntax::ast::{Expr, ExprId, Item, ItemId, Stmt, StmtId, Type, TypeId};

#[salsa::query_group(ParseDatabaseStorage)]
pub trait ParseDatabase: SourceDatabase {
    #[salsa::input]
    fn rodeo(&self) -> StringInterner;

    #[salsa::interned]
    fn intern_item(&self, item: Item) -> ItemId;

    #[salsa::interned]
    fn intern_stmt(&self, stmt: Stmt) -> StmtId;

    #[salsa::interned]
    fn intern_expr(&self, expr: Expr) -> ExprId;

    #[salsa::interned]
    fn intern_type(&self, ty: Type) -> TypeId;

    /// Takes the source of the file and turns it into a sequence
    /// of tokens.
    fn lex(&self, file: FileId) -> Vec<Token>;
}

fn lex(db: &dyn ParseDatabase, file: FileId) -> Vec<Token> {
    let source = &*db.source(file);
    token::lex(source)
}
