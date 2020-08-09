use cell_common::{profiler, source::Span};
use logos::Logos;
use std::fmt;

#[derive(Logos, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Kind {
    #[regex(r"[\p{XID_Start}][\p{XID_Continue}]*")]
    Identifier,

    #[regex(r"[0-9][0-9_]*([e|E][+|-]?[0-9][0-9_]*)")]
    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*([e|E][+|-]?[0-9][0-9_]*)?")]
    Float,
    #[regex(r"[0-9][0-9_]*")]
    #[regex(r"0b[0-1][0-1_]*")]
    #[regex(r"0x[0-9a-fA-F][0-9a-fA-F_]*")]
    Integer,
    #[token("true")]
    #[token("false")]
    Bool,
    #[regex(r"'[^']'")]
    Char,
    #[regex(r#"(\\.|[^\\"])*"#)]
    String,

    #[token("var")]
    Var,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("type")]
    Type,
    #[token("def")]
    Def,
    #[token("match")]
    Match,
    #[token("while")]
    While,
    #[token("union")]
    Union,
    #[token("impl")]
    Impl,
    #[token("alias")]
    Alias,

    #[token("&&")]
    Ampersand2,
    #[token("&")]
    Ampersand,
    #[token("||")]
    Pipe2,
    #[token("|")]
    Pipe,
    #[token("^")]
    Caret,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("**")]
    StarStar,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token("!")]
    Bang,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanEqual,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("->")]
    ThinArrow,

    #[regex(r"//[^\n]*")]
    Comment,
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Kind::Identifier => "identifier",
            Kind::Float => "float",
            Kind::Integer => "integer",
            Kind::Bool => "bool",
            Kind::Char => "char",
            Kind::String => "string",
            Kind::Var => "var",
            Kind::If => "if",
            Kind::Else => "else",
            Kind::Type => "type",
            Kind::Def => "def",
            Kind::Match => "match",
            Kind::While => "while",
            Kind::Union => "union",
            Kind::Impl => "impl",
            Kind::Alias => "alias",
            Kind::Ampersand2 => "&&",
            Kind::Ampersand => "&",
            Kind::Pipe2 => "||",
            Kind::Pipe => "|",
            Kind::Caret => "^",
            Kind::LeftShift => "<<",
            Kind::RightShift => ">>",
            Kind::Plus => "+",
            Kind::Minus => "-",
            Kind::Star => "*",
            Kind::Slash => "/",
            Kind::Percent => "%",
            Kind::StarStar => "**",
            Kind::Equal => "=",
            Kind::EqualEqual => "==",
            Kind::Bang => "!",
            Kind::NotEqual => "!=",
            Kind::LessThan => "<",
            Kind::LessThanEqual => "<=",
            Kind::GreaterThan => ">",
            Kind::GreaterThanEqual => ">=",
            Kind::Dot => ".",
            Kind::Comma => ",",
            Kind::Colon => ":",
            Kind::Semicolon => ";",
            Kind::LeftParen => "(",
            Kind::RightParen => ")",
            Kind::LeftCurly => "{",
            Kind::RightCurly => "}",
            Kind::LeftBracket => "[",
            Kind::RightBracket => "]",
            Kind::ThinArrow => "->",
            Kind::Comment => "comment",
            Kind::Error => "error",
        };
        write!(f, "{}", repr)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Token {
    pub kind: Kind,
    pub span: Span,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}|{})", self.kind, self.span,)
    }
}

/// Takes an input and converts it into a sequence of tokens.
pub fn lex(input: impl AsRef<str>) -> Vec<Token> {
    let _profiler = profiler::trace("Lex", "Lex");

    Kind::lexer(input.as_ref())
        .spanned()
        .map(|(kind, span)| Token {
            kind,
            span: span.into(),
        })
        .collect()
}
