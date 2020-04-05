use crate::types::ast::{LispValue, LispType};
use crate::env::state::{Env, LispEntry};
use std::fmt;
use std::collections::VecDeque;

#[derive(Clone)]
pub enum LispResult {
    Int(i32),
    Float(f32),
    Error
}

impl fmt::Display for LispResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            LispResult::Int(i) => write!(f, "{}", i),
            LispResult::Float(float) => write!(f, "{}", float),
            LispResult::Error => write!(f, "error")
        }

    }
}

pub fn eval_ast(root: &Box<dyn LispValue>, mut env: &mut Env) -> LispResult {
    match root.type_() {
        LispType::List => eval_list(root, &mut env),
        LispType::Atom => eval_symbol(root, env)
    }
}

fn apply_set(list: &Box<dyn LispValue>,  env: &mut Env) -> LispResult {
    let children = list.children();

    match children.len() {
        2 => {
            let key = children[0].symbol().get_text();
            let value = eval_ast(&children[1], env);

            env.set(key.clone(), LispEntry::Value(value.clone()));

            value
        }
        _ => LispResult::Error
    }
}

fn apply_op(op: &String, list: &Box<dyn LispValue>, env: &mut Env) -> LispResult {

    // we can potentially mutate the environment -- which is weird
    let results : VecDeque<LispResult> = list
        .children()
        .iter()
        .map( |x| eval_ast(x, env))
        .collect();

    let f = env.get(op);

    match f {
        None => LispResult::Error,

        Some(maybe_func) => {
            match maybe_func {
                LispEntry::Func(func) => func(results),
                _ => LispResult::Error
            }
        }
    }
}

pub fn eval_list(list: &Box<dyn LispValue>, env: &mut Env) -> LispResult {
    let op = list.symbol().get_text(); // *, +, /, etc

    match op.as_str() {
        "def!" => apply_set(list, env),
        _ => apply_op(op, list, env)
    }

}

pub fn eval_symbol(atom: &Box<dyn LispValue>, env: &Env) -> LispResult {
    let string = atom.symbol().get_text();

    if let Some(LispEntry::Value(res)) = env.get(string) {
        return res.clone()
    }
    // will need to improve this method of resolving as we get more types.
    match string.parse::<i32>() {
        Ok(i) => LispResult::Int(i),

        _ => match string.parse::<f32>() {
            Ok(f) => LispResult::Float(f),
            _ => LispResult::Error
        }
    }
}


#[cfg(test)]
mod test {
    use super::{eval_symbol, LispResult};
    use crate::reader::tokenizer::{Token, TokenType};
    use crate::types::atom::Atom;
    use crate::types::ast::LispValue;
    use crate::env::state;

    #[test]
    fn symbol() {
        let test_token_float = Token::new("3.14".to_string(), TokenType::Symbol);
        let test_token_int = Token::new("3".to_string(), TokenType::Symbol);
        let test_token_gibberish = Token::new("1984.38471jf".to_string(), TokenType::Symbol);

        let lisp_value_float = Box::new(Atom::new(test_token_float));
        let lisp_value_int = Box::new(Atom::new(test_token_int));
        let lisp_val_bad = Box::new(Atom::new(test_token_gibberish));

        match eval_symbol(&(lisp_value_float as Box<dyn LispValue>)) {
            LispResult::Float(v) => assert_eq!(v, 3.14),
            _ => assert!(false)
        }

        match eval_symbol(&(lisp_value_int as Box<dyn LispValue>)) {
            LispResult::Int(v) => assert_eq!(v, 3),
            _ => assert!(false)
        }

        match eval_symbol(&(lisp_val_bad as Box<dyn LispValue>)) {
            LispResult::Error => assert!(true),
            _ => assert!(false)
        }



    }
}