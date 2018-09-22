use action::{Action, CommandAction};
use failure::{err_msg, Error};
use question::Question;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub fn while_success<'a>(statements: Vec<&Statement>) -> Result<&'a str, Error> {
    let mut context = HashMap::new();
    for statement in statements {
        match statement.question.ask(&mut context) {
            Some(_) => {
                if let Some(a) = statement.action {
                    a.run(&context)?;
                }
            }
            None => return Err(format_err!("Question failed")),
        }
    }
    Ok("Yes")
}

pub struct Statement<'a> {
    pub question: &'a Question,
    pub action: Option<&'a Action>,
}
