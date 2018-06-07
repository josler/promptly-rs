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
        .for_each(|s| println!("{}", s.name));
}

pub trait Question {
    fn ask(&self) -> Option<&str>;
}

pub struct Statement<'a> {
    pub name: String,
    pub question: &'a Question,
}

impl<'a> Debug for Statement<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

pub struct YesQuestion {}

impl Question for YesQuestion {
    fn ask(&self) -> Option<&str> {
        Some("some result")
    }
}

pub struct NoQuestion {}

impl Question for NoQuestion {
    fn ask(&self) -> Option<&str> {
        None
    }
}

pub struct YesNoQuestion {}

impl Question for YesNoQuestion {
    fn ask(&self) -> Option<&str> {
        let c = cli::CLI::new();
        return match c.agree() {
            Ok(true) => Some("ok"),
            Ok(false) => None,
            Err(_) => None,
        };
    }
}
