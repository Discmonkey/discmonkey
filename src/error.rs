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

    pub fn print(&self) {
        println!("{:?} error {}: {} - {}", &self.type_,
            &self.line_number, &self.character_number, &self.message
        );
    }


}