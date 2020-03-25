use regex::Regex;

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

    pub fn tokenize(&self, mut line: String) -> (Option<Vec<String>>, ErrorIndex) {

        let mut v = Vec::new();
        let mut current_token = 0;

        loop {
            match self.re.captures(&line) {
                None => return (None, current_token), // could not parse starting at current token
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

                    v.push(m.as_str().to_string());
                    line = line[m.end()..].to_string();
                }
            }
        }

        (Some(v), current_token)
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


        assert_eq!(result.0.unwrap(), vec!{"(".to_string(), "+".to_string(), "4".to_string(),
                                           "4".to_string(), ")".to_string()})


    }
}