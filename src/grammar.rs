#[derive(Debug, Clone, PartialEq)]
pub struct Grammar {
    pub definitions: Vec<Def>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Def {
    /// A regular LBNF rule of the form:
    /// `Label. Cat ::= (Identifier | String)* ;`
    Rule(Label, Cat, Vec<Item>),
    /// A custom macro in the form:
    /// `Ident <Exp> ;`
    Macro(Ident, Exp),
}

/// Either a terminal or non-terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// `"terminal"`
    Terminal(String),
    /// `nterminal`
    NTerminal(Cat),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cat {
    /// `[Cat]`
    ListCat(Box<Cat>),
    /// `Ident`
    IdCat(Ident),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Exp {
    /// `Exp : Exp`
    Cons(Box<Exp>, Box<Exp>),
    /// `Exp | Exp`
    Or(Box<Exp>, Box<Exp>),
    /// `Ident(Exp, Exp)`
    App(Ident, Vec<Exp>),
    /// `[Exp, Exp]`
    List(Vec<Exp>),
    /// `Ident`
    Var(Ident),
    /// `a+`
    Many1(Box<Exp>),
    /// `a*`
    Many0(Box<Exp>),
    /// `123`
    LitInt(i64),
    /// `1.23`
    LitDouble(f64),
    /// `'a'`
    LitChar(char),
    /// `"abc"`
    LitString(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Label {
    /// `Ident`
    Id(Ident),
    /// Should be ignored in the AST.
    /// `_`
    Wild,
    /// `[]`
    ListE,
    /// `(:)`
    ListCons,
    /// `(:[])`
    ListOne,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(pub String);
