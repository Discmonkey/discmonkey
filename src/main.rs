use lispinrust::io::UserIO;
use lispinrust::reader::tokenizer::Tokenizer;
use lispinrust::types::ast::AST;
use lispinrust::reader::parser::Parser;
use lispinrust::env::eval::eval_ast;
use lispinrust::env::math::MathEnv;


fn main() {

    let cmd = UserIO::new();
    let tokenizer = Tokenizer::new();
    let mut maybe_line;
    let env = MathEnv::new();

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


        let (tokens, err) = tokenizer.tokenize(user_input);

        match err {
            Some(error) => {
                println!("{}", error);
                continue;
            }
            _ => ()
        }

        let mut parser = Parser::new(tokens);
        let mut ast = AST::new();

        ast.build(&mut parser);

        match ast.root() {
            Some(r)  => {
                println!("{}", eval_ast(r, &env))
            }
            _ => ()
        }


    }

}