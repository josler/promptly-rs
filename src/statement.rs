use action::{Action, CommandAction};
use question::Question;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub fn while_success(statements: Vec<&Statement>) {
    let mut context = HashMap::new();
    for statement in statements {
        if let Some(_) = statement.question.ask(&mut context) {
            if let Some(a) = statement.action {
                a.run(&context);
            }
        }
    }
}

pub struct Statement<'a> {
    pub question: &'a Question,
    pub action: Option<&'a Action>,
}
