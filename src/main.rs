use lispinrust::io::UserIO;
use lispinrust::reader::tokenizer::{Tokenizer, Tokens};
use lispinrust::types::ast::AST;
use lispinrust::reader::parser::Parser;
use lispinrust::env::eval::eval_ast;
use lispinrust::env::math::Env;

fn make_ast(tokens: Tokens) -> AST {
    let mut parser = Parser::new(tokens);
    let mut ast = AST::new();

    ast.build(&mut parser);

    ast
}

fn main() {

    let cmd = UserIO::new();
    let tokenizer = Tokenizer::new();
    let mut maybe_line;
    let mut env = Env::new();

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

                if let Some(root) = make_ast(tokens).root() {
                    println!("{}", eval_ast(root, &mut env))
                }

            }
        }




    }

}