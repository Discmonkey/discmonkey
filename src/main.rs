
use lispinrust::io::UserIO;

fn main() {

    let mut cmd = UserIO::new();

    cmd.set_prefix("hi max> ".to_string());



    let mut maybe_line;

    loop {
        cmd.greet();

        maybe_line = cmd.read_line();

        match maybe_line {
            // got an EOF
            None => break,

            // anything else, note that newline is length 0
            Some(user_input) => {
                if user_input.len() > 0 {
                    cmd.write_line(&user_input);
                }
            }
        };

    }

}