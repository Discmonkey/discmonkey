use super::reader::parser::Parser;
use super::atom::Atom;
use super::list::List;

pub enum LispValue {
    List(List),
    Atom(Atom)
}

pub type AST = LispValue;

// first draft is assuming we checked for parentheses issues
pub fn build_ast(parser: &mut Parser) -> LispValue {
    read_form(parser)
}

fn read_form(parser: &mut Parser) -> LispValue {
    match parser.peek().unwrap().get_text().as_str() {
        "(" => LispValue::List(read_list(parser)),
        _ => LispValue::Atom(read_atom(parser))
    }
}

fn read_list(parser: &mut Parser) -> List {
    parser.next();

    let mut l = List::new();
    // we've confirmed that there's always a matching ")"
    loop {
        match parser.peek().unwrap().get_text().as_str() {
            ")" => {
                parser.next();
                break;
            },
            _ => l.push(read_form(parser))
        };
    }

    l
}

fn read_atom(parser: &mut Parser) -> Atom {
    Atom::new(parser.next().unwrap())
}
