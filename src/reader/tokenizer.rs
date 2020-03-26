use regex::Regex;
use super::error::{Error, ErrorType}; // instead of super::super we use this here

pub struct Tokenizer {
    re: Regex,
}

pub type ErrorIndex = usize;
pub type Tokens = Vec<String>;

impl Tokenizer {

    pub fn new() -> Self {
        let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#);

        Self{re: re.unwrap()}
    }

    pub fn tokenize(&self, mut line: String) -> (Vec<String>, Option<Error>) {

        let mut v = Vec::new();
        let mut current_token: usize = 0;

        let line_number= 0;
        let mut parentheses_count = 0; // needs to end at zero, otherwise syntax error


        loop {
            match self.re.captures(&line) {

                None => {
                    let err = Error::new(ErrorType::Syntax,
                                         line_number,
                                         current_token,
                                         "could not parse token");

                    return (v, Some(err))
                }

                Some(captures) => {

                    if captures.len() <= 1 {
                        break;
                    } else {
                        // first capture group is the full match
                        let m = captures.get(0).unwrap();
                        current_token += m.end() - m.start();
                    }

                    let m = captures.get(1).unwrap();

                    if m.start() == m.end() {
                        break;
                    }

                    let token = m.as_str();

                    match token {
                        "(" => parentheses_count += 1,
                        ")" => parentheses_count -= 1,
                        _ => ()
                    }

                    v.push(token.to_string());
                    line = line[m.end()..].to_string();
                }
            }
        }

        if parentheses_count == 0 {
            (v, None)
        } else {
            (v, Some(Error::new(ErrorType::Semantic, 0, 0,
                                 "opening/closing parentheses mismatch")))
        }
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

        assert_eq!(result, vec!{"(".to_string(), "+".to_string(), "4".to_string(),
                                           "4".to_string(), ")".to_string()})


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