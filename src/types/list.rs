use super::ast::{LispValue};
use crate::reader::tokenizer::Token;
use std::ops::Index;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct List {
    items: Vec<LispValue>
}

/// List is a shallow wrapper around a Vec<LispValue>, it adds a few convenience methods
impl List {
    pub fn new() -> Self {
        List {
            items: Vec::new()
        }
    }

    pub fn from_vec(items: Vec<LispValue>) -> Self {
        List {
            items
        }
    }

    pub fn first_token(&self) -> Option<&Token> {
        match self.items.first() {
            Some(LispValue::Atom(a)) => {
                Some(a.token())
            }
            _  => None,

        }
    }

    pub fn items(&self) -> &Vec<LispValue> {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, val: LispValue) {
        self.items.push(val)
    }
}

impl Index<usize> for List {
    type Output = LispValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(")?;

        for item in &self.items {
            write!(f, "{}, ", item)?;
        }

        write!(f, ")")
    }
}
