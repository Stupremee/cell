//! The Abstract Syntax Tree

use cell_common::{
    intern_id_struct,
    source::{Span, Spanned},
    strings::StringId,
};
use ordered_float::NotNan;

intern_id_struct! {
    /// An identifier to intern an `Item`.
    pub struct ItemId;
    /// An identifier to intern a `Stmt`.
    pub struct StmtId;
    /// An identifier to intern an `Expr`.
    pub struct ExprId;
    /// An identifier to intern a `Type`.
    pub struct TypeId;
}

/// Any type that the user can choose as a function
/// argument ot type field.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    Int { width: u16, signed: bool },
    Float { width: u16 },
    Bool,
    String,
    Unit,
    Char,
    Pointer(TypeId),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Identifier {
    pub span: Span,
    pub id: StringId,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Block {
    pub span: Span,
    pub stmts: Vec<StmtId>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Item {
    pub span: Span,
    pub kind: ItemKind,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ItemKind {
    TypeDef(TypeDef),
    Def(Def),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TypeDef {
    // TODO: Generics
    pub name: Identifier,
    pub fields: Vec<(Identifier, TypeId)>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Def {
    pub name: Identifier,
    pub args: Vec<(Identifier, TypeId)>,
    pub return_ty: TypeId,
    pub stmts: Vec<StmtId>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Stmt {
    pub span: Span,
    pub kind: StmtKind,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum StmtKind {
    Var(Var),
    Expr(ExprId),
}

/// A let / var stmt.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Var {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub val: ExprId,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Expr {
    pub span: Span,
    pub kind: ExprKind,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ExprKind {
    Literal(Literal),
    Binary(Binary),
    Comparison(Comparison),
    Unary(Unary),
    Tuple(Tuple),
    NamedTuple(NamedTuple),
    If(If),
    Match(Match),
    While(While),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct While {
    pub cond: Option<ExprId>,
    pub block: Block,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Match {
    pub val: ExprId,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Block,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MatchPattern {
    // TODO: Some real patterns here
    Literal(Literal),
    Wildcard,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct If {
    pub cond: ExprId,
    pub then: Block,
    pub arms: Vec<IfArm>,
    pub else_: Block,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IfArm {
    pub cond: ExprId,
    pub action: Block,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NamedTuple {
    pub values: Vec<(Identifier, ExprId)>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Tuple {
    pub values: Vec<ExprId>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Binary {
    pub left: ExprId,
    pub op: Spanned<BinOp>,
    pub right: ExprId,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Comparison {
    pub left: ExprId,
    pub op: Spanned<CmpOp>,
    pub right: ExprId,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Unary {
    pub op: Spanned<UnOp>,
    pub val: ExprId,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum BinOp {
    LogicalAnd,
    LogicalOr,
    BitiwseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum CmpOp {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum UnOp {
    Ref,
    Deref,
    Not,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Literal {
    Int(Int),
    Float(Float),
    Bool(bool),
    String(String),
    Char(char),
    Unit,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Int {
    pub signed: bool,
    pub val: u128,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Float {
    pub val: NotNan<f64>,
}
