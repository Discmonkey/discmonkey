use super::eval::LispResult;
use std::collections::HashMap;

pub struct Env {
    outer: Option<Box<Env>>,
    data: HashMap<String, LispResult>
}

impl Env {

}