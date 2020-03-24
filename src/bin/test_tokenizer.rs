use lispinrust::io::UserIO;
use lispinrust::reader::tokenizer::Tokenizer;

fn main() {

    let cmd = UserIO::new();
    let lexer = Tokenizer::new();

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


        let (option_vec, size) = lexer.tokenize(user_input);


        match option_vec {
            None => cmd.write_line(&format!("error at position {}", size).to_string()),

            Some(matches) => {
                for token in matches {
                    cmd.write(&token.as_str().to_string());
                }

                cmd.write(&("\n".to_string()));
            }
        }


    }

}