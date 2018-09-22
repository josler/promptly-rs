use cli;
use std::cell::RefCell;
use std::collections::HashMap;

pub trait Question {
    fn ask(&self, context: &mut HashMap<String, String>) -> Option<&str>;
}

pub struct AlwaysYesQuestion {}

impl Question for AlwaysYesQuestion {
    fn ask(&self, _context: &mut HashMap<String, String>) -> Option<&str> {
        Some("Yes")
    }
}

pub struct AlwaysNoQuestion {}

impl Question for AlwaysNoQuestion {
    fn ask(&self, _context: &mut HashMap<String, String>) -> Option<&str> {
        None
    }
}

pub struct AskQuestion<'a> {
    pub ask: &'a str,
    pub default: &'a str,
}

impl<'a> Question for AskQuestion<'a> {
    fn ask(&self, context: &mut HashMap<String, String>) -> Option<&str> {
        let c = cli::CLI::new();
        return match c.yes_no_question(self.ask, self.default) {
            Ok(true) => {
                context.insert("foo".to_string(), "bar".to_string());
                Some("yes")
            }
            Ok(false) => None,
            Err(_) => None,
        };
    }
}

pub struct InfoQuestion<'a> {
    pub ask: &'a str,
    pub default: &'a str,
}

impl<'a> Question for InfoQuestion<'a> {
    fn ask(&self, context: &mut HashMap<String, String>) -> Option<&str> {
        let c = cli::CLI::new();
        return match c.question(self.ask, self.default) {
            Ok(result) => {
                context.insert(self.ask.to_string(), result);
                Some("Yes")
            }
            Err(_) => None,
        };
    }
}

pub struct BranchQuestion<'a> {
    pub ask: &'a str,
}

impl<'a> Question for BranchQuestion<'a> {
    fn ask(&self, context: &mut HashMap<String, String>) -> Option<&str> {
        let c = cli::CLI::new();
        let default = self.get_default(context);
        context.insert(
            self.ask.to_string(),
            c.question(self.ask, &default).unwrap(),
        );
        Some("Yes")
    }
}

impl<'a> BranchQuestion<'a> {
    fn get_default(&self, context: &'a mut HashMap<String, String>) -> String {
        let default = context
            .entry("PR Description?".to_string())
            .or_insert("ayyy".to_string());
        default.to_string()
    }
}
