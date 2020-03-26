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

    pub fn new(type_: ErrorType, line_number: usize, character_number: usize, message: &str ) -> Self {
        Error {
            type_,
            line_number,
            character_number,
            message: message.to_string()
        }
    }


    pub fn print(&self) {
        println!("{:?} error {}: {} - {}", &self.type_,
            &self.line_number, &self.character_number, &self.message
        );
    }


}