use super::LispValue;
use super::LispType;

struct List {
    children: Vec<Box<dyn LispValue>>
}

impl LispValue for List {
    fn print(&self) {
        print!("( ");

        for child in &self.children {
            child.print();
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
