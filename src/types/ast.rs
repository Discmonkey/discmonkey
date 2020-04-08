use super::reader::parser::Parser;
use super::atom::Atom;
use super::list::List;
use crate::reader::tokenizer::Token;


pub enum LispValue {
    List(List),
    Atom(Atom)
}

pub type Link = Option<Box<LispValue>>;

pub struct AST {
    root: Link,
}

impl AST {

    pub fn new() -> Self {
        AST {
            root: None
        }
    }

    pub fn build(&mut self, parser: &mut Parser){
        self.root = self.read_form(parser);
    }

    fn read_form(&self,  parser: &mut Parser) -> Option<Box<LispValue>> {

        let maybe_next_character = parser.peek();

        if maybe_next_character.is_none() {
            return None
        }

        match maybe_next_character.unwrap().get_text().as_str() {
            "(" => {
                parser.next(); // consume token for "("

                let first_item_maybe = parser.next();

                // is this the correct behavior?
                // what is an () in Lisp?
                if first_item_maybe.is_none() {
                    return None
                }

                // the first item in a list defines its behavior.
                let first_item = first_item_maybe.unwrap();

                let mut parent_node = List::new(first_item);

                while let Some(val) = self.read_form(parser) {
                    parent_node.add_child(val);

                    if parser.peek().unwrap().get_text().as_str() == ")" {
                        parser.next();
                        break;
                    }
                }

                Some(Box::new(LispValue::List(parent_node)))
            }

            ")" => {
                None
            }

            _ => self.read_atom(parser)
        }
    }

    fn read_atom(&self, parser: &mut Parser) -> Option<Box<LispValue>> {
        parser.next().map(| val | {
            Box::new(LispValue::Atom(Atom::new(val.clone())))
        })
    }

    pub fn root(&self) -> &Link {
        &self.root
    }

    pub fn print(&self) {
        match &self.root {
            None => println!("empty tree"),
            Some(val) => match val.as_ref() {
                LispValue::List(l) => l.print(),
                LispValue::Atom(l) => l.print()
            }
        };
    }



}
