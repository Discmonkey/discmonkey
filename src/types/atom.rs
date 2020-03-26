use super::LispValue;
use super::LispType;

struct Atom {
    val: String
}

impl LispValue for Atom {
    fn print(&self) {
        print!("{}", self.val);
    }

    fn type_(&self) -> LispType {
        LispType::Atom
    }

    fn children(&self) -> &Vec<Box<dyn LispValue>> {
        unimplemented!()
    }

    fn add_child(&mut self, new_node: Box<dyn LispValue>) {
        unimplemented!()
    }
}