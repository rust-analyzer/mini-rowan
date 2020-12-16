use mini_rowan::*;

#[rustfmt::skip]
fn make_tree() -> SyntaxTree {
    fn kw(kw: &'static str) -> PureToken {
        PureToken::new(kw, kw)
    }
    fn op(kw: &'static str) -> PureToken {
        PureToken::new(kw, kw)
    }
    fn ident(ident: &str) -> PureToken {
        PureToken::new("ident", ident)
    }

    let func: PureTree = PureTree::new("function-decl")
        .push(kw("pub"))
        .push(kw("fun"))
        .push(PureTree::new("generic-param-list")
            .push(PureTree::new("param-decl")
                .push(ident("T"))
                .push(PureTree::new("param-bound")
                    .push(op(":"))
                    .push(ident("Clone"))
                )
            )
        )
        .push(PureTree::new("param-list").push(op("(")).push(op(")")))
        .push(PureTree::new("where-clause")
            .push(PureTree::new("where-pred")
                .push(ident("T"))
                .push(PureTree::new("param-bound")
                    .push(op(":"))
                    .push(ident("Eq"))
                )
            )
        ).into();
    func.into()
}

#[test]
fn smoke_test() {
    let func = make_tree();
    println!("{:#?}", func);

    let param_bound = func
        .find_tree("generic-param-list")
        .unwrap()
        .find_tree("param-decl")
        .unwrap()
        .find_tree("param-bound")
        .unwrap();
    println!("{:#?}", param_bound);

    let where_clause = func.find_tree("where-clause").unwrap();
    println!("{:#?}", where_clause);

    let old_offset = where_clause.offset();
    param_bound.detach();
    assert!(where_clause.offset() == old_offset - param_bound.text_len());
    where_clause.find_tree("where-pred").unwrap().insert_child(2, param_bound.into());
    println!("{:#?}", func);

    {
        let fun_kw = func.find_token("fun").unwrap();
        let generic_param_list = fun_kw.next_sibling().unwrap();
        let param_list = generic_param_list.next_sibling().unwrap();

        assert!(func.find_tree("generic-param-list").is_some());
        generic_param_list.detach();
        assert!(func.find_tree("generic-param-list").is_none());

        assert_eq!(fun_kw.next_sibling().unwrap(), param_list);
        assert_eq!(param_list.prev_sibling().unwrap().kind(), "fun");
    }
}
