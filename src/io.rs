use std::io;
use std::io::Write;

pub struct UserIO {
    prefix: String
}

impl UserIO {

    pub fn new() -> Self {
        UserIO {prefix: "user> ".to_string()}
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
    }

    pub fn read_line() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Couldn't read line");

        input
    }

    pub fn write_line(self, line: String) {

        let mut with_prefix = self.prefix.clone();

        with_prefix.push_str(&line);

        io::stdout().write(with_prefix.as_bytes()).ok().expect("could not print line");
    }



}


#[cfg(test)]
mod test {

}