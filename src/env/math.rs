use std::collections::HashMap;
use crate::reader::tokenizer::Token;
use crate::types::ast::LispValue;


type Operator = fn(Vec<Box<dyn LispValue>>) -> Box<dyn LispValue>;

pub struct MathEnv {
    map: HashMap<String, Operator>
}

fn add(args: Vec<Box<dyn LispValue>>) -> Box<dyn LispValue> {
    unimplemented!()
}

fn sub(args: Vec<Box<dyn LispValue>>) -> Box<dyn LispValue> {
    unimplemented!()
}

fn mul(args: Vec<Box<dyn LispValue>>) -> Box<dyn LispValue> {
    unimplemented!()
}

fn div(args: Vec<Box<dyn LispValue>>) -> Box<dyn LispValue> {
    unimplemented!()
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

    pub fn apply(val: Box<dyn LispValue>) {

    }
}