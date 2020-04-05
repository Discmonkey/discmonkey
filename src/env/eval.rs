use crate::types::ast::{LispValue, LispType};
use crate::env::math::MathEnv;
use std::fmt;
use std::collections::VecDeque;

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

pub fn eval_ast(root: &Box<dyn LispValue>, env: &MathEnv) -> LispResult {
    match root.type_() {
        LispType::List => eval_list(root, &env),
        LispType::Atom => eval_symbol(root)
    }
}

pub fn eval_list(list: &Box<dyn LispValue>, env: &MathEnv) -> LispResult {
    let op = list.symbol().get_text(); // *, +, /, etc

    let f = env.get_func(op);

    match f {
        None => LispResult::Error,
        Some(func) => {

            // lets convert everything to results
            let mut results : VecDeque<LispResult> = list
                .children()
                .iter()
                .map(|x| eval_ast(x, env))
                .collect();

            // then lets apply our list func to them
            if let Some(accumulator) = results.pop_front() {
                results.into_iter().fold(
                    accumulator,  | total, next | func(total, next)
                )
            } else {
                LispResult::Error
            }
        }
    }
}

pub fn eval_symbol(atom: &Box<dyn LispValue>) -> LispResult {
    let string = atom.symbol().get_text();

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