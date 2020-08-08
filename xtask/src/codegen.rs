//! Code generation for the AST, Syntax Kinds and Syntax Nodes.

mod syntax;

pub use syntax::generate_syntax;

const SYNTAX_KINDS: &str = "crates/cell_syntax/src/syntax_kind/generated.rs";
const AST_NODES: &str = "crates/cell_syntax/src/ast/generated/nodes.rs";
const AST_TOKENS: &str = "crates/cell_syntax/src/ast/generated/tokens.rs";
