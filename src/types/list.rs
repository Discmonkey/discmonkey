use super::ast::{LispValue};
use crate::reader::tokenizer::Token;


pub struct List {
    children: Vec<Box<LispValue>>,
    symbol: Token,
}

impl List {

    pub fn new(token: Token) -> Self {
        List {
            symbol: token,
            children: Vec::new()
        }
    }

    pub fn add_child(&mut self, value: Box<LispValue>) {
        self.children.push(value);
    }

    pub fn print(&self) {
        print!("sexpr ");
        print!("{}", self.symbol.get_text());
        print!(" (");

        for child in &self.children {
            match child.as_ref() {
                LispValue::Atom(a) => a.print(),
                LispValue::List(l) => l.print()
            };
        }

        print!(" )");
    }

    pub fn children(&self) -> &Vec<Box<LispValue>> {
        &(self.children)
    }

    pub fn symbol(&self) -> &Token {
        &self.symbol
    }
}

impl List {

}
