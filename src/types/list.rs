use super::ast::{LispValue, LispType};

pub struct List {
    children: Vec<Box<dyn LispValue>>
}

impl List {

    pub fn new() -> Self {
        List {
            children: Vec::new()
        }
    }
}

impl LispValue for List {
    fn print(&self) {
        print!("sexpr ( ");

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

    fn add_child(&mut self, new_node: Box<dyn LispValue>) {
        self.children.push(new_node)
    }
}
