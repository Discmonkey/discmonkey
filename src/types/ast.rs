use super::reader::parser::Parser;
use super::atom::Atom;
use super::list::List;
pub enum LispType {
    List,
    Atom,
}

pub trait LispValue {
    fn print(&self);
    fn type_(&self) -> LispType;
    fn children(&self) -> &Vec<Box<dyn LispValue>>;
    fn add_child(&mut self, new_node: Box<dyn LispValue>);
}

pub type Link = Option<Box<dyn LispValue>>;

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
//
    fn read_form(&self,  parser: &mut Parser) -> Option<Box<dyn LispValue>> {

        let maybe_next_character = parser.peek();

        if maybe_next_character.is_none() {
            return None
        }

        match maybe_next_character.unwrap().get_text().as_str() {
            "(" => {
                parser.next(); // consume token

                let mut parent_node = Box::new(List::new());

                while let Some(val) = self.read_form(parser) {
                    parent_node.add_child(val)
                }

                Some(parent_node as Box<dyn LispValue>)
            }

            ")" => {
                None
            }

            _ => self.read_atom(parser)
        }
    }

    fn read_atom(&self, parser: &mut Parser) -> Option<Box<dyn LispValue>> {
        parser.next().map(| val | {
            Box::new(Atom::new(val.clone())) as Box<dyn LispValue>
        })
    }

}
