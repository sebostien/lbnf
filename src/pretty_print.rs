use crate::grammar::{Cat, Def, Exp, Grammar, Ident, Item, Label};

fn escape_quotes(s: &str) -> String {
    s.replace('"', r#"\""#)
}

fn join_slice<T: std::fmt::Display>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(sep)
}

impl std::fmt::Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for def in &self.definitions {
            writeln!(f, "{def}")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Def {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rule(ident, cat, items) => {
                write!(f, "{ident}. {cat} ::= {} ;", join_slice(items, " "))
            }
            Self::Macro(ident, exp) => write!(f, "{ident} {exp} ;"),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Terminal(s) => write!(f, "\"{}\"", escape_quotes(s)),
            Self::NTerminal(cat) => write!(f, "{cat}"),
        }
    }
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListCat(cat) => write!(f, "[{cat}]"),
            Self::IdCat(ident) => write!(f, "{ident}"),
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{id}"),
            Self::Wild => write!(f, "_"),
            Self::ListE => write!(f, "[]"),
            Self::ListCons => write!(f, "(:)"),
            Self::ListOne => write!(f, "(:[])"),
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn format_double(d: f64) -> String {
    let d = format!("{d}");
    if d.contains(|c| c == '.') {
        d
    } else {
        d + ".0"
    }
}

impl std::fmt::Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cons(e1, e2) => write!(f, "({e1} : {e2})"),
            Self::App(ident, params) => write!(f, "{ident}({})", join_slice(params, ", ")),
            Self::Var(ident) => write!(f, "{ident}"),
            Self::LitInt(i) => write!(f, "{i}"),
            Self::LitChar(c) => write!(f, "'{c}'"),
            Self::LitString(s) => write!(f, r#""{}""#, escape_quotes(s)),
            Self::LitDouble(d) => write!(f, "{}", format_double(*d)),
            Self::List(l) => write!(f, "[{}]", join_slice(l, ", ")),
            Self::Or(a, b) => write!(f, "(({a}) | ({b}))"),
            Self::Many1(a) => write!(f, "(({a})+)"),
            Self::Many0(a) => write!(f, "(({a})*)"),
        }
    }
}
