use crate::types::list::List;
use crate::types::ast::LispValue;
use crate::types::env::Scope;
use crate::arg_return;
use crate::exec::eval::eval_ast;

pub fn apply_cons(list: &List, env: &mut Scope) -> LispValue {
    arg_return!(cons, 2, list);

    if let LispValue::List(l) = eval_ast(&list[2],  env) {
        let mut r = List::new();

        r.push(eval_ast(&list[1], env));

        l.items().iter().for_each(|item| {
            r.push(item.clone())
        });

        LispValue::List(r)
    } else {
        LispValue::Error("second argument to cons needs to be list".to_string())
    }
}

pub fn apply_concat(list: &List, env: &mut Scope) -> LispValue {

    let mut r = List::new();

    for maybe_list in list.items().iter().skip(1) {

        if let LispValue::List(l) =  eval_ast(maybe_list, env) {
            l.items().iter().for_each(|item| {
                    r.push(item.clone());
                }
            )
        } else {
            return LispValue::Error("every argument to concat must be a list".to_string());
        }
    }

    LispValue::List(r)
}