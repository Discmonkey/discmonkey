use crate::types::ast::{LispValue};
use crate::types::list::List;
use crate::types::atom::Atom;
use crate::env::state::{Scope, LispEntry};
use std::fmt;
use std::collections::VecDeque;
use crate::env::state::LispEntry::Value;

#[derive(Clone)]
pub enum LispResult {
    Int(i64),
    Float(f64),
    Nil,
    Boolean(bool),
    Error(String)
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

pub fn eval_ast(root: &Box<LispValue>, mut env: &mut Scope) -> LispResult {
    match root.as_ref() {
        LispValue::List(list) => eval_list(&list, &mut env),
        LispValue::Atom(atom) => eval_symbol(&atom, env)
    }
}

fn apply_def(list: &List,  env: &mut Scope) -> LispResult {
    let children = list.children();

    if children.len() != 2 {
        return LispResult::Error("incorrect number of args for definition".to_string());
    }

    match children[0].as_ref() {
        LispValue::Atom(a) => {

            // need to clone to insert into map
            let key = a.token().get_text().clone();
            let value = eval_ast(&children[1], env);

            env.set(key.clone(), LispEntry::Value(value.clone()));

            value
        }

        LispValue::List(l) => LispResult::Error("first argument to def! must be a string".to_string())
    }
}

fn apply_op(op: &String, list: &List, env: &mut Scope) -> LispResult {

    // we can potentially mutate the environment -- which is weird
    let results : VecDeque<LispResult> = list
        .children()
        .iter()
        .map( |x| eval_ast(x, env))
        .collect();

    let f = env.get(op);

    match f {
        None => LispResult::Error(format!("no defined op: {}", op).to_string()),

        Some(maybe_func) => {
            match maybe_func {
                LispEntry::Func(func) => func(results),
                _ => LispResult::Error(
                    format!("symbol '{}' not associated with function", op).to_string()
                )
            }
        }
    }
}

/// lisp let rules are somewhat complicated and this method does not do a good job of making them not compliated.
fn apply_let(list: &List, env: &mut Scope) -> LispResult {
    let children = list.children();

    if children.len() != 2 {
        return LispResult::Error("let* requires at most two arguments in list".to_string())
    }

    // safe unwrap since we pre-check the length
    // on the other hand
    match children.get(0).unwrap().as_ref() {
        LispValue::Atom(a) => LispResult::Error("first argument to let* must be assignment list".to_string()),
        LispValue::List(assignment_list) => {
            let assignment_pairs = assignment_list.children();

            let mut new_scope = env.new_scope();
            let mut key = assignment_list.symbol().get_text();

            for (i, val) in assignment_pairs.iter().enumerate() {
                match i % 2 {
                    0 => {
                        let rvalue = eval_ast(val, &mut new_scope);
                        new_scope.set(key.clone(), Value(rvalue));
                    }
                    _ => {
                        match val.as_ref() {
                            LispValue::Atom(a) => key = a.token().get_text(),
                            LispValue::List(l) => {
                                return LispResult::Error("assignment list even argument be string symbol".to_string())
                            }
                        }
                    }
                }
            }

            let eval = children.get(1).unwrap();


            eval_ast(eval, &mut new_scope)
        }
    }
}



pub fn eval_list(list: &List, env: &mut Scope) -> LispResult {
    let op = list.symbol().get_text(); // *, +, /, etc

    match op.as_str() {
        "def!" => apply_def(list, env),
        "let*" => apply_let(list, env),
        "do" => apply_do(list, env),
        "if" => apply_if(list, env),
        _ => apply_op(op, list, env)
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
    use crate::env::state;
    use crate::env::state::Scope;

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