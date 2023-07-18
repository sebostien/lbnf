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
            def.fmt(f)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Def {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rule(ident, cat, items) => {
                write!(f, "{ident}. {cat} ::= {} ;", join_slice(items, " "))?;
            }
            Self::Macro(ident, exp) => write!(f, "{ident} {exp} ;")?,
        }

        if f.alternate() {
            writeln!(f)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Terminal(ref s) => write!(f, "\"{}\"", escape_quotes(s)),
            Self::NTerminal(ref cat) => cat.fmt(f),
        }
    }
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListCat(cat) => write!(f, "[{cat}]"),
            Self::IdCat(ident) => ident.fmt(f),
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => id.fmt(f),
            Self::Wild => f.write_str("_"),
            Self::ListE => f.write_str("[]"),
            Self::ListCons => f.write_str("(:)"),
            Self::ListOne => f.write_str("(:[])"),
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn format_double(d: f64) -> String {
    let d = d.to_string();
    if d.contains(|c| c == '.') {
        d
    } else {
        d + ".0"
    }
}

impl std::fmt::Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Cons(ref e1, ref e2) => write!(f, "({e1} : {e2})"),
            Self::App(ref ident, ref params) => write!(f, "{ident}({})", join_slice(params, ", ")),
            Self::Var(ref ident) => ident.fmt(f),
            Self::LitInt(i) => i.fmt(f),
            Self::LitChar(c) => write!(f, "'{c}'"),
            Self::LitString(ref s) => write!(f, r#""{}""#, escape_quotes(s)),
            Self::LitDouble(d) => format_double(d).fmt(f),
            Self::List(ref l) => write!(f, "[{}]", join_slice(l, ", ")),
            Self::Or(ref a, ref b) => write!(f, "(({a}) | ({b}))"),
            Self::Many1(ref a) => write!(f, "(({a})+)"),
            Self::Many0(ref a) => write!(f, "(({a})*)"),
        }
    }
}
