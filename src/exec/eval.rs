use crate::types::ast::{LispValue};
use crate::types::list::{List};
use crate::types::atom::Atom;
use crate::exec::env::{Scope, LispEntry};
use crate::exec::core::{apply_let, apply_do, apply_if};
use std::fmt;
use std::collections::VecDeque;

#[derive(Clone)]
pub enum LispResult {
    Int(i64),
    Float(f64),
    Nil,
    Boolean(bool),
    Error(String),
    Closure()
}

impl fmt::Display for LispResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            LispResult::Int(i) => write!(f, "{}", i),
            LispResult::Float(float) => write!(f, "{}", float),
            LispResult::Error(message) => write!(f, "{}", message),
            LispResult::Boolean(b) => write!(f, "{}", b),
            LispResult::Nil => write!(f, "nil")
        }

    }
}

pub fn eval_ast(root: &LispValue, mut env: &mut Scope) -> LispResult {
    match root {
        LispValue::List(list) => eval_list(&list, &mut env),
        LispValue::Atom(atom) => eval_symbol(&atom, env)
    }
}

fn apply_def(list: &List,  env: &mut Scope) -> LispResult {
    if list.len() != 3 {
        return LispResult::Error("incorrect number of args for definition".to_string());
    }

    match &list[1] {
        LispValue::Atom(a) => {

            // need to clone to insert into map
            let key = a.token().get_text().clone();
            let value = eval_ast(&list[2], env);

            env.set(key.clone(), LispEntry::Value(value.clone()));

            value
        }

        LispValue::List(_l) => LispResult::Error("first argument to def! must be a symbol".to_string())
    }
}

pub fn apply_fn(list: &List, env: &mut Scope) -> LispResult {

    unimplemented!()
}

pub fn eval_list(list: &List, env: &mut Scope) -> LispResult {

    if list.len() == 0 {
        return LispResult::Nil
    }

    let op = list.first_token();



    match op {
        None => LispResult::Error("first token in list must be function or symbol".to_string()),
        Some(t) => {
            let maybe_func = env.get(t.get_text());

            match maybe_func {
                Some(LispEntry::Func(f)) => f(list, env),
                _ => LispResult::Error(format!("no function with identifier: {}", t.get_text()))
            }
        }
    }
}

pub fn eval_symbol(atom: &Atom, env: &Scope) -> LispResult {
    let string = atom.token().get_text();

    // check if this symbol is defined
    // note that this means our language currently allows for redefinitions
    if let Some(LispEntry::Value(res)) = env.get(string) {
        return res.clone()
    }

    // in order try nil, true, false and finally check for floats and ints
    match string.as_str() {
        "nil" => LispResult::Nil,
        "true" => LispResult::Boolean(true),
        "false" => LispResult::Boolean(false),
        _ => {
            match string.parse::<i64>() {
                Ok(i) => LispResult::Int(i),

                _ => match string.parse::<f64>() {
                    Ok(f) => LispResult::Float(f),
                    _ => LispResult::Error(format!("could not parse symbol: {}", string))
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::{eval_symbol, LispResult};
    use crate::reader::tokenizer::{Token, TokenType};
    use crate::types::atom::Atom;
    use crate::types::ast::LispValue;
    use crate::exec::env::Scope;

    #[test]
    fn test_ints_and_floats() {
        let env = Scope::new();

        let test_token_float = Token::new("3.14".to_string(), TokenType::Symbol);
        let test_token_int = Token::new("3".to_string(), TokenType::Symbol);
        let test_token_gibberish = Token::new("1984.38471jf".to_string(), TokenType::Symbol);

        let lisp_value_float =Atom::new(test_token_float);
        let lisp_value_int = Atom::new(test_token_int);
        let lisp_val_bad = Atom::new(test_token_gibberish);

        match eval_symbol(&lisp_value_float, &env) {
            LispResult::Float(v) => assert_eq!(v, 3.14),
            _ => assert!(false)
        }

        match eval_symbol(&lisp_value_int, &env) {
            LispResult::Int(v) => assert_eq!(v, 3),
            _ => assert!(false)
        }

        match eval_symbol(&(lisp_val_bad), &env) {
            LispResult::Error(_) => assert!(true),
            _ => assert!(false)
        }

    }


    #[test]
    fn true_false() {
        let env = Scope::new();
        let test_token_false = Atom::new(Token::new("false".to_string(), TokenType::Symbol));
        let test_token_true = Atom::new(Token::new("true".to_string(), TokenType::Symbol));

        match eval_symbol(&(test_token_false), &env) {
            LispResult::Boolean(v) => assert!(!v),
            _ => assert!(false)
        }

        match eval_symbol(&(test_token_true), &env) {
            LispResult::Boolean(v) => assert!(v),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_nil() {
        let env = Scope::new();

        let test_nil_token = Atom::new(Token::new("nil".to_string(), TokenType::Symbol));

        match eval_symbol(&test_nil_token, &env) {
            LispResult::Nil => assert!(true),
            _ => assert!(false)
        }
    }
}