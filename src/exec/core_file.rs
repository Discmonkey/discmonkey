use crate::types::list::List;
use crate::exec::eval::{eval_ast};
use std::fs;
use crate::types::ast::LispValue;
use crate::types::env::Scope;

pub fn apply_slurp(list: &List, env: &mut Scope) -> LispValue {
    if list.len() != 2 {
        LispValue::Error("slurp takes only one argument".to_string())
    } else {
        match eval_ast(&list[1], env) {
            LispValue::Error(error) => LispValue::Error(error),

            LispValue::String(filename) => {
                match fs::read_to_string(filename) {
                    Result::Err(e) => LispValue::Error(e.to_string()),
                    Result::Ok(contents) => LispValue::String(contents)
                }
            },

            _ => LispValue::Error("slurp needs a filename!".to_string())
        }

    }
}