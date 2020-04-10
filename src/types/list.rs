use super::ast::{LispValue};
use crate::reader::tokenizer::Token;


pub type List = Vec<LispValue>;

pub fn first_token(list: &List) -> Option<&Token> {
    match list.first() {
        Some(LispValue::Atom(a)) => {
            Some(a.token())
        }
        _  => None,

    }
}