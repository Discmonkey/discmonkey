use crate::types::list::List;
use crate::exec::env::{Scope, LispResult};
use crate::exec::eval::{eval_ast};


macro_rules! comp {
    ($type:ident, $left:expr, $right:expr) => {
        if let LispResult::$type(unwrapped_r) = $right {
            LispResult::Boolean($left == unwrapped_r)
        } else {
            LispResult::Boolean(false)
        }
    }
}

pub (super) fn apply_equals(list: &List, env: &mut Scope) -> LispResult {
    if list.len() != 3 {
        return LispResult::Error("equals needs two comparisons".to_string());
    }

    let l = eval_ast(&list[1], env);
    let r = eval_ast(&list[2], env);

    match l {
        LispResult::Error(s) => LispResult::Error(s),
        LispResult::Nil => LispResult::Boolean(false),
        LispResult::Function(_f) => LispResult::Boolean(false),
        LispResult::Float(f) => comp!(Float, f, r),
        LispResult::Int(i) => comp!(Int, i, r),
        LispResult::Boolean(b) => comp!(Boolean, b, r)
    }

}