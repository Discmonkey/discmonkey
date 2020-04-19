use crate::types::list::List;
use crate::types::ast::LispValue;
use crate::types::env::Scope;
use crate::arg_return;
use crate::exec::eval::eval_ast;

pub fn apply_quote(list: &List, _env: &mut Scope) -> LispValue {
    if list.len() > 2 {
        LispValue::Error("quoting is only supported for a single argument".to_string())
    } else {
        list[1].clone()
    }
}

pub fn apply_macro(list: &List, env: &mut Scope) -> LispValue {
    arg_return!(defmacro, 2, list);

    match &list[1] {
        LispValue::Unit(a) => {
            // need to clone to insert into map
            let key = a.token().get_text().clone();
            let value = eval_ast(&list[2], env);

            if let LispValue::Function(f) = value {
                env.set(key, LispValue::Macro(f.clone()));

                LispValue::Macro(f)
            } else {
                LispValue::Error("the second argument to macro! must be a function".to_string())
            }
        }

        _ => LispValue::Error("first argument to macro! must be a symbol".to_string())
    }
}

