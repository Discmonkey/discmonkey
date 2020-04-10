use crate::types::ast::{LispValue};
use crate::types::list::{List, first_token};
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

fn apply_op(list: &List, env: &mut Scope) -> LispResult {
    let op = list.first();


    match op {
        None => LispResult::Nil,
        Some(l) => {
            match l {
                LispValue::Atom(a) => {
                    let results : VecDeque<LispResult> = list
                        .iter()
                        .skip(1)
                        .map( |x| eval_ast(x, env))
                        .collect();

                    let op_name = a.token().get_text();

                    match env.get(op_name) {
                        Some(LispEntry::Func(func)) => {
                           func(results)
                        }

                        _ => LispResult::Error(
                            format!("symbol '{}' not associated with function",
                                    op_name).to_string())
                    }
                }

                _ => LispResult::Error("cannot use list as op to evaluate".to_string())
            }
        }
    }

    // we can potentially mutate the environment -- which is weird

}

/// lisp let rules are somewhat complicated and this method does not do a good job of making them not compliated.
fn apply_let(list: &List, env: &mut Scope) -> LispResult {
    if list.len() != 3 {
        return LispResult::Error("let* two arguments in list".to_string())
    }

    // safe unwrap since we pre-check the length
    match list.get(1).unwrap() {
        LispValue::List(assignment_list) => {

            let mut new_scope = env.new_scope();
            let mut key= "".to_string();
            let mut rvalue;

            for (i, val) in assignment_list.iter().enumerate() {
                if i % 2  == 0 {
                    match val {
                        LispValue::Atom(a) => key = a.token().get_text().clone(),
                        LispValue::List(_l) => {
                            return LispResult::Error("assignment list even argument be string symbol".to_string())
                        }
                    }
                } else {
                    rvalue = eval_ast(val, &mut new_scope);

                    if let LispResult::Error(e) = rvalue {
                        return LispResult::Error(e)
                    }

                    new_scope.set(key.clone(), Value(rvalue));
                }
            }

            let eval = list.get(2).unwrap();


            eval_ast(eval, &mut new_scope)
        }

        LispValue::Atom(_a) => LispResult::Error("first argument to let* must be assignment list".to_string()),
    }
}

fn apply_do(list: &List, env: &mut Scope) -> LispResult {
    list.iter().skip(1).map(|item| {
        eval_ast(item, env)
    }).last().unwrap_or(LispResult::Nil)
}

fn apply_if(list: &List, env: &mut Scope) -> LispResult {
    let length = list.len();

    // (if true)
    if length < 3 {
        return LispResult::Error("if statement needs at least one statement to execute".to_string())
    }

    // if(true (stmt) (stmt) (some extra stuff))
    if length > 4 {
        return LispResult::Error("if statement can have at most two arms".to_string())
    }

    let boolean_flag = eval_ast(&list[1], env);

    match boolean_flag {
        LispResult::Boolean(false) | LispResult::Nil  => {
            match length {
                2 => eval_ast(&list[3], env),
                _ => LispResult::Nil
            }
        },
        // don't evaluate on error
        LispResult::Error(s) => LispResult::Error(s),

        // everything else is considered "truthy"
        _ => {
            eval_ast(&list[2], env)
        }
    }

}

pub fn apply_fn(list: &List, env: &mut Scope) -> LispResult {

    unimplemented!()
}

pub fn eval_list(list: &List, env: &mut Scope) -> LispResult {

    if list.len() == 0 {
        return LispResult::Nil
    }

    let op = first_token(list);

    match op {
        None => LispResult::Error("first token in list must be function or symbol".to_string()),
        Some(t) => {
            match t.get_text().as_str() {
                "def!" => apply_def(list, env),
                "let*" => apply_let(list, env),
                "do" => apply_do(list, env),
                "if" => apply_if(list, env),
                "lambda" => apply_fn(list, env),
                _ => apply_op(list, env)
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