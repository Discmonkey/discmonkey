use lispinrust::io::UserIO;
use lispinrust::reader::tokenizer::Tokenizer;
use lispinrust::types::ast::AST;
use lispinrust::reader::parser::Parser;


fn main() {

    let cmd = UserIO::new();
    let tokenizer = Tokenizer::new();

    let mut maybe_line;

    loop {
        cmd.greet();

        maybe_line = cmd.read_line();

        if maybe_line.is_none() {
            break; // exit successfully
        }

        let user_input = maybe_line.unwrap();

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
        ast.print();

        println!();



    }

}