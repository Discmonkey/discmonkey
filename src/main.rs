use lispinrust::io::UserIO;
use lispinrust::reader::tokenizer::{Tokenizer, Tokens};
use lispinrust::types::ast::{AST, build_ast};
use lispinrust::reader::parser::Parser;
use lispinrust::env::eval::eval_ast;
use lispinrust::env::state::Scope;

fn make_ast(tokens: Tokens) -> AST {
    let mut parser = Parser::new(tokens);

    build_ast(&mut parser)
}

fn main() {

    let cmd = UserIO::new();
    let tokenizer = Tokenizer::new();
    let mut maybe_line;
    let mut env = Scope::new();

    loop {
        cmd.greet();


        maybe_line = cmd.read_line();

        // received an EOF so we break out loop
        if maybe_line.is_none() {
            break; // exit successfully
        }

        let user_input = maybe_line.unwrap();

        // user just hit enter
        if user_input.len() == 0 {
            continue;
        }


        let result = tokenizer.tokenize(user_input);

        match result {
            Err(error) => println!("{}", error),

            Ok(tokens) => {
                println!("{}", eval_ast(&make_ast(tokens), &mut env));
            }
        }




    }

}