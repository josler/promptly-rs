use cli::CLI;
use std::collections::HashMap;
use std::process::{Command, Stdio};

pub trait Action {
    fn run(&self, context: &HashMap<String, String>) -> Option<&str>;
}

pub struct CommandAction<'a> {
    pub action: &'a str,
    pub args: &'a [&'a str],
}

impl<'a> Action for CommandAction<'a> {
    fn run(&self, _context: &HashMap<String, String>) -> Option<&str> {
        return match Command::new(&self.action)
            .args(self.args)
            .stdout(Stdio::inherit())
            .output()
        {
            Ok(_data) => Some("yea"),
            Err(e) => {
                CLI::new().say_red(&format!("{:}", e));
                None
            }
        };
    }
}
