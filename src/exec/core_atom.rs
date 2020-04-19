use crate::types::list::List;
use crate::types::env::Scope;
use crate::types::ast::LispValue;
use crate::exec::eval::eval_ast;
use std::rc::Rc;
use std::cell::RefCell;
use crate::arg_return;


pub fn apply_atom(list: &List, env: &mut Scope) -> LispValue {
    arg_return!(apply_atom, 1, list);

    let res = eval_ast(&list[1], env);

    if let LispValue::Error(_e) = res {
        LispValue::Error("cannot box error value".to_string())
    } else {
        LispValue::Atom(Rc::new(RefCell::new(res)))
    }
}

pub fn apply_is_atom(list: &List, env: &mut Scope) -> LispValue {
    arg_return!(is_atom, 1, list);

    let res = eval_ast(&list[1], env);

    if let LispValue::Atom(_a) = res {
        LispValue::Boolean(true)
    } else {
        LispValue::Boolean(false)
    }
}

pub fn apply_deref(list: &List, env: &mut Scope) -> LispValue {
    arg_return!(deref, 1, list);

    let res = eval_ast(&list[1], env);

    if let LispValue::Atom(val) = res {
        val.as_ref().borrow().clone()
    } else {
        LispValue::Error("value is not an atom".to_string())
    }
}


pub fn apply_reset(list: &List, env: &mut Scope) -> LispValue {
    arg_return!(reset, 2, list);

    let maybe_atom = eval_ast(&list[1], env);
    let new_value = eval_ast(&list[2], env);

    if let LispValue::Atom(val) = maybe_atom {
        if let LispValue::Error(e) = new_value {
            return LispValue::Error(e)
        }

        val.as_ref().replace(new_value.clone());

        new_value

    } else {
        LispValue::Error("first argument to reset should be an atom".to_string())
    }
}

pub fn apply_swap(list: &List, env: &mut Scope) -> LispValue {
    arg_return!(swap, 2, list);

    let maybe_atom = eval_ast(&list[1], env);
    let maybe_f = eval_ast(&list[2], env);

    if let LispValue::Atom(val) = maybe_atom {
        if let LispValue::Function(f) = maybe_f {

            let mut input_list = List::new();
            input_list.push(LispValue::Nil);
            input_list.push(val.as_ref().borrow().clone());

            let new_value = f(&input_list, env);

            if let LispValue::Error(e) = new_value {
                return LispValue::Error(e)
            }

            val.as_ref().replace(new_value.clone());

            new_value

        } else {
            LispValue::Error("second argument to swap must be a function".to_string())
        }
    } else {
        LispValue::Error("first argument to swap should be an atom".to_string())
    }
}