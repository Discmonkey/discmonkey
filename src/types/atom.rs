
use crate::reader::tokenizer::Token;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Atom {
    token: Token
}

impl Atom {
    pub fn new(token: Token) -> Atom {
        Atom{token}
    }

    pub fn print(&self) {
        print!("{}", self.token);
    }

    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.token)
    }
}
