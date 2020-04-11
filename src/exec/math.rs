
use std::collections::VecDeque;
use crate::types::list::List;
use crate::exec::env::{Scope, LispResult};
use crate::exec::eval::{eval_ast};


macro_rules! operate {
    ($op:tt, $a:expr, $b:expr) => {
        match $a {
            LispResult::Error(message) => LispResult::Error(message),

            LispResult::Int(i) => match $b {
                LispResult::Error(message) => LispResult::Error(message),
                LispResult::Int(i2) => LispResult::Int(i $op i2),
                LispResult::Float(f2) => LispResult::Float(i as f64 $op f2),
                _ => LispResult::Error("incompatible types".to_string())
            },

            LispResult::Float(f) => match $b {
                LispResult::Error(message) => LispResult::Error(message),
                LispResult::Int(i2) => LispResult::Float(f $op i2 as f64),
                LispResult::Float(f2) => LispResult::Float(f $op f2),
                 _ => LispResult::Error("incompatible types".to_string())

            },

            _ => LispResult::Error("incompatible types".to_string())

        }
    };
}

fn add_helper(a: LispResult, b: LispResult) -> LispResult {
    operate!(+, a, b)
}

fn sub_helper(a: LispResult, b: LispResult) -> LispResult {
    operate!(-, a, b)
}

fn mul_helper(a: LispResult, b: LispResult) -> LispResult {
    operate!(*, a, b)
}

fn div_helper(a: LispResult, b: LispResult) -> LispResult {
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
            LispResult::Error("function called with no arguments".to_string())
        }
    }
}

fn prepare_args(args: &List, env: &mut Scope) -> VecDeque<LispResult> {
    args.items()
        .iter()
        .skip(1)
        .map(|x| eval_ast(x, env))
        .collect()
}



pub (super) fn add (args: &List, mut env: &mut Scope) -> LispResult {
    let mut mapped = prepare_args(args, &mut env);
    gen_reducer!(add_helper, mapped)
}

pub (super) fn sub (args: &List, env: &mut Scope) -> LispResult {
    let mut mapped = prepare_args(args, env);
    gen_reducer!(sub_helper, mapped)
}

pub (super) fn mul (args: &List, env: &mut Scope) -> LispResult {
    let mut mapped = prepare_args(args, env);
    gen_reducer!(mul_helper, mapped)
}

pub (super) fn div (args: &List, env: &mut Scope) -> LispResult {
    let mut mapped = prepare_args(args, env);
    gen_reducer!(div_helper, mapped)
}
