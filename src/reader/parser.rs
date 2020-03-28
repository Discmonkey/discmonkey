use super::tokenizer::Tokens;
use super::tokenizer::Token;

pub struct Parser {
    tokens: Tokens,
    index: usize
}

impl Parser {

    pub fn new(tokens: Tokens) -> Self {
        Self {tokens, index: 0}
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index);

        self.index += 1;

        token
    }
}



