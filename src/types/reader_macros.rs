use crate::reader::parser::Parser;
use crate::types::ast::{LispValue, read_form};
use crate::reader::tokenizer::{TokenType, Token};
use crate::types::list::List;
use crate::types::unit::Unit;


fn list_with_token(token_text: &str) -> List {
    let mut l = List::new();
    let token = Token::new(token_text.to_string(), TokenType::Symbol);

    l.push(LispValue::Unit(Unit::new(token)));

    l
}

pub (super) fn at_macro(parser: &mut Parser) -> LispValue {
    parser.next();
    let mut l = list_with_token("deref");

    l.push(read_form(parser));

    LispValue::List(l)
}

pub (super) fn quote_macro(parser: &mut Parser) -> LispValue {
    parser.next();

    let mut l = list_with_token("quote");

    l.push(read_form(parser));

    LispValue::List(l)
}