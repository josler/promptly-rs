use std::fmt;
use std::fmt::{Debug, Formatter};

pub fn while_success(statements: Vec<&Statement>) {
    println!("{:?}", statements);
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
        None
    }
}
