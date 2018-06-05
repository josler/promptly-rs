use std::io::{stdin, stdout, BufRead, Result, Write};
// use termion::input::TermRead;

pub struct CLI {}

impl CLI {
    pub fn new() -> CLI {
        CLI {}
    }

    // like asking a question, but the question is "do you agree |yes|"
    pub fn agree(&self, question: &str) {
        self.write_out(question);

        let mut input = String::new();
        match self.read_in(&mut input) {
            Ok(_) => println!("{}", input),
            Err(error) => println!("error: {}", error),
        }
    }

    // needs to take a closure too
    // ask a question and ...
    pub fn ask_for(&self, query: &str) {}

    pub fn say_red(&self, text: &str) {}

    pub fn say_green(&self, text: &str) {
        println!("{}", text);
    }

    fn write_out(&self, text: &str) {
        let stdout = stdout();
        let mut stdout = stdout.lock();

        &stdout.write_all(text.as_bytes()).unwrap();
    }

    fn read_in(&self, input: &mut String) -> Result<usize> {
        let stdin = stdin();
        let mut stdin = stdin.lock();

        stdin.read_line(input)
    }
}
