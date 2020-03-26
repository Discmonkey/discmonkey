mod atom;
mod list;
use super::reader::tokenizer::Tokens;

enum LispType {
    List,
    Atom,
}

trait LispValue {
    fn print(&self);
    fn type_(&self) -> LispType;
    fn children(&self) -> &Vec<Box<dyn LispValue>>;
    fn add_child(&mut self, new_node: Box<dyn LispValue>);
}


type Link = Option<Box<dyn LispValue>>;

struct AST {
    head: Link
}

impl AST {

    pub fn build(tokens: Tokens) -> Self {

    }

}

