use std::collections::HashMap;
use crate::env::eval::LispResult;
use std::collections::VecDeque;


type Operator = fn(args: VecDeque<LispResult>) -> LispResult;

pub enum LispEntry {
    Func(Operator),
    Value(LispResult)
}

pub struct Env {
    data: HashMap<String, LispEntry>,
    outer: Option<Box<Env>>
}

macro_rules! operate {
    ($op:tt, $a:expr, $b:expr) => {
        match $a {
            LispResult::Error => LispResult::Error,

            LispResult::Int(i) => match $b {
                LispResult::Error => LispResult::Error,
                LispResult::Int(i2) => LispResult::Int(i $op i2),
                LispResult::Float(f2) => LispResult::Float(i as f32 $op f2)
            }

            LispResult::Float(f) => match $b {
                LispResult::Error => LispResult::Error,
                LispResult::Int(i2) => LispResult::Float(f $op i2 as f32),
                LispResult::Float(f2) => LispResult::Float(f $op f2)
            }

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
            LispResult::Error
        }
    }
}

fn add (mut args: VecDeque<LispResult>) -> LispResult {
    gen_reducer!(add_helper, args)
}

fn sub (mut args: VecDeque<LispResult>) -> LispResult {
    gen_reducer!(sub_helper, args)
}

fn mul (mut args: VecDeque<LispResult>) -> LispResult {
    gen_reducer!(mul_helper, args)
}

fn div (mut args: VecDeque<LispResult>) -> LispResult {
    gen_reducer!(div_helper, args)
}


impl Env {

    pub fn new() -> Self {
        let mut map: HashMap<String, LispEntry> = HashMap::new();

        map.insert("+".to_string(), LispEntry::Func(add));
        map.insert("-".to_string(), LispEntry::Func(sub));
        map.insert("*".to_string(), LispEntry::Func(mul));
        map.insert("/".to_string(), LispEntry::Func(div));

        Self {
            data: map,
            outer: None
        }
    }

    pub fn set(&mut self, key: String, entry: LispEntry) {
        self.data.insert(key, entry );
    }

    pub fn find(&self, key: &String) -> Option<&Self> {
        if self.data.contains_key(key) {
            Some(self)
        } else {
            match &self.outer {
                None => None,
                Some(env) => env.find(key)
            }
        }
    }

    pub fn get(&self, key: &String) -> Option<&LispEntry> {
        // safe to unwrap here since find will insure that the key exists.
        self.find(key).map(| env | {
            env.data.get(key).unwrap()
        })
    }


}