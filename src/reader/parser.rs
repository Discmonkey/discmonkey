use super::tokenizer::Tokens;
use super::tokenizer::Token;

pub struct Parser {
    tokens: Tokens,
}

impl Parser {

    pub fn new(tokens: Tokens) -> Self {
        Self {tokens}
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
}



