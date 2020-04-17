
use crate::reader::tokenizer::Token;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Unit {
    token: Token
}

impl Unit {
    pub fn new(token: Token) -> Unit {
        Unit{token}
    }

    pub fn print(&self) {
        print!("{}", self.token);
    }

    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.token)
    }
}
