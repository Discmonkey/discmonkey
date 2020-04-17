
use std::collections::VecDeque;
use crate::types::list::List;
use crate::types::ast::LispValue;
use crate::exec::eval::{eval_ast};
use crate::types::env::Scope;


macro_rules! operate {
    ($op:tt, $a:expr, $b:expr) => {
        match $a {
            LispValue::Error(message) => LispValue::Error(message),

            LispValue::Int(i) => match $b {
                LispValue::Error(message) => LispValue::Error(message),
                LispValue::Int(i2) => LispValue::Int(i $op i2),
                LispValue::Float(f2) => LispValue::Float(i as f64 $op f2),
                _ => LispValue::Error("incompatible types".to_string())
            },

            LispValue::Float(f) => match $b {
                LispValue::Error(message) => LispValue::Error(message),
                LispValue::Int(i2) => LispValue::Float(f $op i2 as f64),
                LispValue::Float(f2) => LispValue::Float(f $op f2),
                 _ => LispValue::Error("incompatible types".to_string())

            },

            _ => LispValue::Error("incompatible types".to_string())

        }
    };
}

fn add_helper(a: LispValue, b: LispValue) -> LispValue {
    operate!(+, a, b)
}

fn sub_helper(a: LispValue, b: LispValue) -> LispValue {
    operate!(-, a, b)
}

fn mul_helper(a: LispValue, b: LispValue) -> LispValue {
    operate!(*, a, b)
}

fn div_helper(a: LispValue, b: LispValue) -> LispValue {
    operate!(/, a, b)
}

macro_rules! gen_reducer {
    ($operator:ident, $deque:expr) => {

        if let Some(accumulator) = $deque.pop_front() {
            $deque
                .into_iter()
                .fold(
                    accumulator,  | total, next | $operator(total, next)
                )
        } else {
            LispValue::Error("function called with no arguments".to_string())
        }
    }
}

fn prepare_args(args: &List, env: &mut Scope) -> VecDeque<LispValue> {
    args.items()
        .iter()
        .skip(1)
        .map(|x| eval_ast(x, env))
        .collect()
}



pub fn add (args: &List, mut env: &mut Scope) -> LispValue {
    let mut mapped = prepare_args(args, &mut env);
    gen_reducer!(add_helper, mapped)
}

pub fn sub (args: &List, env: &mut Scope) -> LispValue {
    let mut mapped = prepare_args(args, env);
    gen_reducer!(sub_helper, mapped)
}

pub fn mul (args: &List, env: &mut Scope) -> LispValue {
    let mut mapped = prepare_args(args, env);
    gen_reducer!(mul_helper, mapped)
}

pub fn div (args: &List, env: &mut Scope) -> LispValue {
    let mut mapped = prepare_args(args, env);
    gen_reducer!(div_helper, mapped)
}
