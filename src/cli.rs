use std::io::{stdin, stdout, BufRead, Result, Write};
// use termion::input::TermRead;

pub struct CLI {}

impl CLI {
    pub fn new() -> CLI {
        CLI {}
    }

    pub fn question(&self, question: &str, default: &str) -> Result<bool> {
        self.write_out(question, default);

        let mut input = String::new();
        if let Err(err) = self.read_in(&mut input) {
            return Err(err);
        }

        let lower = self.get_clean_string(&input, default);
        let yes_values: [String; 3] = ["yes".to_string(), "y".into(), "1".into()];
        if yes_values.contains(&lower) {
            return Ok(true);
        }
        Ok(false)
    }

    // needs to take a closure too
    // ask a question and ...
    pub fn ask_for(&self, question: &String) {}

    pub fn say_red(&self, question: &String) {}

    pub fn say_green(&self, question: &String) {
        println!("{}", question);
    }

    fn write_out(&self, text: &str, default: &str) {
        let stdout = stdout();
        let mut stdout = stdout.lock();

        let formatted = format!("{} |{}|\n", text, default);

        stdout.write_all(formatted.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }

    fn read_in(&self, input: &mut String) -> Result<usize> {
        let stdin = stdin();
        let mut stdin = stdin.lock();

        stdin.read_line(input)
    }

    fn get_clean_string(&self, input: &String, default: &str) -> String {
        let mut lower = input.trim().to_lowercase();
        if lower.is_empty() {
            lower = default.to_string();
        }
        lower
    }
}
