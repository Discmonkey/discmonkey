use super::ast::{LispValue, LispType};
use crate::reader::tokenizer::Token;


pub struct List {
    children: Vec<Box<dyn LispValue>>,
    symbol: Token,
}

impl List {

    pub fn new(token: Token) -> Self {
        List {
            symbol: token,
            children: Vec::new()
        }
    }

    pub fn add_child(&mut self, value: Box<dyn LispValue>) {
        self.children.push(value);
    }
}

impl LispValue for List {
    fn print(&self) {
        print!("sexpr ");
        print!("{}", self.symbol.get_text());
        print!(" (");

        for child in &self.children {
            child.print();
            print!(", ");
        }

        print!(" )");
    }

    fn type_(&self) -> LispType {
        LispType::List
    }

    fn children(&self) -> &Vec<Box<dyn LispValue>> {
        &(self.children)
    }

    fn symbol(&mut self) -> &Token {
        unimplemented!()
    }
}
