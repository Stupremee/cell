//! Abstract syntax tree.
//!
//! Every node in the tree is just a wrapper for a
//! `SyntaxNode` with special methods to retrieve
//! specific data.

/// The main trait to convert an untyped `SyntaxNode`
/// into a typed ast.
///
/// The conversion has zero runtime cost, because every `AstNode`
/// retrieves all it's data from a `SyntaxNode`.
pub trait AstNode {}
