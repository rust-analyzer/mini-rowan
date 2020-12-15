use mini_rowan::*;

#[rustfmt::skip]
fn make_tree() -> FunTree {
    fn kw(kw: &'static str) -> FunToken {
        FunToken::new(kw, kw)
    }
    fn op(kw: &'static str) -> FunToken {
        FunToken::new(kw, kw)
    }
    fn ident(ident: &str) -> FunToken {
        FunToken::new("ident", ident)
    }

    let func: FunTree = FunTree::new("function-decl")
        .push(kw("pub"))
        .push(kw("fun"))
        .push(FunTree::new("generic-param-list")
            .push(FunTree::new("param-decl")
                .push(ident("T"))
                .push(FunTree::new("param-bound")
                    .push(op(":"))
                    .push(ident("Clone"))
                )
            )
        )
        .push(FunTree::new("param-list").push(op("(")).push(op(")")))
        .push(FunTree::new("where-clause")
            .push(FunTree::new("where-pred")
                .push(ident("T"))
                .push(FunTree::new("param-bound")
                    .push(op(":"))
                    .push(ident("Eq"))
                )
            )
        ).into();
    eprintln!("func = {:?}", func);
    func
}

#[test]
fn smoke_test() {
    let func = make_tree();
    println!("{:#?}\n\n", func);

    //     let param_bound = func
    //         .find("generic-param-list")
    //         .unwrap()
    //         .find("param-decl")
    //         .unwrap()
    //         .find("param-bound")
    //         .unwrap();
    //     println!("{:#}", param_bound);

    //     let where_clause = func.find("where-clause").unwrap();
    //     println!("{:#}", where_clause);

    //     param_bound.detach();
    //     where_clause.find("where-pred").unwrap().insert_child(2, param_bound);
    //     println!("{:#}", func);

    //     {
    //         let fun_kw = func.find("fun").unwrap();
    //         let generic_param_list = fun_kw.next_sibling().unwrap();
    //         let param_list = generic_param_list.next_sibling().unwrap();

    //         assert!(func.find("generic-param-list").is_some());
    //         generic_param_list.detach();
    //         assert!(func.find("generic-param-list").is_none());

    //         assert_eq!(fun_kw.next_sibling().unwrap(), param_list);
    //         assert_eq!(param_list.prev_sibling().unwrap(), fun_kw);
    //     }
}
