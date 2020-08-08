//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    EOF,
    SEMICOLON,
    COLON,
    COMMA,
    DOT,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    AMP2,
    PIPE2,
    AMP,
    PIPE,
    CARET,
    SHR,
    SHL,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    STAR2,
    EQ,
    EQ2,
    NEQ,
    L_ANGLE,
    R_ANGLE,
    LTEQ,
    GTEQ,
    BANG,
    AMPEQ,
    PIPEEQ,
    CARETEQ,
    SHREQ,
    SHLEQ,
    PLUSEQ,
    MINUSEQ,
    STAREQ,
    SLASHEQ,
    PERCENTEQ,
    THIN_ARROW,
    VAR_KW,
    IF_KW,
    ELSE_KW,
    TYPE_KW,
    DEF_KW,
    MATCH_KW,
    WHILE_KW,
    BREAK_KW,
    CONTINUE_KW,
    UNION_KW,
    TRUE_KW,
    FALSE_KW,
    INT_NUMBER,
    FLOAT_NUMBER,
    CHAR,
    STRING,
    ERROR,
    IDENT,
    WHITESPACE,
    COMMENT,
    PROGRAM,
    TYPE_DEF,
    TYPE_IMPL,
    TYPE_ALIAS,
    TYPE_TRAIT,
    TYPE_UNION,
    FN_DEF,
    PARAMETER_LIST,
    VAR_STMT,
    EXPR_STMT,
    PAREN_EXPR,
    TUPLE_EXPR,
    NAMED_TUPLE_EXPR,
    IF_EXPR,
    MATCH_EXPR,
    MATCH_ARM,
    WHILE_EXPR,
    CONTINUE,
    BREAK,
    CALL_EXPR,
    PREFIX_EXPR,
    BINARY_EXPR,
    CMP_EXPR,
    BLOCK_EXPR,
    CONDITION,
    LITERAL,
    NAME,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        match self {
            VAR_KW | IF_KW | ELSE_KW | TYPE_KW | DEF_KW | MATCH_KW | WHILE_KW | BREAK_KW
            | CONTINUE_KW | UNION_KW | TRUE_KW | FALSE_KW => true,
            _ => false,
        }
    }
    pub fn is_punct(self) -> bool {
        match self {
            SEMICOLON | COLON | COMMA | DOT | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK
            | R_BRACK | AMP2 | PIPE2 | AMP | PIPE | CARET | SHR | SHL | PLUS | MINUS | STAR
            | SLASH | PERCENT | STAR2 | EQ | EQ2 | NEQ | L_ANGLE | R_ANGLE | LTEQ | GTEQ | BANG
            | AMPEQ | PIPEEQ | CARETEQ | SHREQ | SHLEQ | PLUSEQ | MINUSEQ | STAREQ | SLASHEQ
            | PERCENTEQ | THIN_ARROW => true,
            _ => false,
        }
    }
    pub fn is_literal(self) -> bool {
        match self {
            INT_NUMBER | FLOAT_NUMBER | CHAR | STRING => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "var" => VAR_KW,
            "if" => IF_KW,
            "else" => ELSE_KW,
            "type" => TYPE_KW,
            "def" => DEF_KW,
            "match" => MATCH_KW,
            "while" => WHILE_KW,
            "break" => BREAK_KW,
            "continue" => CONTINUE_KW,
            "union" => UNION_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            ';' => SEMICOLON,
            ':' => COLON,
            ',' => COMMA,
            '.' => DOT,
            '(' => L_PAREN,
            ')' => R_PAREN,
            '{' => L_CURLY,
            '}' => R_CURLY,
            '[' => L_BRACK,
            ']' => R_BRACK,
            '&' => AMP,
            '|' => PIPE,
            '^' => CARET,
            '+' => PLUS,
            '-' => MINUS,
            '*' => STAR,
            '/' => SLASH,
            '%' => PERCENT,
            '=' => EQ,
            '<' => L_ANGLE,
            '>' => R_ANGLE,
            '!' => BANG,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules ! T { [ ; ] => { $ crate :: SyntaxKind :: SEMICOLON } ; [ : ] => { $ crate :: SyntaxKind :: COLON } ; [ , ] => { $ crate :: SyntaxKind :: COMMA } ; [ . ] => { $ crate :: SyntaxKind :: DOT } ; [ '(' ] => { $ crate :: SyntaxKind :: L_PAREN } ; [ ')' ] => { $ crate :: SyntaxKind :: R_PAREN } ; [ '{' ] => { $ crate :: SyntaxKind :: L_CURLY } ; [ '}' ] => { $ crate :: SyntaxKind :: R_CURLY } ; [ '[' ] => { $ crate :: SyntaxKind :: L_BRACK } ; [ ']' ] => { $ crate :: SyntaxKind :: R_BRACK } ; [ && ] => { $ crate :: SyntaxKind :: AMP2 } ; [ || ] => { $ crate :: SyntaxKind :: PIPE2 } ; [ & ] => { $ crate :: SyntaxKind :: AMP } ; [ | ] => { $ crate :: SyntaxKind :: PIPE } ; [ ^ ] => { $ crate :: SyntaxKind :: CARET } ; [ >> ] => { $ crate :: SyntaxKind :: SHR } ; [ << ] => { $ crate :: SyntaxKind :: SHL } ; [ + ] => { $ crate :: SyntaxKind :: PLUS } ; [ - ] => { $ crate :: SyntaxKind :: MINUS } ; [ * ] => { $ crate :: SyntaxKind :: STAR } ; [ / ] => { $ crate :: SyntaxKind :: SLASH } ; [ % ] => { $ crate :: SyntaxKind :: PERCENT } ; [ ** ] => { $ crate :: SyntaxKind :: STAR2 } ; [ = ] => { $ crate :: SyntaxKind :: EQ } ; [ == ] => { $ crate :: SyntaxKind :: EQ2 } ; [ != ] => { $ crate :: SyntaxKind :: NEQ } ; [ < ] => { $ crate :: SyntaxKind :: L_ANGLE } ; [ > ] => { $ crate :: SyntaxKind :: R_ANGLE } ; [ <= ] => { $ crate :: SyntaxKind :: LTEQ } ; [ >= ] => { $ crate :: SyntaxKind :: GTEQ } ; [ ! ] => { $ crate :: SyntaxKind :: BANG } ; [ &= ] => { $ crate :: SyntaxKind :: AMPEQ } ; [ |= ] => { $ crate :: SyntaxKind :: PIPEEQ } ; [ ^= ] => { $ crate :: SyntaxKind :: CARETEQ } ; [ >>= ] => { $ crate :: SyntaxKind :: SHREQ } ; [ <<= ] => { $ crate :: SyntaxKind :: SHLEQ } ; [ += ] => { $ crate :: SyntaxKind :: PLUSEQ } ; [ -= ] => { $ crate :: SyntaxKind :: MINUSEQ } ; [ *= ] => { $ crate :: SyntaxKind :: STAREQ } ; [ /= ] => { $ crate :: SyntaxKind :: SLASHEQ } ; [ %= ] => { $ crate :: SyntaxKind :: PERCENTEQ } ; [ -> ] => { $ crate :: SyntaxKind :: THIN_ARROW } ; [ var ] => { $ crate :: SyntaxKind :: VAR_KW } ; [ if ] => { $ crate :: SyntaxKind :: IF_KW } ; [ else ] => { $ crate :: SyntaxKind :: ELSE_KW } ; [ type ] => { $ crate :: SyntaxKind :: TYPE_KW } ; [ def ] => { $ crate :: SyntaxKind :: DEF_KW } ; [ match ] => { $ crate :: SyntaxKind :: MATCH_KW } ; [ while ] => { $ crate :: SyntaxKind :: WHILE_KW } ; [ break ] => { $ crate :: SyntaxKind :: BREAK_KW } ; [ continue ] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; [ union ] => { $ crate :: SyntaxKind :: UNION_KW } ; [ true ] => { $ crate :: SyntaxKind :: TRUE_KW } ; [ false ] => { $ crate :: SyntaxKind :: FALSE_KW } ; [ ident ] => { $ crate :: SyntaxKind :: IDENT } ; }
