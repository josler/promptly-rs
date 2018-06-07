use cli;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub fn while_success(statements: Vec<&Statement>) {
    statements
        .iter()
        .take_while(|s| match s.question.ask() {
            Some(_) => true,
            None => false,
        })
        .for_each(|s| println!("{}", s.action));
}

pub trait Question {
    fn ask(&self) -> Option<&str>;
}

pub struct Statement<'a> {
    pub question: &'a Question,
    pub action: String,
}

impl<'a> Debug for Statement<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.action)
    }
}

pub struct YesQuestion {}

impl Question for YesQuestion {
    fn ask(&self) -> Option<&str> {
        Some("yes")
    }
}

pub struct NoQuestion {}

impl Question for NoQuestion {
    fn ask(&self) -> Option<&str> {
        None
    }
}

pub struct AskQuestion {}

impl Question for AskQuestion {
    fn ask(&self) -> Option<&str> {
        let c = cli::CLI::new();
        return match c.question("do you agree?", "yes") {
            Ok(true) => Some("ok"),
            Ok(false) => None,
            Err(_) => None,
        };
    }
}
