#![doc = include_str!("../README.md")]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::perf,
    missing_docs
)]
#![allow(clippy::multiple_crate_versions)]

/// The structure of LBNF grammar.
pub mod grammar;

mod display;

#[cfg(test)]
mod proptests;

use crate::grammar::Grammar;

use lalrpop_util::{lexer::Token, ParseError};

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(clippy::nursery)]
    #[allow(clippy::perf)]
    #[allow(unused)]
    lbnf
);

/// Parse the given string as LBNF grammar.
/// Returns [`Grammar`] if the provided grammar is valid.
///
/// # Errors
///
/// Will return `Err` if the provided grammar fails to parse.
///
/// # Examples
///
/// ```rust
/// use lbnf::parse;
///
/// let grammar = r#"
///    EAdd.  Exp ::= Exp "+" Exp ;
///    EInt.  Exp ::= Integer     ;
/// "#;
/// assert!(parse(grammar).is_ok());
/// ```
pub fn parse(input: &str) -> Result<Grammar, ParseError<usize, Token<'_>, &str>> {
    lbnf::GrammarParser::new().parse(input)
}
