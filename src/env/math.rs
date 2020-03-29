use std::collections::HashMap;
use crate::reader::tokenizer::Token;
use crate::types::ast::LispValue;


pub struct MathEnv {
    map: HashMap<String, fn(Vec<Box<dyn LispValue>>) -> Box<dyn LispValue>>
}

impl MathEnv {

    pub fn new() -> Self {

    }
}