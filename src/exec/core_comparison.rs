use crate::types::list::List;
use crate::exec::env::{Scope, LispResult};
use crate::exec::eval::{eval_ast};


macro_rules! comp {
    ($comp:tt, $type:ident, $left:expr, $right:expr) => {
        if let LispResult::$type(unwrapped_r) = $right {
            LispResult::Boolean($left $comp unwrapped_r)
        } else {
            LispResult::Boolean(false)
        }
    }
}

macro_rules! comp_op {
    ($func:ident, $op:tt) => {
        pub (super) fn $func(list: &List, env: &mut Scope) -> LispResult {
            if list.len() != 3 {
                return LispResult::Error(stringify!($op works with exactly two items to compare).to_string());
            }

            let l = eval_ast(&list[1], env);
            let r = eval_ast(&list[2], env);

            match l {
                LispResult::Error(s) => LispResult::Error(s),
                LispResult::Nil => LispResult::Boolean(false),
                LispResult::Function(_f) => LispResult::Boolean(false),
                LispResult::Float(f) => comp!($op, Float, f, r),
                LispResult::Int(i) => comp!($op, Int, i, r),
                LispResult::Boolean(_b) => LispResult::Boolean(false),
                LispResult::String(_s) => LispResult::Boolean(false)
            }
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
        LispResult::Float(f) => comp!(==, Float, f, r),
        LispResult::Int(i) => comp!(==, Int, i, r),
        LispResult::Boolean(b) => comp!(==, Boolean, b, r),
        LispResult::String(s) => comp!(==, String, s, r)
    }

}


comp_op!(apply_less_than, <);
comp_op!(apply_less_than_equals, <=);
comp_op!(apply_greater_than, >);
comp_op!(apply_greater_than_equals, >=);
