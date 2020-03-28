use super::ast::{LispValue, LispType};
use super::super::reader::tokenizer::Token;

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

    fn add_child(&mut self, _new_node: Box<dyn LispValue>) {
        unimplemented!()
    }
}