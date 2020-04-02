use super::ast::{LispValue, LispType};
use crate::reader::tokenizer::Token;

pub struct Atom {
    token: Token
}

impl Atom {
    pub fn new(token: Token) -> Atom {
        Atom{token}
    }
}

impl LispValue for Atom {
    fn print(&self) {
        print!("{}", self.token);
    }

    fn type_(&self) -> LispType {
        LispType::Atom
    }

    fn children(&self) -> &Vec<Box<dyn LispValue>> {
        unimplemented!()
    }

    fn symbol(&self) -> &Token {
        &self.token
    }
}