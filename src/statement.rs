use action::{Action, CommandAction};
use question::Question;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub fn while_success(statements: Vec<&Statement>) {
    let mut context = HashMap::new();
    for statement in statements {
        match statement.question.ask(&mut context) {
            Some(_) => {
                match statement.action.run(&context) {
                    Some(_) => {}
                    None => {}
                };
            }
            None => return,
        };
    }
}

pub struct Statement<'a> {
    pub question: &'a Question,
    pub action: &'a CommandAction<'a>,
}

impl<'a> Debug for Statement<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.action.action)
    }
}
