use lispinrust::io::UserIO;
use lispinrust::reader::tokenizer::{Tokenizer};
use lispinrust::exec::eval::eval_ast;
use lispinrust::types::env::Scope;
use lispinrust::exec::core_utils::read_string;

/// pre_load is for functions we want the user to have,
/// but also defined within lisp and not on the core, interpreter level
fn pre_load(tokenizer: &Tokenizer, mut env: &mut Scope) {
    let read_file = read_string(&tokenizer, "(def! load-file (lambda (f) (eval (read-string (str \"(do \" (slurp f) \"\nnil)\")))))".to_string());
    eval_ast(&read_file, &mut env);
}

fn main() {

    let cmd = UserIO::new();
    let tokenizer = Tokenizer::new();
    let mut maybe_line;
    let mut env = Scope::new();

    // functions defined with mal
    pre_load(&tokenizer, &mut env);

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

        let result = read_string(&tokenizer, user_input);
        println!("{}", eval_ast(&result, &mut env))

    }

}