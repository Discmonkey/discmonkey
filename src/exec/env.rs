use std::collections::HashMap;
use super::math::{add, sub, mul, div};
use std::rc::Rc;
use std::cell::RefCell;
use crate::types::list::List;
use crate::exec::core::{apply_do, apply_let, apply_if, create_func, apply_def};


pub type Lambda = Rc<dyn Fn(&List, &mut Scope) -> LispResult>;

#[derive(Clone)]
pub enum LispResult {
    Int(i64),
    Float(f64),
    Nil,
    Boolean(bool),
    Function(Lambda),
    Error(String),
}

pub struct Scope {
    current: Rc<RefCell<Env>>
}

pub struct Env {
    data: HashMap<String, LispResult>,
    outer: Option<Rc<RefCell<Env>>>
}

impl Env {
    pub fn get(&self, key: &String) -> Option<LispResult> {
        // safe to unwrap here since find will insure that the key exists.
        self.data.get(key).map( | ref_result| {
            ref_result.clone()
        })
    }

    pub fn insert(&mut self, key: String, entry: LispResult) {
        self.data.insert(key, entry);
    }
}

impl Scope {

    // the math functions live at the base scope level, which also means
    // that they can actually be pretty freely redefined.
    pub fn new() -> Self {
        let mut map: HashMap<String, LispResult> = HashMap::new();
        map.insert("+".to_string(), LispResult::Function(Rc::new(add)));
        map.insert("-".to_string(), LispResult::Function(Rc::new(sub)));
        map.insert("*".to_string(), LispResult::Function(Rc::new(mul)));
        map.insert("/".to_string(), LispResult::Function(Rc::new(div)));
        map.insert("do".to_string(), LispResult::Function(Rc::new(apply_do)));
        map.insert("let*".to_string(), LispResult::Function(Rc::new(apply_let)));
        map.insert("if".to_string(), LispResult::Function(Rc::new(apply_if)));
        map.insert("lambda".to_string(), LispResult::Function(Rc::new(create_func)));
        map.insert("def!".to_string(), LispResult::Function(Rc::new(apply_def)));
        let env = Rc::new(RefCell::new(Env {
            data: map,
            outer: None
        }));

        Scope{
            current: env
        }
    }

    // make a clone of what we have, and pass it as the parent environment of the child
    // which will then return.
    pub fn new_scope(&self) -> Self {

        let env = Rc::new(RefCell::new(Env {
            data: HashMap::new(),
            outer: Some(self.current.clone()),
        }));

        Self {
            current: env
        }
    }


    pub fn set(&mut self, key: String, entry: LispResult) {
        // one of many places were as_ref is used --
        // this *tricks* the Rc to correctly delegate the borrow_mut method call to the underlying refcell
        // otherwise everything is a mess.
        self.current.as_ref().borrow_mut().insert(key, entry );
    }

    pub fn find(&self, key: &String) -> Option<Rc<RefCell<Env>>> {
        let mut env = Some(self.current.clone());

        while let Some(e) = env {
            if e.as_ref().borrow().data.contains_key(key) {
                return Some(e.clone());
            } else {
                env = e.as_ref().borrow().outer.clone();
            }
        }

        None
    }

    pub fn get(&self, key: &String) -> Option<LispResult> {
        match self.find(key) {
            None => None,
            Some(e) => e.as_ref().borrow().get(key)
        }
    }
}