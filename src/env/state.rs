use std::collections::HashMap;
use crate::env::eval::LispResult;
use std::collections::VecDeque;
use super::math::{add, sub, mul, div};
use std::rc::Rc;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;

type Operator = fn(args: VecDeque<LispResult>) -> LispResult;

#[derive(Clone)]
pub enum LispEntry {
    Func(Operator),
    Value(LispResult)
}

pub struct Scope {
    current: Rc<RefCell<Env>>
}

pub struct Env {
    data: HashMap<String, LispEntry>,
    outer: Option<Rc<RefCell<Env>>>
}

impl Env {
    pub fn get(&self, key: &String) -> Option<LispEntry> {
        // safe to unwrap here since find will insure that the key exists.
        self.data.get(key).map( | ref_result| {
            ref_result.clone()
        })
    }

    pub fn insert(&mut self, key: String, entry: LispEntry) {
        self.data.insert(key, entry);
    }
}

impl Scope {

    // the math functions live at the base scope level, which also means
    // that they can actually be pretty freely redefined.
    pub fn new() -> Self {
        let mut map: HashMap<String, LispEntry> = HashMap::new();

        map.insert("+".to_string(), LispEntry::Func(add));
        map.insert("-".to_string(), LispEntry::Func(sub));
        map.insert("*".to_string(), LispEntry::Func(mul));
        map.insert("/".to_string(), LispEntry::Func(div));

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


    pub fn set(&mut self, key: String, entry: LispEntry) {
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

    pub fn get(&self, key: &String) -> Option<LispEntry> {
        match self.find(key) {
            None => None,
            Some(e) => e.as_ref().borrow().get(key)
        }
    }
}