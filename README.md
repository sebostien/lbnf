# lbnf

A crate for parsing LBNF grammar.
The resulting structure can be used to create complete parsers for context-free grammars.

## What is LBNF?

Labelled Backus-Naur form is an extension to [BNF] formalized by the tool [BNFC].
In addition to regular BNF syntax, each production must also be given a label
which can then be used to infer an AST structure.

This crate does not follow the exact structure of the rules specified in
BNFC's [LBNF reference]. Some liberties, especially with the macro and
pragma structure, have been taken in order to make LBNF suitable
for a wider number of applications.

[BNF]: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form
[BNFC]: https://github.com/BNFC/bnfc
[LBNF reference]: https://bnfc.readthedocs.io/en/latest/lbnf.html?

## Example

Given the LBNF grammar:

```BNF
EAdd. Expr ::= Expr "+" Num ;
ENum. Expr ::= Num          ;

NZero. Num ::= "0" ;
```

It is possible to generate the structure:

```rust
enum Expr {
    EAdd(Box<Expr>, Num),
    ENum(Num),
}

enum Num {
    NZero,
}
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
