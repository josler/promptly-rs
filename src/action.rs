use cli::CLI;
use std::cell::RefCell;
use std::collections::HashMap;
use std::process::{Command, Stdio};

use failure::{err_msg, Error};

pub trait Action {
    fn run(&self, context: &HashMap<String, String>) -> Result<&str, Error>;
}

pub struct CommandAction<'a> {
    pub action: &'a str,
    pub args: &'a [&'a str],
}

impl<'a> Action for CommandAction<'a> {
    fn run(&self, _context: &HashMap<String, String>) -> Result<&str, Error> {
        let output = Command::new(&self.action)
            .args(self.args)
            .stdout(Stdio::inherit())
            .output()?;
        Ok("yes")
        // Ok(&String::from_utf8_lossy(&output.stdout))
    }
}
