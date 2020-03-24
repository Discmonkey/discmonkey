use super::tokenizer::Tokens;

pub struct Parser {
    tokens: Tokens,
    index: usize
}

enum ReaderState {
    ReadingForm,
    ReadingAtom
}

pub struct Reader {
    state: ReaderState,
    parser: Parser
}




impl Parser {

    pub fn new(tokens: Tokens) -> Self {
        Self {tokens, index: 0}
    }

    pub fn peek(&self) -> Option<&String> {
        self.tokens.get(self.index)
    }

    pub fn next(&mut self) -> Option<&String> {
        let token = self.tokens.get(self.index);

        self.index += 1;

        token
    }
}



#[cfg(test)]
mod test {

    use super::Tokenizer;

    #[test]
    fn tokenize() {

        let r = Tokenizer::new();
        let line = "(+ 4 4)".to_string();

        let result = r.tokenize(line);

//        for &n in result {
//            println!("{}", *n);
//        }

        assert_eq!(result.0.unwrap(), vec!{"(".to_string(), "+".to_string(), "4".to_string(),
                                           "4".to_string(), ")".to_string()})


    }
}