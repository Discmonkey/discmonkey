use crate::types::list::List;
use crate::exec::env::{Scope, LispResult};
use crate::exec::eval::{eval_ast};
use crate::types::ast::LispValue;
use crate::exec::closure::create_closure;

/// lisp let rules are somewhat complicated and this method does not do a good job of making them not compliated.
pub (super) fn apply_let(list: &List, env: &mut Scope) -> LispResult {
    if list.len() != 3 {
        return LispResult::Error("let* two arguments in list".to_string())
    }

    // safe unwrap since we pre-check the length
    match &list[1] {
        LispValue::List(assignment_list) => {

            let mut new_scope = env.new_scope();
            let mut key= "".to_string();
            let mut rvalue;

            for (i, val) in assignment_list.items().iter().enumerate() {
                if i % 2  == 0 {
                    match val {
                        LispValue::Atom(a) => key = a.token().get_text().clone(),
                        LispValue::List(_l) => {
                            return LispResult::Error("assignment list even argument be string symbol".to_string())
                        }
                    }
                } else {
                    rvalue = eval_ast(val, &mut new_scope);

                    if let LispResult::Error(e) = rvalue {
                        return LispResult::Error(e)
                    }

                    new_scope.set(key.clone(), rvalue);
                }
            }

            eval_ast(&list[2], &mut new_scope)
        }

        LispValue::Atom(_a) => LispResult::Error("first argument to let* must be assignment list".to_string()),
    }
}

pub (super) fn apply_do(list: &List, env: &mut Scope) -> LispResult {
    list.items().iter().skip(1).map(|item| {
        eval_ast(item, env)
    }).last().unwrap_or(LispResult::Nil)
}

pub (super) fn apply_if(list: &List, env: &mut Scope) -> LispResult {
    let length = list.len();

    // (if true)
    if length < 3 {
        return LispResult::Error("if statement needs at least one statement to execute".to_string())
    }

    if length > 4 {
        return LispResult::Error("if statement can have at most two arms".to_string())
    }

    let boolean_flag = eval_ast(&list[1], env);

    match boolean_flag {
        LispResult::Boolean(false) | LispResult::Nil  => {
            match length {
                4 => eval_ast(&list[3], env),
                _ => LispResult::Nil
            }
        },
        // don't evaluate on error and forward
        LispResult::Error(s) => LispResult::Error(s),

        // everything else is considered "truthy"
        _ => {
            eval_ast(&list[2], env)
        }
    }

}

pub (super) fn create_func(list: &List, _env: &mut Scope) -> LispResult {
    if list.len() != 3 {
        return LispResult::Error("usage fn* (args list) (body)".to_string());
    }

    let mut args = Vec::new();

    match &list[1] {
        LispValue::List(l) => {
            for val in l.items() {
                match val {
                    LispValue::Atom(v) => {
                        args.push(v.token().clone());
                    },

                    _ => return LispResult::Error("function args list must be symbols".to_string())
                }
            }
        },
        LispValue::Atom(_a) => return LispResult::Error("function args must be a list".to_string())
    }


    LispResult::Function(create_closure(args, list[2].clone()))
}

pub (super) fn apply_def(list: &List,  env: &mut Scope) -> LispResult {
    if list.len() != 3 {
        return LispResult::Error("incorrect number of args for definition".to_string());
    }

    match &list[1] {
        LispValue::Atom(a) => {

            // need to clone to insert into map
            let key = a.token().get_text().clone();
            let value = eval_ast(&list[2], env);

            env.set(key.clone(), value.clone());

            value
        }

        LispValue::List(_l) => LispResult::Error("first argument to def! must be a symbol".to_string())
    }
}