use crate::types::list::List;
use crate::exec::eval::{eval_ast};
use crate::types::env::Scope;
use crate::types::ast::LispValue;


macro_rules! comp {
    ($comp:tt, $type:ident, $left:expr, $right:expr) => {
        if let LispValue::$type(unwrapped_r) = $right {
            LispValue::Boolean($left $comp unwrapped_r)
        } else {
            LispValue::Boolean(false)
        }
    }
}

macro_rules! comp_op {
    ($func:ident, $op:tt) => {
        pub fn $func(list: &List, env: &mut Scope) -> LispValue {
            if list.len() != 3 {
                return LispValue::Error(stringify!($op works with exactly two items to compare).to_string());
            }

            let l = eval_ast(&list[1], env);
            let r = eval_ast(&list[2], env);

            match l {
                LispValue::Error(s) => LispValue::Error(s),
                LispValue::Float(f) => comp!($op, Float, f, r),
                LispValue::Int(i) => comp!($op, Int, i, r),
                _ => LispValue::Boolean(false)

            }
        }

    }
}


pub fn apply_equals(list: &List, env: &mut Scope) -> LispValue {
    if list.len() != 3 {
        return LispValue::Error("equals needs two comparisons".to_string());
    }

    let l = eval_ast(&list[1], env);
    let r = eval_ast(&list[2], env);

    match l {
        LispValue::Error(s) => LispValue::Error(s),
        LispValue::Float(f) => comp!(==, Float, f, r),
        LispValue::Int(i) => comp!(==, Int, i, r),
        LispValue::Boolean(b) => comp!(==, Boolean, b, r),
        LispValue::String(s) => comp!(==, String, s, r),
        _ => LispValue::Boolean(false)
    }

}


comp_op!(apply_less_than, <);
comp_op!(apply_less_than_equals, <=);
comp_op!(apply_greater_than, >);
comp_op!(apply_greater_than_equals, >=);
