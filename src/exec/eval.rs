use crate::types::ast::{LispValue};
use crate::types::list::{List};
use crate::types::atom::Atom;
use crate::exec::env::{Scope, LispResult};
use std::fmt;
use crate::reader::tokenizer::TokenType;


impl fmt::Display for LispResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            LispResult::Int(i) => write!(f, "{}", i),
            LispResult::Float(float) => write!(f, "{}", float),
            LispResult::Error(message) => write!(f, "error - {}", message),
            LispResult::Boolean(b) => write!(f, "{}", b),
            LispResult::Nil => write!(f, "nil"),
            LispResult::Function(_l) => write!(f, "#<lambda>"),
            LispResult::String(s) => write!(f, "\"{}\"", s)
        }

    }
}

pub fn eval_ast(root: &LispValue, mut env: &mut Scope) -> LispResult {
    match root {
        LispValue::List(list) => eval_list(&list, &mut env),
        LispValue::Atom(atom) => eval_symbol(&atom, env)
    }
}

pub fn eval_list(list: &List, env: &mut Scope) -> LispResult {

    if list.len() == 0 {
        return LispResult::Nil
    }

    let op = eval_ast(&list[0], env);

    match op {
        LispResult::Function(f) => f(list, env),
        LispResult::Error(s) => LispResult::Error(s),
        _ => op
    }
}

fn convert_string(s: &String) -> LispResult {
    if !s.starts_with("\"")|| !s.ends_with("\"") {
        LispResult::Error("malformatted string".to_string())
    } else {
        let mut copy = s.clone();
        copy.pop();
        copy.remove(0);

        LispResult::String(copy)
    }
}

pub fn eval_symbol(atom: &Atom, env: &Scope) -> LispResult {
    if atom.token().get_type() == TokenType::String {
        return convert_string(atom.token().get_text())
    }


    let string = atom.token().get_text();
    // check if this symbol is defined
    // note that this means our language currently allows for redefinitions
    if let Some(result) = env.get(string) {
        return result.clone()
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