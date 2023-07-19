# LBNF

A crate for parsing LBNF.

This crate is solely responsible for parsing LBNF.
Several components, such as lexer, parser, and AST generation, are
missing and needed to generate complete parsers for context-free grammars.

## What is LBNF?

Labeled Backus-Naur form (LBNF) is an extension of [BNF] formalized by the tool [BNFC].
In addition to regular BNF syntax, each production is also given a label that
is used to generate the abstract syntax of context-free languages.

This crate does not follow all the rules specified in BNFC's [LBNF reference].
Some changes, especially with the macro syntax, have been made to make LBNF
suitable for a wider number of applications.

[BNF]: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form
[BNFC]: https://github.com/BNFC/bnfc
[LBNF reference]: https://bnfc.readthedocs.io/en/latest/lbnf.html?

## Usage

```rust
let source = r#"
    EAdd.  Expr ::= Expr "+" Num ;
    ENum.  Expr ::= Num          ;

    NZero. Num  ::= "0"          ;
"#;

if let Ok(grammar) = lbnf::parse(source) {
    // The parsed grammar can now be used to generate the following structure:
    enum Expr {
        EAdd(Box<Expr>, Num),
        ENum(Num),
    }

    enum Num {
        NZero
    }
}

```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
