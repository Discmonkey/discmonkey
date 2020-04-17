use crate::types::list::List;
use crate::exec::eval::{eval_ast};
use crate::types::ast::LispValue;
use crate::exec::closure::create_closure;
use crate::types::env::Scope;

/// lisp let rules are somewhat complicated and this method does not do a good job of making them not compliated.
pub fn apply_let(list: &List, env: &mut Scope) -> LispValue {
    if list.len() != 3 {
        return LispValue::Error("let* two arguments in list".to_string())
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
                        _ => {
                            return LispValue::Error("assignment list even argument be string symbol".to_string())
                        }
                    }
                } else {
                    rvalue = eval_ast(val, &mut new_scope);

                    if let LispValue::Error(e) = rvalue {
                        return LispValue::Error(e)
                    }

                    new_scope.set(key.clone(), rvalue);
                }
            }

            eval_ast(&list[2], &mut new_scope)
        }

        _ => LispValue::Error("first argument to let* must be assignment list".to_string()),
    }
}

pub fn apply_do(list: &List, env: &mut Scope) -> LispValue {
    list.items().iter().skip(1).map(|item| {
        eval_ast(item, env)
    }).last().unwrap_or(LispValue::Nil)
}

pub fn apply_if(list: &List, env: &mut Scope) -> LispValue {
    let length = list.len();

    // (if true)
    if length < 3 {
        return LispValue::Error("if statement needs at least one statement to execute".to_string())
    }

    if length > 4 {
        return LispValue::Error("if statement can have at most two arms".to_string())
    }

    let boolean_flag = eval_ast(&list[1], env);

    match boolean_flag {
        LispValue::Boolean(false) | LispValue::Nil  => {
            match length {
                4 => eval_ast(&list[3], env),
                _ => LispValue::Nil
            }
        },
        // don't evaluate on error and forward
        LispValue::Error(s) => LispValue::Error(s),

        // everything else is considered "truthy"
        _ => {
            eval_ast(&list[2], env)
        }
    }

}

pub fn create_func(list: &List, _env: &mut Scope) -> LispValue {
    if list.len() != 3 {
        return LispValue::Error("usage fn* (args list) (body)".to_string());
    }

    let mut args = Vec::new();

    match &list[1] {
        LispValue::List(l) => {
            for val in l.items() {
                match val {
                    LispValue::Atom(v) => {
                        args.push(v.token().clone());
                    },

                    _ => return LispValue::Error("function args list must be symbols".to_string())
                }
            }
        },
        _ => return LispValue::Error("function args must be a list".to_string())
    }


    LispValue::Function(create_closure(args, list[2].clone()))
}

pub fn apply_def(list: &List,  env: &mut Scope) -> LispValue {
    if list.len() != 3 {
        return LispValue::Error("incorrect number of args for definition".to_string());
    }

    match &list[1] {
        LispValue::Atom(a) => {

            // need to clone to insert into map
            let key = a.token().get_text().clone();
            let value = eval_ast(&list[2], env);

            env.set(key.clone(), value.clone());

            value
        }

        _ => LispValue::Error("first argument to def! must be a symbol".to_string())
    }
}