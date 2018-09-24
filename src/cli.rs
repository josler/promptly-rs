use std::io::{stdin, stdout, BufRead, Result, Write, Error, ErrorKind};
use std::process::{Command, Stdio};

pub struct CLI {}

impl CLI {
    pub fn new() -> CLI {
        CLI {}
    }

    pub fn question<'a>(&self, question: &str, default: &'a str) -> Result<String> {
        self.write_out(question, default);

        let mut input = String::new();
        if let Err(err) = self.read_in(&mut input) {
            return Err(err);
        }
        Ok(self.trim_lower(input, default))
    }

    pub fn yes_no_question(&self, question: &str, default: &str) -> Result<bool> {
        let res = self.question(question, default);
        return match res {
            Err(err) => Err(err),
            Ok(val) => {
                let yes_values: [String; 3] = ["yes".to_string(), "y".into(), "1".into()];
                if yes_values.contains(&val) {
                    Ok(true)
                } else {
                    Err(Error::new(ErrorKind::Other, "unsuccessful command"))
                }
            }
        };
    }

    pub fn run_command(&self, action: &str, args: &[&str]) -> Result<String> {
        let output = Command::new(&action)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;
        if !output.status.success() {
            return Err(Error::new(ErrorKind::Other, "unsuccessful command"));
        }
        let str_out = &String::from_utf8_lossy(&output.stdout);
        Ok(str_out.to_string())
    }

    fn trim_lower<'a>(&self, input: String, default: &'a str) -> String {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return default.to_string();
        }
        let lower = trimmed.to_lowercase();
        lower
    }

    fn write_out(&self, text: &str, default: &str) {
        let stdout = stdout();
        let mut stdout = stdout.lock();

        let formatted = match default.is_empty() {
            true => format!("{}\n", text),
            false => format!("{} |{}|\n", text, default),
        };

        stdout.write_all(formatted.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }

    fn read_in(&self, input: &mut String) -> Result<usize> {
        let stdin = stdin();
        let mut stdin = stdin.lock();

        stdin.read_line(input)
    }
}
