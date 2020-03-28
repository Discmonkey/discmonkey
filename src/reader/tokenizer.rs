use regex::Regex;
use super::error::{Error, ErrorType}; // instead of super::super we use this here
use std::fmt;


#[derive(Debug, Clone)]
pub enum TokenType {
    SpecialTwo,
    SpecialOne,
    String,
    Comment,
    Symbol
}

#[derive(Debug, Clone)]
pub struct Token {
    text: String,
    token_type: TokenType
}

impl Token {
    pub fn new(text: String, token_type: TokenType) -> Self {
        Self {
            text, token_type
        }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let pre;
        match self.token_type {
            TokenType::String => pre = "string",
            TokenType::SpecialOne => pre = "special",
            TokenType::SpecialTwo => pre = "special_two",
            TokenType::Comment => pre = "comment",
            TokenType::Symbol => pre = "symbol"
        };

        write!(f, "{}: {}", pre, self.text)

    }
}

pub type Tokens = Vec<Token>;

pub struct Tokenizer {
    re: Regex,
}

pub type ErrorIndex = usize;


impl Tokenizer {

    pub fn new() -> Self {

        let re = Regex::new(r#"(?x)
            [\s,]* #skip white spaces
            (?P<special_two>~@)
            |
            (?P<special_one>[\[\]{}()'`~^@])
            |
            (?P<string>"(?:\\.|[^\\"])*"?)
            |
            (?P<comment>;.*)
            |
            (?P<symbol>[^\s\[\]{}('"`,;)]*)"#);

        Self{re: re.unwrap()}
    }

    pub fn tokenize(&self, line:  String) -> (Tokens, Option<Error>) {

        let mut v = Vec::new();

        let mut parentheses_count = 0; // needs to end at zero, otherwise syntax error
        let mut err = None;


        for cap in self.re.captures_iter(&line) {

            if let Some(m) = cap.name("symbol") {
                v.push(Token::new(m.as_str().to_string(), TokenType::Symbol));

            } else if let Some(m) = cap.name("string") {
                let last_char = m.as_str().chars().last().unwrap();

                if last_char != '"' {
                    err = Some(Error::new(ErrorType::Syntax, "unclosed string"));
                    break;
                }

                v.push(Token::new(m.as_str().to_string(), TokenType::String));

            } else if let Some(m) = cap.name("special_one") {
                let s = m.as_str();

                if s == "(" {
                    parentheses_count += 1;
                } else if s == ")" {
                    parentheses_count -= 1;
                }

                v.push(Token::new(s.to_string(),
                                  TokenType::SpecialOne));

            } else if let Some(m) = cap.name("special_two") {
                v.push(Token::new(m.as_str().to_string(),
                                  TokenType::SpecialTwo));

            } else if let Some(m) = cap.name("comment") {
                v.push(Token::new(m.as_str().to_string(),
                                  TokenType::Comment));
            }
        }

        if err.is_none() && parentheses_count != 0 {
            err = Some(Error::new(ErrorType::Syntax, "mismatched parentheses"));
        }

        (v, err)

    }

}

#[cfg(test)]
mod test {

    use super::Tokenizer;

    #[test]
    fn tokenize() {

        let r = Tokenizer::new();
        let line = "(+ 4 4)".to_string();

        let (result, err) = r.tokenize(line);

        match err {
            None => assert!(true),
            _ => assert!(false)
        }

        assert_eq!(result[0].get_text(), "(");
        assert_eq!(result[4].get_text(), ")");


    }

    #[test]
    fn bad_input() {
        let r = Tokenizer::new();
        let line = "(+ 4 4 (+ 4)".to_string();

        let (_, err) = r.tokenize(line);

        match err {
            None => assert!(false),
            _ => assert!(true)
        }
    }
}