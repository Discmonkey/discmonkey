use std::collections::HashMap;
use crate::env::eval::LispResult;


type Operator = fn(LispResult, LispResult) -> LispResult;

pub struct MathEnv {
    map: HashMap<String, Operator>
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


fn add(a: LispResult, b: LispResult) -> LispResult {
    operate!(+, a, b)
}

fn sub(a: LispResult, b: LispResult) -> LispResult {
    operate!(-, a, b)
}

fn mul(a: LispResult, b: LispResult) -> LispResult {
    operate!(*, a, b)
}

fn div(a: LispResult, b: LispResult) -> LispResult {
    operate!(/, a, b)
}


impl MathEnv {

    pub fn new() -> Self {
        let mut map: HashMap<String, Operator> = HashMap::new();

        map.insert("+".to_string(), add);
        map.insert("-".to_string(), sub);
        map.insert("*".to_string(), mul);
        map.insert("/".to_string(), div);

        Self {
            map
        }
    }

    pub fn get_func(&self, key: &String) -> (Option<&Operator>) {
        self.map.get(key)
    }
}