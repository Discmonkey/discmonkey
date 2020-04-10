
use crate::reader::tokenizer::Token;

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
