//! Generate syntax related stuff.

use super::SYNTAX_KINDS;
use crate::{
    ast::{KindsSrc, KINDS_SRC},
    util::to_upper_snake_case,
};
use anyhow::{Context, Result};
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};
use std::path::Path;

macro_rules! collect_values {
    ($val:expr, $action:expr) => {
        $val.iter().map($action).collect::<Vec<_>>()
    };

    ($val:expr) => {
        $val.iter()
            .map(|name| format_ident!("{}", name))
            .collect::<Vec<_>>()
    };
}

pub fn generate_syntax() -> Result<()> {
    let syntax_kinds = generate_syntax_kinds(KINDS_SRC)?;
    write_to_file(SYNTAX_KINDS, syntax_kinds)?;

    Ok(())
}

fn write_to_file(path: impl AsRef<Path>, source: String) -> Result<()> {
    let path = path.as_ref();
    std::fs::write(path, source)
        .with_context(|| format!("failed to write to file {}", path.display()))
}

fn generate_syntax_kinds(grammar: KindsSrc<'_>) -> Result<String> {
    let (single_tokens_values, single_tokens): (Vec<_>, Vec<_>) = grammar
        .punct
        .iter()
        .filter(|(token, _name)| token.len() == 1)
        .map(|(token, name)| (token.chars().next().unwrap(), format_ident!("{}", name)))
        .unzip();

    let punctuation_values = grammar.punct.iter().map(|(token, _name)| {
        if "{}[]()".contains(token) {
            let c = token.chars().next().unwrap();
            quote! { #c }
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            quote! { #(#cs)* }
        }
    });
    let punctuation = collect_values!(grammar.punct, |(_token, name)| format_ident!("{}", name));
    let literals = collect_values!(grammar.literals);
    let tokens = collect_values!(grammar.tokens);
    let nodes = collect_values!(grammar.nodes);

    let all_keywords_values = grammar.keywords;
    let all_keywords_idents = all_keywords_values.iter().map(|kw| format_ident!("{}", kw));
    let all_keywords = all_keywords_values
        .iter()
        .map(|name| format_ident!("{}_KW", to_upper_snake_case(&name)))
        .collect::<Vec<_>>();
    let ast = quote! {
        #![allow(bad_style, missing_docs, unreachable_pub)]
        /// The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        pub enum SyntaxKind {
            #[doc(hidden)]
            EOF,
            #(#punctuation,)*
            #(#all_keywords,)*
            #(#literals,)*
            #(#tokens,)*
            #(#nodes,)*

            // Technical kind so that we can cast from u16 safely
            #[doc(hidden)]
            __LAST,
        }
        use self::SyntaxKind::*;

        impl SyntaxKind {
            pub fn is_keyword(self) -> bool {
                match self {
                    #(#all_keywords)|* => true,
                    _ => false,
                }
            }

            pub fn is_punct(self) -> bool {
                match self {
                    #(#punctuation)|* => true,
                    _ => false,
                }
            }

            pub fn is_literal(self) -> bool {
                match self {
                    #(#literals)|* => true,
                    _ => false,
                }
            }

            pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
                let kw = match ident {
                    #(#all_keywords_values => #all_keywords,)*
                    _ => return None,
                };
                Some(kw)
            }

            pub fn from_char(c: char) -> Option<SyntaxKind> {
                let tok = match c {
                    #(#single_tokens_values => #single_tokens,)*
                    _ => return None,
                };
                Some(tok)
            }
        }

        #[macro_export]
        macro_rules! T {
            #([#punctuation_values] => { $crate::SyntaxKind::#punctuation };)*
            #([#all_keywords_idents] => { $crate::SyntaxKind::#all_keywords };)*
            [ident] => { $crate::SyntaxKind::IDENT };
        }
    };

    crate::reformat(ast)
}
