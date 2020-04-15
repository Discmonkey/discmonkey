use std::collections::HashMap;
use super::math::{add, sub, mul, div};
use std::rc::Rc;
use std::cell::RefCell;
use crate::types::list::List;
use crate::exec::core_recursive::{apply_do, apply_let, apply_if, create_func, apply_def};
use crate::exec::core_comparison::{apply_equals, apply_greater_than, apply_greater_than_equals, apply_less_than, apply_less_than_equals};

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

macro_rules! to_func {
    ($f:expr) => {
        LispResult::Function(Rc::new($f))
    }
}

macro_rules! insert {
    ($m:expr, $key:literal, $func:ident) => {
         $m.insert($key.to_string(), to_func!($func));
    }
}

impl Scope {

    // the math functions live at the base scope level, which also means
    // that they can actually be pretty freely redefined.
    pub fn new() -> Self {
        let mut map: HashMap<String, LispResult> = HashMap::new();
        insert!(map, "+", add);
        insert!(map, "-", sub);
        insert!(map, "/", div);
        insert!(map, "*", mul);

        insert!(map, "do", apply_do);
        insert!(map, "let*", apply_let);
        insert!(map, "if", apply_if);
        insert!(map, "lambda", create_func);
        insert!(map, "def!", apply_def);

        insert!(map, "=", apply_equals);
        insert!(map, ">", apply_greater_than);
        insert!(map, "<", apply_less_than);
        insert!(map, "<=", apply_less_than_equals);
        insert!(map, ">=", apply_greater_than_equals);

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