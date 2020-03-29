use std::fmt::{Display, Result, Formatter};

#[derive(Debug)]
pub enum ErrorType {
    Semantic,
    Syntax,
    NotSet
}

pub struct Error {
    type_: ErrorType,
    line_number: usize,
    character_number: usize,
    message: String
}

impl Error {

    pub fn new(type_: ErrorType, message: &str ) -> Self {
        Error {
            type_,
            line_number: 0,
            character_number: 0,
            message: message.to_string()
        }
    }

    pub fn set_error_index(&mut self, at: usize) {
        self.character_number = at;
    }

    pub fn set_error_line(&mut self, at: usize) {
        self.line_number = at;
    }

}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} error - {}", self.type_, self.message)
    }
}