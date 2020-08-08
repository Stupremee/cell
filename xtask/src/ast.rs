//! Input for syntax code generation.

pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    pub(crate) literals: &'a [&'a str],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc<'static> = KindsSrc {
    punct: &[
        (";", "SEMICOLON"),
        (":", "COLON"),
        (",", "COMMA"),
        (".", "DOT"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("&&", "AMP2"),
        ("||", "PIPE2"),
        ("&", "AMP"),
        ("|", "PIPE"),
        ("^", "CARET"),
        (">>", "SHR"),
        ("<<", "SHL"),
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("*", "STAR"),
        ("/", "SLASH"),
        ("%", "PERCENT"),
        ("**", "STAR2"),
        ("=", "EQ"),
        ("==", "EQ2"),
        ("!=", "NEQ"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("<=", "LTEQ"),
        (">=", "GTEQ"),
        ("!", "BANG"),
        ("&=", "AMPEQ"),
        ("|=", "PIPEEQ"),
        ("^=", "CARETEQ"),
        (">>=", "SHREQ"),
        ("<<=", "SHLEQ"),
        ("+=", "PLUSEQ"),
        ("-=", "MINUSEQ"),
        ("*=", "STAREQ"),
        ("/=", "SLASHEQ"),
        ("%=", "PERCENTEQ"),
        ("->", "THIN_ARROW"),
    ],
    keywords: &[
        "var", "if", "else", "type", "def", "match", "while", "break", "continue", "union", "true",
        "false",
    ],
    literals: &["INT_NUMBER", "FLOAT_NUMBER", "CHAR", "STRING"],
    tokens: &["ERROR", "IDENT", "WHITESPACE", "COMMENT"],
    nodes: &[
        "PROGRAM",
        "TYPE_DEF",
        "TYPE_IMPL",
        "TYPE_ALIAS",
        "TYPE_TRAIT",
        "TYPE_UNION",
        "FN_DEF",
        "PARAMETER_LIST",
        "VAR_STMT",
        "EXPR_STMT",
        "PAREN_EXPR",
        "TUPLE_EXPR",
        "NAMED_TUPLE_EXPR",
        "IF_EXPR",
        "MATCH_EXPR",
        "MATCH_ARM",
        "WHILE_EXPR",
        "CONTINUE",
        "BREAK",
        "CALL_EXPR",
        "PREFIX_EXPR",
        "BINARY_EXPR",
        "CMP_EXPR",
        "BLOCK_EXPR",
        "CONDITION",
        "LITERAL",
        "NAME",
    ],
};

#[derive(Default, Debug)]
pub(crate) struct AstSrc<'s> {
    pub(crate) tokens: &'s [&'s str],
    pub(crate) nodes: &'s [AstNodeSrc<'s>],
    pub(crate) enums: &'s [&'s str],
}

#[derive(Default, Debug)]
pub(crate) struct AstNodeSrc<'s> {
    pub(crate) name: &'s str,
    pub(crate) fields: &'s [Field<'s>],
}

#[derive(Debug)]
pub(crate) enum Field<'s> {
    Token(&'s str),
    Node { name: &'s str, src: FieldSrc<'s> },
}

#[derive(Debug)]
pub(crate) enum FieldSrc<'a> {
    Optional(&'a str),
    Many(&'a str),
}

#[derive(Debug)]
pub(crate) struct AstEnumSrc<'s> {
    pub(crate) name: &'s str,
    pub(crate) variants: &'s [&'s str],
}
