use crate::types::list::List;
use crate::types::env::Scope;
use crate::types::ast::LispValue;
use crate::exec::eval::eval_ast;

pub fn apply_atom(list: &List, env: &mut Scope) -> LispValue {
    if list.len() != 2 {
        LispValue::Error("atom takes a single arugment".to_string())
    } else {
        let res = eval_ast(&list[1], env);

        if let LispValue::Error(_e) = res {
            LispValue::Error("cannot box error value".to_string())
        } else {
            LispValue::Atom(Box::new(res.clone()))
        }
    }
}