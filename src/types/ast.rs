use super::reader::parser::Parser;
use super::unit::Unit;
use super::list::List;
use crate::types::env::Scope;
use std::rc::Rc;
use std::fmt::{Display, Formatter, Result};
use std::cell::RefCell;

pub type Lambda = Rc<dyn Fn(&List, &mut Scope) -> LispValue>;

#[derive(Clone)]
pub enum LispValue {
    List(List),
    Unit(Unit),
    Int(i64),
    Float(f64),
    Nil,
    Boolean(bool),
    Function(Lambda),
    Error(String),
    String(String),
    // we need interior mutability here since we can potentially "swap" and "mutate" the contents of an atom
    // they are inspired by the clojure atoms.
    Atom(Rc<RefCell<LispValue>>)
}

pub type AST = LispValue;

impl Display for LispValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {

        match self {
            LispValue::Int(i) => write!(f, "{}", i),
            LispValue::Float(float) => write!(f, "{}", float),
            LispValue::Error(message) => write!(f, "error - {}", message),
            LispValue::Boolean(b) => write!(f, "{}", b),
            LispValue::Nil => write!(f, "nil"),
            LispValue::Function(_l) => write!(f, "#<lambda>"),
            LispValue::String(s) => write!(f, "{}", s),
            LispValue::List(l) => write!(f, "{}", l),
            LispValue::Unit(a) => write!(f, "{}", a),
            LispValue::Atom(b) => write!(f, "Atom <{}>", b.as_ref().borrow())
        }

    }
}

// first draft is assuming we checked for parentheses issues
pub fn build_ast(parser: &mut Parser) -> LispValue {
    read_form(parser)
}

fn read_form(parser: &mut Parser) -> LispValue {
    match parser.peek().unwrap().get_text().as_str() {
        "(" => LispValue::List(read_list(parser)),
        _ => LispValue::Unit(read_atom(parser))
    }
}

fn read_list(parser: &mut Parser) -> List {
    parser.next();

    let mut l = List::new();
    // we've confirmed that there's always a matching ")"
    loop {
        match parser.peek().unwrap().get_text().as_str() {
            ")" => {
                parser.next();
                break;
            },
            _ => l.push(read_form(parser))
        };
    }

    l
}

fn read_atom(parser: &mut Parser) -> Unit {
    Unit::new(parser.next().unwrap())
}
