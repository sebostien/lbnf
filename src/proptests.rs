use crate::grammar::{Cat, Def, Exp, Grammar, Ident, Item, Label};
use proptest::prelude::{
    any, any_with, prop, prop_assert_eq, prop_oneof, proptest, Arbitrary, BoxedStrategy, Just,
    Strategy,
};

impl Arbitrary for Grammar {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        any::<Vec<Def>>()
            .prop_map(|definitions| Grammar { definitions })
            .boxed()
    }
}

impl Arbitrary for Def {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        prop_oneof![
            (any::<Label>(), any::<Cat>(), any::<Vec<Item>>())
                .prop_map(|(a, b, c)| Self::Rule(a, b, c)),
            (any::<Ident>(), any::<Exp>()).prop_map(|(a, b)| Self::Macro(a, b)),
        ]
        .boxed()
    }
}

impl Arbitrary for Item {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        prop_oneof![
            any_with::<String>(r#"[.--"]*"#.into()).prop_map(Item::Terminal),
            any::<Cat>().prop_map(Item::NTerminal),
        ]
        .boxed()
    }
}

impl Arbitrary for Cat {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        let leaf = prop_oneof![
            any::<Ident>().prop_map(Cat::IdCat),
            any::<Ident>().prop_map(|a| Cat::ListCat(Box::new(Cat::IdCat(a)))),
            any::<Ident>()
                .prop_map(|a| Cat::ListCat(Box::new(Cat::ListCat(Box::new(Cat::IdCat(a)))))),
        ];
        leaf.prop_recursive(8, 8, 1, |inner| {
            prop_oneof![inner
                .clone()
                .prop_map(|inner| Cat::ListCat(Box::new(inner)))]
        })
        .boxed()
    }
}

impl Arbitrary for Exp {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        // Terminals:
        //   Var, LitInt, LitDouble, LitChar, LitString
        let leaf = prop_oneof![
            any::<Ident>().prop_map(Self::Var),
            any::<i64>().prop_map(Self::LitInt),
            any::<f64>().prop_map(Self::LitDouble),
            any_with::<String>(r#"[.--"]"#.into())
                .prop_map(|s| Self::LitChar(s.chars().next().unwrap())),
            any_with::<String>(r#"[.--"]*"#.into()).prop_map(Self::LitString),
        ];

        // Non-Terminals:
        //  Cons, Or, App, List, Many1, Many0
        leaf.prop_recursive(8, 256, 8, |inner| {
            prop_oneof![
                (inner.clone(), inner.clone())
                    .prop_map(|(a, b)| Exp::Cons(Box::new(a), Box::new(b))),
                (inner.clone(), inner.clone()).prop_map(|(a, b)| Exp::Or(Box::new(a), Box::new(b))),
                (any::<Ident>(), prop::collection::vec(inner.clone(), 0..10))
                    .prop_map(|(ident, exps)| Exp::App(ident, exps)),
                prop::collection::vec(inner.clone(), 0..10).prop_map(Exp::List),
                inner.clone().prop_map(|e| Exp::Many1(Box::new(e))),
                inner.prop_map(|e| Exp::Many0(Box::new(e))),
            ]
        })
        .boxed()
    }
}

impl Arbitrary for Label {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Self::Wild),
            Just(Self::ListE),
            Just(Self::ListCons),
            Just(Self::ListOne),
            any::<Ident>().prop_map(Self::Id),
        ]
        .boxed()
    }
}

impl Arbitrary for Ident {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        prop_oneof![any_with::<String>("[a-zA-Z][a-zA-Z0-9_]*".into()).prop_map(Self)].boxed()
    }
}

proptest! {
    #[test]
    fn test_parse_grammar(source : Grammar) {
        let regular = format!("{}", source);
        let parsed_node = crate::parse(&regular);
        prop_assert_eq!(
            Ok(&source),
            parsed_node.as_ref(),
            "\n---- Formatted grammar ----\n{}",
            regular
        );

        let pretty = format!("{:#}", source);
        let parsed_node = crate::parse(&pretty);
        prop_assert_eq!(
            Ok(&source),
            parsed_node.as_ref(),
            "\n---- Formatted grammar (with # flag) ----\n{}",
            pretty
        );

    }
}
