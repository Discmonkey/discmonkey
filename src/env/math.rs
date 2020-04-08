use crate::env::eval::LispResult;
use std::collections::VecDeque;


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
            $deque.into_iter().fold(
                accumulator,  | total, next | $operator(total, next)
            )
        } else {
            LispResult::Error("function called with no arguments".to_string())
        }
    }
}

pub (super) fn add (mut args: VecDeque<LispResult>) -> LispResult {
    gen_reducer!(add_helper, args)
}

pub (super) fn sub (mut args: VecDeque<LispResult>) -> LispResult {
    gen_reducer!(sub_helper, args)
}

pub (super) fn mul (mut args: VecDeque<LispResult>) -> LispResult {
    gen_reducer!(mul_helper, args)
}

pub (super) fn div (mut args: VecDeque<LispResult>) -> LispResult {
    gen_reducer!(div_helper, args)
}
